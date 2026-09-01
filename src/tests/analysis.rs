use std::fs;

use tempfile::TempDir;

use crate::analysis::{AnalysisDiagnosticKind, AnalyzedFile, analyze};
use crate::ingest::discover;
use crate::language::LanguageRegistry;

use super::configuration::load_config;
type TestResult<T = ()> = anyhow::Result<T>;

#[test]
fn formatting_comments_identifiers_and_literals_normalize_as_required() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("first.rs"),
        "fn calculate(value: i32) -> i32 {\n    // ignored\n    value + 1\n}\n",
    )?;
    fs::write(
        fixture.path().join("formatted.rs"),
        "fn calculate ( value : i32 )->i32{value+1}\n",
    )?;
    fs::write(
        fixture.path().join("renamed.rs"),
        "fn transform(input: i32) -> i32 { input + 99 }\n",
    )?;
    fs::write(
        fixture.path().join("changed.rs"),
        "fn transform(input: i32) -> i32 { input * 99 }\n",
    )?;

    let files = analyze_fixture(&fixture)?;
    let first = block(&files, "first.rs")?;
    let formatted = block(&files, "formatted.rs")?;
    let renamed = block(&files, "renamed.rs")?;
    let changed = block(&files, "changed.rs")?;

    assert_eq!(first.exact, formatted.exact);
    assert_eq!(first.normalized, renamed.normalized);
    assert_ne!(first.exact, renamed.exact);
    assert_ne!(renamed.normalized, changed.normalized);
    Ok(())
}

#[test]
fn providers_extract_blocks_and_count_complexity_decisions() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("logic.rs"),
        "fn logic(a: bool, b: bool) {\n if a && b { loop { break; } }\n}\n",
    )?;
    fs::write(
        fixture.path().join("logic.py"),
        "def logic(a, b):\n    if a and b:\n        return 1\n    return 0\n",
    )?;
    fs::write(
        fixture.path().join("logic.ts"),
        "function logic(a: boolean, b: boolean) { return a && b ? 1 : 0; }\n",
    )?;
    fs::write(
        fixture.path().join("view.tsx"),
        "const View = (ok: boolean) => ok ? <p>yes</p> : <p>no</p>;\n",
    )?;

    let files = analyze_fixture(&fixture)?;
    for path in ["logic.rs", "logic.py", "logic.ts", "view.tsx"] {
        let analyzed = block(&files, path)?;
        assert!(analyzed.named_node_count > 0, "{path}");
        assert!(analyzed.line_count > 0, "{path}");
        assert!(analyzed.complexity >= 2, "{path}");
        assert!(!analyzed.shingles.is_empty(), "{path}");
        assert!(
            analyzed.shingles.windows(2).all(|pair| pair[0] < pair[1]),
            "{path}"
        );
    }
    assert_eq!(block(&files, "logic.rs")?.complexity, 4);
    assert_eq!(block(&files, "logic.py")?.complexity, 3);
    assert_eq!(block(&files, "logic.ts")?.complexity, 3);
    Ok(())
}

#[test]
fn python_extracts_classes_decorated_async_functions_methods_and_lambdas() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("forms.py"),
        concat!(
            "@trace(\"network\")\n",
            "async def fetch(value):\n",
            "    return value\n",
            "\n",
            "class Service:\n",
            "    def run(self, value):\n",
            "        return value\n",
            "\n",
            "factory = lambda item: item + 1\n",
        ),
    )?;

    let files = analyze_fixture(&fixture)?;
    let analyzed = files
        .iter()
        .find(|file| file.identity.path.to_string_lossy() == "forms.py")
        .ok_or_else(|| anyhow::anyhow!("forms.py was not analyzed"))?;
    let start_lines: Vec<_> = analyzed
        .blocks
        .iter()
        .map(|block| block.location.start_line)
        .collect();

    assert_eq!(start_lines, [1, 5, 6, 9]);
    Ok(())
}

#[test]
fn partial_and_invalid_utf8_files_do_not_abort_other_analysis() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("partial.py"),
        "def good():\n    return 1\n\ndef broken(:\n    return 2\n",
    )?;
    fs::write(
        fixture.path().join("bytes.rs"),
        b"fn bytes() { let value = \"\xff\"; }\n",
    )?;

    let files = analyze_fixture(&fixture)?;
    let partial = files
        .iter()
        .find(|file| file.identity.path.to_string_lossy() == "partial.py")
        .ok_or_else(|| anyhow::anyhow!("partial.py was not analyzed"))?;

    assert!(
        partial
            .blocks
            .iter()
            .any(|block| block.location.start_line == 1)
    );
    assert_eq!(partial.diagnostics.len(), 1);
    assert_eq!(
        partial.diagnostics[0].kind,
        AnalysisDiagnosticKind::PartialParse
    );
    assert_eq!(files.len(), 2);
    Ok(())
}

#[test]
fn repeated_analysis_is_identical() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("stable.py"),
        "def stable(value):\n    return value if value else 1\n",
    )?;

    assert_eq!(analyze_fixture(&fixture)?, analyze_fixture(&fixture)?);
    Ok(())
}

pub(super) fn analyze_fixture(fixture: &TempDir) -> TestResult<Vec<AnalyzedFile>> {
    let config = load_config("[core]\nexclude = []")?;
    let registry = LanguageRegistry::compile()?;
    let discovery = discover(fixture.path(), &config, &registry)?;
    Ok(analyze(discovery.files, &registry)?)
}

fn block<'a>(
    files: &'a [AnalyzedFile],
    path: &str,
) -> TestResult<&'a crate::analysis::AnalyzedBlock> {
    files
        .iter()
        .find(|file| file.identity.path.to_string_lossy() == path)
        .and_then(|file| file.blocks.first())
        .ok_or_else(|| anyhow::anyhow!("no analyzed block for {path}"))
}
