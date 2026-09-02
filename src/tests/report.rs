use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::allow_list::AllowList;
use crate::analysis::{
    AnalysisDiagnostic, AnalysisDiagnosticKind, AnalyzedBlock, AnalyzedFile, SourceLocation,
};
use crate::cache::CacheDiagnostic;
use crate::detection::{CloneGroup, CloneKind, FindingId};
use crate::ingest::{FileIdentity, IngestDiagnostic};
use crate::language::LanguageId;
use crate::report::{
    DiagnosticCategory, REPORT_SCHEMA_VERSION, RenderOptions, build, render, render_ci,
};
use crate::{OutputFormat, TerminalOutput};

use super::configuration::load_config;
type TestResult = anyhow::Result<()>;

#[test]
fn report_filters_strict_complexity_thresholds_and_sorts_data() -> TestResult {
    let config =
        load_config("[core]\nmin_lines = 1\nmin_nodes = 1\n[metrics]\ncomplexity_threshold = 3")?;
    let mut file = analyzed_file("source.rs", &[3, 4]);
    file.diagnostics.push(AnalysisDiagnostic {
        path: PathBuf::from("source.rs"),
        kind: AnalysisDiagnosticKind::PartialParse,
    });
    let duplicate = duplicate(CloneKind::Type2, "source.rs", 1, "source.rs", 6);
    let report = build(
        &[file],
        vec![duplicate],
        &config,
        &AllowList::default(),
        vec![IngestDiagnostic {
            path: PathBuf::from("walk"),
            message: "walk failed".to_owned(),
        }],
        vec![CacheDiagnostic {
            path: PathBuf::from(".aposlop_cache"),
            message: "cache failed".to_owned(),
        }],
    );

    assert_eq!(report.schema_version, REPORT_SCHEMA_VERSION);
    assert_eq!(report.summary.scanned_files, 1);
    assert_eq!(report.summary.analyzed_blocks, 2);
    assert_eq!(report.summary.duplicate_count, 1);
    assert_eq!(report.complexity.len(), 1);
    assert_eq!(report.complexity[0].score, 4);
    assert_eq!(report.complexity[0].threshold, 3);
    assert_eq!(report.diagnostics.len(), 3);
    assert_eq!(report.diagnostics[0].category, DiagnosticCategory::Cache);
    Ok(())
}

#[test]
fn disabling_complexity_changes_reports_without_changing_analysis() -> TestResult {
    let file = analyzed_file("source.rs", &[99]);
    let enabled = build(
        std::slice::from_ref(&file),
        Vec::new(),
        &load_config("[metrics]\ncomplexity_threshold = 1")?,
        &AllowList::default(),
        Vec::new(),
        Vec::new(),
    );
    let disabled = build(
        std::slice::from_ref(&file),
        Vec::new(),
        &load_config("[metrics]\ncalculate_complexity = false")?,
        &AllowList::default(),
        Vec::new(),
        Vec::new(),
    );

    assert_eq!(file.blocks[0].complexity, 99);
    assert_eq!(enabled.complexity.len(), 1);
    assert!(disabled.complexity.is_empty());
    Ok(())
}

#[test]
fn terminal_and_json_render_the_same_report_contract() -> TestResult {
    let config = load_config("[core]\nmin_lines = 1\nmin_nodes = 1")?;
    let duplicate = duplicate(CloneKind::Type1, "left.rs", 1, "right.rs", 1);
    let report = build(
        &[analyzed_file("source.rs", &[16])],
        vec![duplicate],
        &config,
        &AllowList::default(),
        Vec::new(),
        Vec::new(),
    );
    let mut terminal = Vec::new();
    let mut json = Vec::new();
    render(
        &mut terminal,
        &report,
        RenderOptions {
            format: OutputFormat::Terminal,
            terminal_output: TerminalOutput::Locations,
            root: Path::new("."),
            color: false,
        },
    )?;
    render(
        &mut json,
        &report,
        RenderOptions {
            format: OutputFormat::Json,
            terminal_output: TerminalOutput::Code,
            root: Path::new("."),
            color: false,
        },
    )?;

    let terminal = String::from_utf8(terminal)?;
    assert!(terminal.contains("Duplicate groups (1)"));
    assert!(terminal.contains("left.rs:1"));
    assert!(terminal.contains("lines 1–5 (5 lines)"));
    assert!(terminal.contains(&report.duplicates[0].id.to_string()));
    assert!(terminal.contains("Type-1"));
    assert!(terminal.contains("Complexity (1)"));
    assert!(terminal.contains(&report.complexity[0].id.to_string()));
    assert!(terminal.contains("Diagnostics (0)"));
    assert!(terminal.contains("Summary"));
    assert!(terminal.ends_with("\nUnused ignores (0)\n  None\n"));
    assert!(json.ends_with(b"\n"));
    let value: serde_json::Value = serde_json::from_slice(&json)?;
    assert_eq!(value["schema_version"], 5);
    assert_eq!(value["duplicates"][0]["kind"], "type_1");
    assert_eq!(value["duplicates"][0]["minimum_similarity"], 1.0);
    assert_eq!(
        value["duplicates"][0]["instances"].as_array().map(Vec::len),
        Some(2)
    );
    assert!(
        !value["duplicates"][0].as_object().is_some_and(
            |duplicate| duplicate.contains_key("left") || duplicate.contains_key("right")
        )
    );
    assert_eq!(value["summary"]["duplicate_count"], 1);
    assert_eq!(
        value["complexity"][0]["id"],
        report.complexity[0].id.to_string()
    );
    let finding_id = value["duplicates"][0]["id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("JSON duplicate did not contain an ID"))?;
    assert_eq!(finding_id.len(), 5);
    assert!(finding_id.as_bytes()[0].is_ascii_alphanumeric());
    assert!(
        finding_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    );
    assert_eq!(value["unused_ignores"], serde_json::json!([]));
    Ok(())
}

#[test]
fn report_tracks_used_and_unused_ignore_ids() -> TestResult {
    let fixture = TempDir::new()?;
    let duplicate = duplicate(CloneKind::Type1, "left.rs", 1, "right.rs", 1);
    let duplicate_id = duplicate.id;
    let complexity_id = FindingId::for_complexity_location(&location("source.rs", 1));
    let unused_id = FindingId::for_complexity_location(&location("removed.rs", 1));
    fs::write(
        fixture.path().join(".aposlopignore"),
        format!("{duplicate_id}\n{complexity_id}\n{unused_id}\n{unused_id}\n"),
    )?;
    let allow_list = AllowList::load(fixture.path())?;
    let report = build(
        &[analyzed_file("source.rs", &[4])],
        vec![duplicate],
        &load_config("[core]\nmin_lines = 1\nmin_nodes = 1\n[metrics]\ncomplexity_threshold = 3")?,
        &allow_list,
        Vec::new(),
        Vec::new(),
    );

    assert!(report.duplicates.is_empty());
    assert!(report.complexity.is_empty());
    assert_eq!(report.unused_ignores, vec![unused_id]);
    assert!(!report.has_findings());

    let mut terminal = Vec::new();
    render(
        &mut terminal,
        &report,
        RenderOptions {
            format: OutputFormat::Terminal,
            terminal_output: TerminalOutput::Locations,
            root: fixture.path(),
            color: false,
        },
    )?;
    let terminal = String::from_utf8(terminal)?;
    let summary = terminal
        .find("\nSummary\n")
        .ok_or_else(|| anyhow::anyhow!("terminal report did not contain Summary"))?;
    let unused = terminal
        .find("\nUnused ignores (1)\n")
        .ok_or_else(|| anyhow::anyhow!("terminal report did not contain unused ignores"))?;
    assert!(summary < unused);
    assert!(terminal.ends_with(&format!("  {unused_id}\n")));

    let mut json = Vec::new();
    render(
        &mut json,
        &report,
        RenderOptions {
            format: OutputFormat::Json,
            terminal_output: TerminalOutput::Code,
            root: fixture.path(),
            color: false,
        },
    )?;
    let value: serde_json::Value = serde_json::from_slice(&json)?;
    assert_eq!(value["schema_version"], 5);
    assert_eq!(
        value["unused_ignores"],
        serde_json::json!([unused_id.to_string()])
    );

    let mut ci = Vec::new();
    render_ci(&mut ci, &report)?;
    assert_eq!(
        String::from_utf8(ci)?,
        "Aposlop CI: passed\nDuplicate groups: 0\nComplexity violations: 0\nUnused ignores: 1\n"
    );
    Ok(())
}

#[test]
fn ci_output_is_a_compact_finding_summary() -> TestResult {
    let duplicate = duplicate(CloneKind::Type1, "left.rs", 1, "right.rs", 1);
    let failing = build(
        &[analyzed_file("source.rs", &[4])],
        vec![duplicate],
        &load_config("[core]\nmin_lines = 1\nmin_nodes = 1\n[metrics]\ncomplexity_threshold = 3")?,
        &AllowList::default(),
        Vec::new(),
        Vec::new(),
    );
    let passing = build(
        &[analyzed_file("source.rs", &[3])],
        Vec::new(),
        &load_config("[metrics]\ncomplexity_threshold = 3")?,
        &AllowList::default(),
        Vec::new(),
        Vec::new(),
    );
    let mut failing_output = Vec::new();
    let mut passing_output = Vec::new();

    for (report, output) in [
        (&failing, &mut failing_output),
        (&passing, &mut passing_output),
    ] {
        render_ci(output, report)?;
    }

    assert_eq!(
        String::from_utf8(failing_output)?,
        "Aposlop CI: failed\nDuplicate groups: 1\nComplexity violations: 1\nUnused ignores: 0\n"
    );
    assert_eq!(
        String::from_utf8(passing_output)?,
        "Aposlop CI: passed\nDuplicate groups: 0\nComplexity violations: 0\nUnused ignores: 0\n"
    );
    Ok(())
}

#[test]
fn finding_ids_are_deterministic_and_membership_sensitive() -> TestResult {
    let left = location("src/left.rs", 10);
    let right = location("src/right.rs", 20);
    let third = location("src/third.rs", 30);
    let id = FindingId::for_duplicate_locations(&[left.clone(), right.clone()]);

    assert_eq!(
        id,
        FindingId::for_duplicate_locations(&[left.clone(), right.clone()])
    );
    assert_eq!(
        id,
        FindingId::for_duplicate_locations(&[right.clone(), left.clone()])
    );
    assert_ne!(
        id,
        FindingId::for_duplicate_locations(&[left.clone(), right, third])
    );
    let complexity_id = FindingId::for_complexity_location(&left);
    assert_eq!(complexity_id, FindingId::for_complexity_location(&left));
    assert_ne!(id, complexity_id);
    assert_eq!(id.to_string().parse::<FindingId>()?, id);
    assert!("abcd".parse::<FindingId>().is_err());
    assert!("-abcd".parse::<FindingId>().is_err());
    assert!("abc.d".parse::<FindingId>().is_err());
    Ok(())
}

#[test]
fn terminal_code_output_prints_every_duplicate_instance() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("left.rs"),
        "fn left() {\n    let value = 1;\n    println!(\"{value}\");\n    drop(value);\n}\n",
    )?;
    fs::write(
        fixture.path().join("right.rs"),
        "fn right() {\n    let value = 2;\n    println!(\"{value}\");\n    drop(value);\n}\n",
    )?;
    let report = build(
        &[analyzed_file("source.rs", &[1])],
        vec![duplicate(CloneKind::Type3, "left.rs", 1, "right.rs", 1)],
        &load_config("[core]\nmin_lines = 1\nmin_nodes = 1")?,
        &AllowList::default(),
        Vec::new(),
        Vec::new(),
    );
    let mut output = Vec::new();

    render(
        &mut output,
        &report,
        RenderOptions {
            format: OutputFormat::Terminal,
            terminal_output: TerminalOutput::Code,
            root: fixture.path(),
            color: false,
        },
    )?;

    let output = String::from_utf8(output)?;
    assert!(output.contains("  Instance 1 code\n    1 │ fn left() {"));
    assert!(output.contains("  Instance 2 code\n    1 │ fn right() {"));
    Ok(())
}

fn analyzed_file(path: &str, scores: &[usize]) -> AnalyzedFile {
    AnalyzedFile {
        identity: FileIdentity {
            path: PathBuf::from(path),
            size: 1,
            modified_seconds: 1,
            modified_nanoseconds: 0,
            language: LanguageId::Rust,
        },
        blocks: scores
            .iter()
            .enumerate()
            .map(|(index, score)| AnalyzedBlock {
                location: location(path, index * 5 + 1),
                start_byte: index * 10,
                end_byte: index * 10 + 9,
                line_count: 5,
                named_node_count: 30,
                exact: vec![index as u8],
                normalized: vec![index as u8],
                exact_hash: index as u64,
                normalized_hash: index as u64,
                shingles: vec![index as u64],
                complexity: *score,
            })
            .collect(),
        diagnostics: Vec::new(),
    }
}

fn duplicate(
    kind: CloneKind,
    left_path: &str,
    left_line: usize,
    right_path: &str,
    right_line: usize,
) -> CloneGroup {
    let instances = vec![
        location(left_path, left_line),
        location(right_path, right_line),
    ];
    CloneGroup {
        kind,
        minimum_similarity: 1.0,
        id: FindingId::for_duplicate_locations(&instances),
        instances,
    }
}

fn location(path: &str, start_line: usize) -> SourceLocation {
    SourceLocation {
        path: PathBuf::from(path),
        start_line,
        end_line: start_line + 4,
    }
}
