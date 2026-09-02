use std::fs;
use std::io;

use clap::Parser;
use tempfile::TempDir;

use crate::{Cli, CommandStatus, run};
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
fn five_duplicates_render_as_one_group() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join(".aposlop.toml"),
        "[core]\nmin_lines = 1\nmin_nodes = 1\nuse_cache = false\n[metrics]\ncalculate_complexity = false",
    )?;
    for path in ["a.py", "b.py", "c.py", "d.py", "e.py"] {
        fs::write(
            fixture.path().join(path),
            "def duplicate(value):\n    return value + 1\n",
        )?;
    }
    let target = fixture.path().to_string_lossy().into_owned();
    let mut json = Vec::new();
    run(
        Cli::try_parse_from(["aposlop", target.as_str(), "--format", "json"])?,
        &mut json,
    )?;
    let report: serde_json::Value = serde_json::from_slice(&json)?;
    assert_eq!(report["schema_version"], 4);
    assert_eq!(report["summary"]["duplicate_count"], 1);
    assert_eq!(
        report["duplicates"][0]["instances"]
            .as_array()
            .map(Vec::len),
        Some(5)
    );

    let mut terminal = Vec::new();
    run(
        Cli::try_parse_from(["aposlop", target.as_str(), "--terminal-output", "code"])?,
        &mut terminal,
    )?;
    let terminal = String::from_utf8(terminal)?;
    assert!(terminal.contains("Duplicate groups (1)"));
    assert!(terminal.contains("  Instances   5"));
    for index in 1..=5 {
        assert!(terminal.contains(&format!("  Instance {index} code")));
    }
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
fn allow_command_suppresses_finding_until_id_is_removed() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join(".aposlop.toml"),
        "[core]\nmin_lines = 1\nmin_nodes = 1\nuse_cache = false\n[metrics]\ncomplexity_threshold = 1",
    )?;
    for path in ["a.rs", "b.rs", "c.rs", "d.rs", "e.rs"] {
        fs::write(
            fixture.path().join(path),
            "fn duplicate(value: bool) -> i32 { if value { 1 } else { 0 } }\n",
        )?;
    }
    let target = fixture.path().to_string_lossy().into_owned();
    let scan = ["aposlop", target.as_str(), "--format", "json"];
    let mut initial = Vec::new();
    run(Cli::try_parse_from(scan)?, &mut initial)?;
    let report: serde_json::Value = serde_json::from_slice(&initial)?;
    let finding = report["duplicates"][0]["id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("report did not contain a duplicate ID"))?
        .to_owned();
    assert_eq!(
        report["duplicates"][0]["instances"]
            .as_array()
            .map(Vec::len),
        Some(5)
    );
    let complexity_ids: Vec<_> = report["complexity"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("report did not contain complexity findings"))?
        .iter()
        .map(|finding| {
            finding["id"]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| anyhow::anyhow!("complexity finding did not contain an ID"))
        })
        .collect::<Result<_, _>>()?;
    assert_eq!(complexity_ids.len(), 5);

    let mut command_output = Vec::new();
    run(
        Cli::try_parse_from(["aposlop", "allow", finding.as_str(), target.as_str()])?,
        &mut command_output,
    )?;

    let allow_list = fs::read_to_string(fixture.path().join(".aposlopignore"))?;
    assert!(allow_list.contains(&finding));
    let mut suppressed = Vec::new();
    run(Cli::try_parse_from(scan)?, &mut suppressed)?;
    let report: serde_json::Value = serde_json::from_slice(&suppressed)?;
    assert_eq!(report["summary"]["duplicate_count"], 0);
    assert_eq!(report["summary"]["complexity_violation_count"], 5);

    for complexity_id in &complexity_ids {
        run(
            Cli::try_parse_from(["aposlop", "allow", complexity_id.as_str(), target.as_str()])?,
            &mut command_output,
        )?;
    }
    let mut fully_suppressed = Vec::new();
    run(Cli::try_parse_from(scan)?, &mut fully_suppressed)?;
    let report: serde_json::Value = serde_json::from_slice(&fully_suppressed)?;
    assert_eq!(report["summary"]["duplicate_count"], 0);
    assert_eq!(report["summary"]["complexity_violation_count"], 0);

    fs::write(
        fixture.path().join(".aposlopignore"),
        "# Manually excluded Aposlop findings.\n",
    )?;
    let mut restored = Vec::new();
    run(Cli::try_parse_from(scan)?, &mut restored)?;
    let report: serde_json::Value = serde_json::from_slice(&restored)?;
    assert_eq!(report["summary"]["duplicate_count"], 1);
    assert_eq!(report["summary"]["complexity_violation_count"], 5);
    Ok(())
}

#[test]
fn ci_output_returns_finding_and_success_statuses() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join(".aposlop.toml"),
        "[core]\nmin_lines = 1\nmin_nodes = 1\nuse_cache = false\n[metrics]\ncalculate_complexity = false",
    )?;
    for path in ["left.rs", "right.rs"] {
        fs::write(
            fixture.path().join(path),
            "fn duplicate(value: bool) -> i32 { if value { 1 } else { 0 } }\n",
        )?;
    }
    let target = fixture.path().to_string_lossy().into_owned();
    let arguments = ["aposlop", "ci", target.as_str()];
    let mut failing = Vec::new();

    let status = run(Cli::try_parse_from(arguments)?, &mut failing)?;

    assert_eq!(status, CommandStatus::Findings);
    assert_eq!(
        String::from_utf8(failing)?,
        "Aposlop CI: failed\nDuplicate groups: 1\nComplexity violations: 0\n"
    );

    let mut overridden = Vec::new();
    let status = run(
        Cli::try_parse_from(["aposlop", "ci", target.as_str(), "--exclude", "right.rs"])?,
        &mut overridden,
    )?;
    assert_eq!(status, CommandStatus::Success);
    assert_eq!(
        String::from_utf8(overridden)?,
        "Aposlop CI: passed\nDuplicate groups: 0\nComplexity violations: 0\n"
    );

    fs::remove_file(fixture.path().join("right.rs"))?;
    let mut passing = Vec::new();
    let status = run(Cli::try_parse_from(arguments)?, &mut passing)?;
    assert_eq!(status, CommandStatus::Success);
    assert_eq!(
        String::from_utf8(passing)?,
        "Aposlop CI: passed\nDuplicate groups: 0\nComplexity violations: 0\n"
    );
    Ok(())
}

#[test]
fn malformed_allow_list_is_an_actionable_failure() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(fixture.path().join(".aposlopignore"), "not-a-finding\n")?;
    let target = fixture.path().to_string_lossy().into_owned();
    let mut output = Vec::new();

    let error = match run(
        Cli::try_parse_from(["aposlop", target.as_str()])?,
        &mut output,
    ) {
        Err(error) => error,
        Ok(_) => anyhow::bail!("malformed allow list was accepted"),
    };

    let message = format!("{error:#}");
    assert!(message.contains("failed to load allow list"));
    assert!(message.contains(".aposlopignore at line 1"));
    Ok(())
}

#[test]
fn usage_and_operational_failures_are_actionable() -> TestResult {
    let usage = match Cli::try_parse_from(["aposlop", "--format", "ci"]) {
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
        Ok(_) => anyhow::bail!("file target was accepted"),
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
        Ok(_) => anyhow::bail!("output failure was ignored"),
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
