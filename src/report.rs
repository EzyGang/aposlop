use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use thiserror::Error;

use crate::allow_list::AllowList;
use crate::analysis::{AnalysisDiagnosticKind, AnalyzedFile, SourceLocation};
use crate::cache::CacheDiagnostic;
use crate::config::Config;
use crate::detection::{CloneGroup, FindingId};
use crate::ingest::IngestDiagnostic;
use crate::{OutputFormat, TerminalOutput};

pub(crate) const REPORT_SCHEMA_VERSION: u32 = 4;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Report {
    pub(crate) schema_version: u32,
    pub(crate) summary: Summary,
    pub(crate) duplicates: Vec<CloneGroup>,
    pub(crate) complexity: Vec<ComplexityViolation>,
    pub(crate) diagnostics: Vec<Diagnostic>,
}
impl Report {
    #[must_use]
    pub(crate) fn has_findings(&self) -> bool {
        self.summary.duplicate_count > 0 || self.summary.complexity_violation_count > 0
    }
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
    pub(crate) id: FindingId,
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

#[derive(Clone, Copy, Debug)]
pub(crate) struct RenderOptions<'a> {
    pub(crate) format: OutputFormat,
    pub(crate) terminal_output: TerminalOutput,
    pub(crate) root: &'a Path,
    pub(crate) color: bool,
}

#[derive(Debug, Error)]
pub(crate) enum ReportError {
    #[error("failed to write report: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to encode JSON report: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed to read source file {path}: {source}")]
    Source {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[must_use]
pub(crate) fn build(
    files: &[AnalyzedFile],
    mut duplicates: Vec<CloneGroup>,
    config: &Config,
    allow_list: &AllowList,
    ingestion: Vec<IngestDiagnostic>,
    cache: Vec<CacheDiagnostic>,
) -> Report {
    duplicates.retain(|finding| !allow_list.contains(finding.id));
    duplicates.sort_unstable_by(|left, right| {
        left.instances
            .cmp(&right.instances)
            .then(left.kind.cmp(&right.kind))
    });
    let mut complexity = Vec::new();
    let mut diagnostics = Vec::new();
    for file in files {
        let rules = config.rules(file.identity.language.key(), file.identity.extension());
        if rules.calculate_complexity {
            for block in &file.blocks {
                if block.complexity > rules.complexity_threshold {
                    let id = FindingId::for_complexity_location(&block.location);
                    if allow_list.contains(id) {
                        continue;
                    }
                    complexity.push(ComplexityViolation {
                        id,
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
    options: RenderOptions<'_>,
) -> Result<(), ReportError> {
    match options.format {
        OutputFormat::Terminal => crate::report_terminal::render(writer, report, options)?,
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut *writer, report)?;
            writeln!(writer)?;
        }
    }
    Ok(())
}

pub(crate) fn render_ci(writer: &mut impl Write, report: &Report) -> Result<(), ReportError> {
    let status = if report.has_findings() {
        "failed"
    } else {
        "passed"
    };
    writeln!(writer, "Aposlop CI: {status}")?;
    writeln!(
        writer,
        "Duplicate groups: {}",
        report.summary.duplicate_count
    )?;
    writeln!(
        writer,
        "Complexity violations: {}",
        report.summary.complexity_violation_count
    )?;
    Ok(())
}
