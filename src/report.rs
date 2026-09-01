use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;
use thiserror::Error;

use crate::OutputFormat;
use crate::analysis::{AnalysisDiagnosticKind, AnalyzedFile, SourceLocation};
use crate::cache::CacheDiagnostic;
use crate::config::Config;
use crate::detection::CloneMatch;
use crate::ingest::IngestDiagnostic;

pub(crate) const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Report {
    pub(crate) schema_version: u32,
    pub(crate) summary: Summary,
    pub(crate) duplicates: Vec<CloneMatch>,
    pub(crate) complexity: Vec<ComplexityViolation>,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Summary {
    pub(crate) scanned_files: usize,
    pub(crate) analyzed_blocks: usize,
    pub(crate) duplicate_count: usize,
    pub(crate) complexity_violation_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ComplexityViolation {
    pub(crate) location: SourceLocation,
    pub(crate) score: usize,
    pub(crate) threshold: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Diagnostic {
    pub(crate) path: PathBuf,
    pub(crate) category: DiagnosticCategory,
    pub(crate) message: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DiagnosticCategory {
    Analysis,
    Cache,
    Ingestion,
}

#[derive(Debug, Error)]
pub(crate) enum ReportError {
    #[error("failed to write report: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to encode JSON report: {0}")]
    Json(#[from] serde_json::Error),
}

#[must_use]
pub(crate) fn build(
    files: &[AnalyzedFile],
    mut duplicates: Vec<CloneMatch>,
    config: &Config,
    ingestion: Vec<IngestDiagnostic>,
    cache: Vec<CacheDiagnostic>,
) -> Report {
    duplicates.sort_unstable_by(|left, right| {
        left.kind
            .cmp(&right.kind)
            .then(left.left.cmp(&right.left))
            .then(left.right.cmp(&right.right))
    });
    let mut complexity = Vec::new();
    let mut diagnostics = Vec::new();
    for file in files {
        let rules = config.rules(file.identity.language.key(), file.identity.extension());
        if rules.calculate_complexity {
            for block in &file.blocks {
                if block.complexity > rules.complexity_threshold {
                    complexity.push(ComplexityViolation {
                        location: block.location.clone(),
                        score: block.complexity,
                        threshold: rules.complexity_threshold,
                    });
                }
            }
        }
        for item in &file.diagnostics {
            let message = match &item.kind {
                AnalysisDiagnosticKind::Read(message) => {
                    format!("failed to read source: {message}")
                }
                AnalysisDiagnosticKind::Parse(message) => {
                    format!("failed to parse source: {message}")
                }
                AnalysisDiagnosticKind::PartialParse => {
                    "The file contains syntax errors. Aposlop skipped invalid blocks.".to_owned()
                }
            };
            diagnostics.push(Diagnostic {
                path: item.path.clone(),
                category: DiagnosticCategory::Analysis,
                message,
            });
        }
    }
    diagnostics.extend(ingestion.into_iter().map(|item| Diagnostic {
        path: item.path,
        category: DiagnosticCategory::Ingestion,
        message: item.message,
    }));
    diagnostics.extend(cache.into_iter().map(|item| Diagnostic {
        path: item.path,
        category: DiagnosticCategory::Cache,
        message: item.message,
    }));
    complexity.sort_unstable_by(|left, right| {
        left.location
            .cmp(&right.location)
            .then(left.score.cmp(&right.score))
    });
    diagnostics.sort_unstable_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.category.cmp(&right.category))
            .then(left.message.cmp(&right.message))
    });
    let summary = Summary {
        scanned_files: files.len(),
        analyzed_blocks: files.iter().map(|file| file.blocks.len()).sum(),
        duplicate_count: duplicates.len(),
        complexity_violation_count: complexity.len(),
    };
    Report {
        schema_version: REPORT_SCHEMA_VERSION,
        summary,
        duplicates,
        complexity,
        diagnostics,
    }
}

pub(crate) fn render(
    writer: &mut impl Write,
    report: &Report,
    format: OutputFormat,
) -> Result<(), ReportError> {
    match format {
        OutputFormat::Terminal => render_terminal(writer, report)?,
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut *writer, report)?;
            writeln!(writer)?;
        }
    }
    Ok(())
}

fn render_terminal(writer: &mut impl Write, report: &Report) -> std::io::Result<()> {
    writeln!(writer, "Aposlop report")?;
    writeln!(writer, "\nDuplicates")?;
    if report.duplicates.is_empty() {
        writeln!(writer, "None")?;
    } else {
        writeln!(writer, "TYPE\tSIMILARITY\tLEFT\tRIGHT")?;
        for item in &report.duplicates {
            writeln!(
                writer,
                "{:?}\t{:.3}\t{}:{}-{}\t{}:{}-{}",
                item.kind,
                item.similarity,
                item.left.path.display(),
                item.left.start_line,
                item.left.end_line,
                item.right.path.display(),
                item.right.start_line,
                item.right.end_line
            )?;
        }
    }
    writeln!(writer, "\nComplexity")?;
    if report.complexity.is_empty() {
        writeln!(writer, "None")?;
    } else {
        writeln!(writer, "SCORE\tTHRESHOLD\tLOCATION")?;
        for item in &report.complexity {
            writeln!(
                writer,
                "{}\t{}\t{}:{}-{}",
                item.score,
                item.threshold,
                item.location.path.display(),
                item.location.start_line,
                item.location.end_line
            )?;
        }
    }
    writeln!(writer, "\nDiagnostics")?;
    if report.diagnostics.is_empty() {
        writeln!(writer, "None")?;
    } else {
        writeln!(writer, "CATEGORY\tPATH\tMESSAGE")?;
        for item in &report.diagnostics {
            writeln!(
                writer,
                "{:?}\t{}\t{}",
                item.category,
                item.path.display(),
                item.message
            )?;
        }
    }
    writeln!(writer, "\nSummary")?;
    writeln!(writer, "Scanned files: {}", report.summary.scanned_files)?;
    writeln!(
        writer,
        "Analyzed blocks: {}",
        report.summary.analyzed_blocks
    )?;
    writeln!(writer, "Duplicates: {}", report.summary.duplicate_count)?;
    writeln!(
        writer,
        "Complexity violations: {}",
        report.summary.complexity_violation_count
    )?;
    Ok(())
}
