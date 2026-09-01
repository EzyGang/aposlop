use std::fs;
use std::io;

use clap::Parser;
use tempfile::TempDir;

use crate::{Cli, run};
type TestResult = anyhow::Result<()>;

#[test]
fn full_pipeline_is_identical_before_and_after_cache_hit() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join(".aposlop.toml"),
        "[core]\nmin_lines = 1\nmin_nodes = 1\nuse_cache = true\n[metrics]\ncomplexity_threshold = 1",
    )?;
    fs::write(
        fixture.path().join("left.rs"),
        "fn duplicate(value: bool) -> i32 { if value { 1 } else { 0 } }\n",
    )?;
    fs::write(
        fixture.path().join("right.rs"),
        "fn duplicate(value: bool) -> i32 { if value { 1 } else { 0 } }\n",
    )?;

    let target = fixture.path().to_string_lossy().into_owned();
    let arguments = ["aposlop", target.as_str(), "--format", "json"];
    let mut cold = Vec::new();
    run(Cli::try_parse_from(arguments)?, &mut cold)?;
    let mut warm = Vec::new();
    run(Cli::try_parse_from(arguments)?, &mut warm)?;

    assert_eq!(cold, warm);
    let report: serde_json::Value = serde_json::from_slice(&cold)?;
    assert_eq!(report["summary"]["scanned_files"], 2);
    assert_eq!(report["summary"]["duplicate_count"], 1);
    assert_eq!(report["summary"]["complexity_violation_count"], 2);
    assert!(fixture.path().join(".aposlop_cache").is_file());
    Ok(())
}

#[test]
fn cli_overrides_change_pipeline_rules() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join(".aposlop.toml"),
        "[core]\nmin_lines = 1\nmin_nodes = 1\nuse_cache = false\n[duplicates_detection]\ntype_1 = true",
    )?;
    for path in ["left.py", "right.py"] {
        fs::write(
            fixture.path().join(path),
            "def duplicate(value):\n    return value + 1\n",
        )?;
    }
    let target = fixture.path().to_string_lossy().into_owned();
    let mut output = Vec::new();
    run(
        Cli::try_parse_from([
            "aposlop",
            target.as_str(),
            "--format",
            "json",
            "--type-1",
            "false",
        ])?,
        &mut output,
    )?;

    let report: serde_json::Value = serde_json::from_slice(&output)?;
    assert_eq!(report["summary"]["duplicate_count"], 0);
    Ok(())
}

#[test]
fn usage_and_operational_failures_are_actionable() -> TestResult {
    let usage = match Cli::try_parse_from(["aposlop", "--format", "xml"]) {
        Err(error) => error,
        Ok(_) => anyhow::bail!("invalid output format was accepted"),
    };
    assert_eq!(usage.exit_code(), 2);

    let fixture = TempDir::new()?;
    let file = fixture.path().join("not-a-directory");
    fs::write(&file, "data")?;
    let mut output = Vec::new();
    let target = file.to_string_lossy().into_owned();
    let error = match run(
        Cli::try_parse_from(["aposlop", target.as_str()])?,
        &mut output,
    ) {
        Err(error) => error,
        Ok(()) => anyhow::bail!("file target was accepted"),
    };
    assert!(error.to_string().contains("is not a directory"));
    Ok(())
}

#[test]
fn output_failures_are_fatal_and_contextual() -> TestResult {
    let fixture = TempDir::new()?;
    let target = fixture.path().to_string_lossy().into_owned();
    let mut writer = BrokenWriter;
    let error = match run(
        Cli::try_parse_from(["aposlop", target.as_str()])?,
        &mut writer,
    ) {
        Err(error) => error,
        Ok(()) => anyhow::bail!("output failure was ignored"),
    };

    assert!(error.to_string().contains("failed to render report"));
    assert!(!fixture.path().join(".aposlop_cache").exists());
    Ok(())
}

struct BrokenWriter;

impl io::Write for BrokenWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("injected output failure"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
