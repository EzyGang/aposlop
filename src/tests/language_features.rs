use std::fs;

use tempfile::TempDir;

use crate::analysis::{AnalyzedBlock, AnalyzedFile};

use super::analysis::analyze_fixture;
type TestResult<T = ()> = anyhow::Result<T>;

#[test]
fn go_covers_blocks_type_2_normalization_and_cyclomatic_decisions() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("control.go"),
        concat!(
            "package sample\n",
            "\n",
            "func Control[T comparable](value T, items []T, ch <-chan T, ready bool) bool {\n",
            "    if ready {\n",
            "    } else if value == items[0] {\n",
            "    }\n",
            "    for range 3 {}\n",
            "    for _, item := range items { _ = item }\n",
            "    switch value {\n",
            "    case items[0]:\n",
            "    case items[1], items[2]:\n",
            "    default:\n",
            "    }\n",
            "    switch typed := any(value).(type) {\n",
            "    case T: _ = typed\n",
            "    default:\n",
            "    }\n",
            "    select {\n",
            "    case <-ch:\n",
            "    default:\n",
            "    }\n",
            "    nested := func(flag bool) bool {\n",
            "        if flag && ready { return true }\n",
            "        return false\n",
            "    }\n",
            "    _ = nested\n",
            "    return ready && len(items) > 0 || value == items[0]\n",
            "}\n",
        ),
    )?;
    fs::write(
        fixture.path().join("forms.go"),
        concat!(
            "package sample\n",
            "\n",
            "type Box[T any] struct { value T }\n",
            "func Free() {}\n",
            "func (box *Box[T]) Method() T { return box.value }\n",
            "var Literal = func(value int) int { return value }\n",
        ),
    )?;
    fs::write(
        fixture.path().join("first.go"),
        concat!(
            "package sample\n",
            "\n",
            "func Collect(value firstpkg.First) []any {\n",
            "again:\n",
            "    // first comment\n",
            "    output := []any{1, 1.5, 2i, 'a', `raw`, \"text\", true, false, nil}\n",
            "    if value.Left != 3 { goto again }\n",
            "    return output\n",
            "}\n",
        ),
    )?;
    fs::write(
        fixture.path().join("second.go"),
        concat!(
            "package sample\n",
            "\n",
            "func Gather(other secondpkg.Second) []any {\n",
            "retry:\n",
            "    /* second comment */\n",
            "    result := []any{99, 9.5, 7i, 'z', `other`, \"changed\", true, false, nil}\n",
            "    if other.Right != 8 { goto retry }\n",
            "    return result\n",
            "}\n",
        ),
    )?;

    let files = analyze_fixture(&fixture)?;
    let control = file(&files, "control.go")?;
    assert!(control.diagnostics.is_empty());
    assert_eq!(
        control
            .blocks
            .iter()
            .map(|block| block.complexity)
            .collect::<Vec<_>>(),
        [11, 3]
    );
    assert_eq!(file(&files, "forms.go")?.blocks.len(), 3);

    let first = only_block(&files, "first.go")?;
    let second = only_block(&files, "second.go")?;
    assert_ne!(first.exact, second.exact);
    assert_eq!(first.normalized, second.normalized);
    Ok(())
}

#[test]
fn python_314_syntax_and_decisions_parse_without_diagnostics() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("modern.py"),
        concat!(
            "type Pair[T] = tuple[T, T]\n",
            "\n",
            "def modern[T](items: list[T], value: int):\n",
            "    assert items\n",
            "    values = [item for item in items if item]\n",
            "    for item in values:\n",
            "        pass\n",
            "    while value:\n",
            "        break\n",
            "    if value > 1:\n",
            "        pass\n",
            "    elif value < 0:\n",
            "        pass\n",
            "    match value:\n",
            "        case item if item:\n",
            "            pass\n",
            "        case _:\n",
            "            pass\n",
            "    try:\n",
            "        return values if value and items else []\n",
            "    except OSError, RuntimeError:\n",
            "        return ...\n",
        ),
    )?;
    fs::write(
        fixture.path().join("nested.py"),
        concat!(
            "def outer():\n",
            "    def inner(value):\n",
            "        if value:\n",
            "            return 1\n",
            "        return 0\n",
            "    return inner\n",
        ),
    )?;

    let files = analyze_fixture(&fixture)?;
    let modern = file(&files, "modern.py")?;
    assert!(modern.diagnostics.is_empty());
    assert_eq!(modern.blocks.len(), 1);
    assert_eq!(modern.blocks[0].complexity, 14);

    let nested = file(&files, "nested.py")?;
    assert!(nested.diagnostics.is_empty());
    assert_eq!(block_at(nested, 1)?.complexity, 1);
    assert_eq!(block_at(nested, 2)?.complexity, 2);
    Ok(())
}

#[test]
fn rust_covers_closures_modern_control_flow_and_identifier_kinds() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("logic.rs"),
        concat!(
            "fn outer(result: Result<i32, ()>, items: &[i32]) -> Result<i32, ()> {\n",
            "    let closure = |flag: bool| { if flag && flag { 1 } else { 0 } };\n",
            "    for _item in items {\n",
            "        while false {\n",
            "            loop { break; }\n",
            "        }\n",
            "    }\n",
            "    if let Ok(value) = result && value > 0 {}\n",
            "    match 2 {\n",
            "        value if value > 1 && value < 3 => {}\n",
            "        _ => {}\n",
            "    }\n",
            "    let value = Ok::<i32, ()>(1)?;\n",
            "    Ok(value + closure(true))\n",
            "}\n",
        ),
    )?;
    fs::write(
        fixture.path().join("first.rs"),
        "fn extract(item: First) -> i32 { item.left + 1 }\n",
    )?;
    fs::write(
        fixture.path().join("second.rs"),
        "fn extract(other: Second) -> i32 { other.right + 99 }\n",
    )?;

    let files = analyze_fixture(&fixture)?;
    let logic = file(&files, "logic.rs")?;
    assert!(logic.diagnostics.is_empty());
    assert_eq!(logic.blocks.len(), 2);
    assert_eq!(block_at(logic, 1)?.complexity, 10);
    assert_eq!(block_at(logic, 2)?.complexity, 3);

    let first = block_at(file(&files, "first.rs")?, 1)?;
    let second = block_at(file(&files, "second.rs")?, 1)?;
    assert_ne!(first.exact, second.exact);
    assert_eq!(first.normalized, second.normalized);
    Ok(())
}

#[test]
fn typescript_covers_current_branching_and_implicit_blocks() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("default.ts"),
        "function defaulted(value = 1) { return value; }\n",
    )?;
    fs::write(
        fixture.path().join("destructure.ts"),
        "function destructured({ value = 1 }: { value?: number }) { return value; }\n",
    )?;
    fs::write(
        fixture.path().join("optional.ts"),
        "function optional(value: any, fallback: any) { return value?.first?.second ?? fallback; }\n",
    )?;
    fs::write(
        fixture.path().join("assign.ts"),
        "function assign(value: any) { value ||= 1; value &&= 2; value ??= 3; }\n",
    )?;
    fs::write(
        fixture.path().join("class.ts"),
        concat!(
            "class Example {\n",
            "    field = source?.value ?? fallback;\n",
            "    static { if (ready) {} }\n",
            "    method() { const nested = () => condition ? 1 : 0; return nested; }\n",
            "}\n",
        ),
    )?;
    fs::write(
        fixture.path().join("control.ts"),
        concat!(
            "function control(value: number, object: object, items: number[]) {\n",
            "    if (value) {}\n",
            "    for (let index = 0; index < value; index++) {}\n",
            "    for (const key in object) {}\n",
            "    for (const item of items) {}\n",
            "    while (value) { break; }\n",
            "    do {} while (value);\n",
            "    switch (value) { case 1: break; case 2: break; default: break; }\n",
            "    try {} catch (error) {}\n",
            "    return ((value && object) || (items ?? [])) ? 1 : 0;\n",
            "}\n",
        ),
    )?;
    fs::write(
        fixture.path().join("forms.ts"),
        concat!(
            "function declared() {}\n",
            "function* declaredGenerator() {}\n",
            "const expression = function named() {};\n",
            "const generator = function* namedGenerator() {};\n",
            "const arrow = () => 1;\n",
            "class Forms { field = 1; static {} method() {} }\n",
        ),
    )?;

    let files = analyze_fixture(&fixture)?;
    assert_eq!(only_block(&files, "default.ts")?.complexity, 2);
    assert_eq!(only_block(&files, "destructure.ts")?.complexity, 2);
    assert_eq!(only_block(&files, "optional.ts")?.complexity, 4);
    assert_eq!(only_block(&files, "assign.ts")?.complexity, 4);
    assert_eq!(only_block(&files, "control.ts")?.complexity, 14);
    assert_eq!(file(&files, "forms.ts")?.blocks.len(), 8);

    let class = file(&files, "class.ts")?;
    let blocks: Vec<_> = class
        .blocks
        .iter()
        .map(|block| (block.location.start_line, block.complexity))
        .collect();
    assert_eq!(blocks, [(2, 3), (3, 2), (4, 1), (4, 2)]);
    Ok(())
}

#[test]
fn typescript_and_tsx_normalize_all_identifier_and_literal_forms() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("first.ts"),
        concat!(
            "class Box {\n",
            "    #left: First;\n",
            "    read(input: First) { return this.#left + input.value + 1; }\n",
            "}\n",
        ),
    )?;
    fs::write(
        fixture.path().join("second.ts"),
        concat!(
            "class Box {\n",
            "    #right: Second;\n",
            "    read(other: Second) { return this.#right + other.result + 99; }\n",
            "}\n",
        ),
    )?;
    fs::write(
        fixture.path().join("first.tsx"),
        "const View = () => <Panel title=\"first\">Hello &amp;</Panel>;\n",
    )?;
    fs::write(
        fixture.path().join("second.tsx"),
        "const View = () => <Panel title=\"second\">Goodbye &copy;</Panel>;\n",
    )?;

    let files = analyze_fixture(&fixture)?;
    let first = only_block(&files, "first.ts")?;
    let second = only_block(&files, "second.ts")?;
    assert_ne!(first.exact, second.exact);
    assert_eq!(first.normalized, second.normalized);

    let first_tsx = only_block(&files, "first.tsx")?;
    let second_tsx = only_block(&files, "second.tsx")?;
    assert_ne!(first_tsx.exact, second_tsx.exact);
    assert_eq!(first_tsx.normalized, second_tsx.normalized);
    Ok(())
}

fn only_block<'a>(files: &'a [AnalyzedFile], path: &str) -> TestResult<&'a AnalyzedBlock> {
    let analyzed = file(files, path)?;
    match analyzed.blocks.as_slice() {
        [block] => Ok(block),
        blocks => anyhow::bail!("expected one block in {path}, found {}", blocks.len()),
    }
}

fn block_at(file: &AnalyzedFile, start_line: usize) -> TestResult<&AnalyzedBlock> {
    file.blocks
        .iter()
        .find(|block| block.location.start_line == start_line)
        .ok_or_else(|| anyhow::anyhow!("no block starts at line {start_line}"))
}

fn file<'a>(files: &'a [AnalyzedFile], path: &str) -> TestResult<&'a AnalyzedFile> {
    files
        .iter()
        .find(|file| file.identity.path.to_string_lossy() == path)
        .ok_or_else(|| anyhow::anyhow!("{path} was not analyzed"))
}
