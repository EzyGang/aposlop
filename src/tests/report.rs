use std::path::PathBuf;

use crate::OutputFormat;
use crate::analysis::{
    AnalysisDiagnostic, AnalysisDiagnosticKind, AnalyzedBlock, AnalyzedFile, SourceLocation,
};
use crate::cache::CacheDiagnostic;
use crate::config::Config;
use crate::detection::{CloneKind, CloneMatch};
use crate::ingest::{FileIdentity, IngestDiagnostic};
use crate::language::LanguageId;
use crate::report::{DiagnosticCategory, REPORT_SCHEMA_VERSION, build, render};
type TestResult = anyhow::Result<()>;

#[test]
fn report_filters_strict_complexity_thresholds_and_sorts_data() -> TestResult {
    let config =
        Config::parse("[core]\nmin_lines = 1\nmin_nodes = 1\n[metrics]\ncomplexity_threshold = 3")?;
    let mut file = analyzed_file("source.rs", &[3, 4]);
    file.diagnostics.push(AnalysisDiagnostic {
        path: PathBuf::from("source.rs"),
        kind: AnalysisDiagnosticKind::PartialParse,
    });
    let duplicate = CloneMatch {
        kind: CloneKind::Type2,
        similarity: 1.0,
        left: location("source.rs", 1),
        right: location("source.rs", 6),
    };
    let report = build(
        &[file],
        vec![duplicate],
        &config,
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
        &Config::parse("[metrics]\ncomplexity_threshold = 1")?,
        Vec::new(),
        Vec::new(),
    );
    let disabled = build(
        std::slice::from_ref(&file),
        Vec::new(),
        &Config::parse("[metrics]\ncalculate_complexity = false")?,
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
    let config = Config::parse("[core]\nmin_lines = 1\nmin_nodes = 1")?;
    let duplicate = CloneMatch {
        kind: CloneKind::Type1,
        similarity: 1.0,
        left: location("left.rs", 1),
        right: location("right.rs", 1),
    };
    let report = build(
        &[analyzed_file("source.rs", &[1])],
        vec![duplicate],
        &config,
        Vec::new(),
        Vec::new(),
    );
    let mut terminal = Vec::new();
    let mut json = Vec::new();
    render(&mut terminal, &report, OutputFormat::Terminal)?;
    render(&mut json, &report, OutputFormat::Json)?;

    let terminal = String::from_utf8(terminal)?;
    assert!(terminal.contains("Duplicates"));
    assert!(terminal.contains("Complexity"));
    assert!(terminal.contains("Diagnostics"));
    assert!(terminal.contains("Summary"));
    assert!(json.ends_with(b"\n"));
    let value: serde_json::Value = serde_json::from_slice(&json)?;
    assert_eq!(value["schema_version"], REPORT_SCHEMA_VERSION);
    assert_eq!(value["duplicates"][0]["kind"], "type_1");
    assert_eq!(value["summary"]["duplicate_count"], 1);
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
                token_hashes: Vec::new(),
                shingles: vec![index as u64],
                signature: vec![index as u64; 100],
                complexity: *score,
            })
            .collect(),
        diagnostics: Vec::new(),
    }
}

fn location(path: &str, start_line: usize) -> SourceLocation {
    SourceLocation {
        path: PathBuf::from(path),
        start_line,
        end_line: start_line + 4,
    }
}
