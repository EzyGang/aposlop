use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;

use crate::ingest::discover;
use crate::language::{LanguageId, LanguageRegistry};

use super::configuration::load_config;
type TestResult = anyhow::Result<()>;

#[test]
fn discovery_respects_ignores_excludes_and_supported_extensions() -> TestResult {
    let fixture = TempDir::new()?;
    fs::create_dir(fixture.path().join(".git"))?;
    fs::create_dir(fixture.path().join("generated"))?;
    fs::create_dir_all(fixture.path().join("module/tests"))?;
    fs::create_dir_all(fixture.path().join("module/contest"))?;
    fs::write(fixture.path().join(".gitignore"), "ignored.py\n")?;
    fs::write(fixture.path().join("code.rs"), "fn main() {}\n")?;
    fs::write(fixture.path().join("module.py"), "def run():\n    pass\n")?;
    fs::write(fixture.path().join("view.ts"), "const run = () => 1;\n")?;
    fs::write(
        fixture.path().join("view.tsx"),
        "const View = () => <p />;\n",
    )?;
    fs::write(fixture.path().join("ignored.py"), "def ignored(): pass\n")?;
    fs::write(fixture.path().join("generated/skip.rs"), "fn skip() {}\n")?;
    fs::write(
        fixture.path().join("module/tests/skip.rs"),
        "fn nested_test() {}\n",
    )?;
    fs::write(
        fixture.path().join("module/contest/keep.rs"),
        "fn contest() {}\n",
    )?;
    fs::write(fixture.path().join("notes.txt"), "unsupported\n")?;

    let config = load_config("[core]\nexclude = [\"generated/\", \"**/tests/**\"]")?;
    let registry = LanguageRegistry::compile()?;
    let discovery = discover(fixture.path(), &config, &registry)?;
    let paths: Vec<_> = discovery
        .files
        .iter()
        .map(|file| file.identity.path.clone())
        .collect();

    assert_eq!(
        paths,
        vec![
            PathBuf::from("code.rs"),
            PathBuf::from("module").join("contest").join("keep.rs"),
            PathBuf::from("module.py"),
            PathBuf::from("view.ts"),
            PathBuf::from("view.tsx"),
        ]
    );
    assert!(discovery.diagnostics.is_empty());
    assert_eq!(discovery.files[0].identity.language, LanguageId::Rust);
    assert_eq!(discovery.files[1].identity.language, LanguageId::Rust);
    assert_eq!(discovery.files[2].identity.language, LanguageId::Python);
    assert_eq!(discovery.files[3].identity.language, LanguageId::TypeScript);
    Ok(())
}

#[test]
fn discovery_order_is_stable_across_runs() -> TestResult {
    let fixture = TempDir::new()?;
    for path in ["z.rs", "a.py", "m.tsx", "b.ts"] {
        fs::write(fixture.path().join(path), "fn value() {}\n")?;
    }
    let config = load_config("[core]\nexclude = []")?;
    let registry = LanguageRegistry::compile()?;

    let first = discover(fixture.path(), &config, &registry)?;
    let second = discover(fixture.path(), &config, &registry)?;
    let first_paths: Vec<_> = first.files.iter().map(|file| &file.identity.path).collect();
    let second_paths: Vec<_> = second
        .files
        .iter()
        .map(|file| &file.identity.path)
        .collect();

    assert_eq!(first_paths, second_paths);
    assert!(first_paths.windows(2).all(|pair| pair[0] < pair[1]));
    Ok(())
}
