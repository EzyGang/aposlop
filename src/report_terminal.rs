use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::TerminalOutput;
use crate::analysis::SourceLocation;
use crate::detection::CloneMatch;
use crate::report::{RenderOptions, Report, ReportError};

pub(crate) fn render(
    writer: &mut impl Write,
    report: &Report,
    options: RenderOptions<'_>,
) -> Result<(), ReportError> {
    let style = Styles::new(options.color);
    let sources = match options.terminal_output {
        TerminalOutput::Locations => None,
        TerminalOutput::Code => Some(load_sources(options.root, &report.duplicates)?),
    };
    writeln!(writer, "{}Aposlop report{}", style.title(), style.reset())?;
    render_duplicates(writer, report, sources.as_ref(), style)?;
    render_complexity(writer, report, style)?;
    render_diagnostics(writer, report, style)?;
    render_summary(writer, report, style)?;
    Ok(())
}

fn render_duplicates(
    writer: &mut impl Write,
    report: &Report,
    sources: Option<&BTreeMap<PathBuf, SourceText>>,
    style: Styles,
) -> Result<(), ReportError> {
    section(writer, "Duplicates", report.duplicates.len(), style)?;
    if report.duplicates.is_empty() {
        writeln!(writer, "  None")?;
        return Ok(());
    }
    for (index, item) in report.duplicates.iter().enumerate() {
        if index > 0 {
            writeln!(writer)?;
        }
        writeln!(writer, "{}{}{}", style.finding(), item.id, style.reset())?;
        writeln!(writer, "  Similarity  {:.1}%", item.similarity * 100.0)?;
        render_location(writer, "Left", &item.left)?;
        render_location(writer, "Right", &item.right)?;
        if let Some(sources) = sources {
            render_excerpt(writer, "Left", &item.left, sources)?;
            render_excerpt(writer, "Right", &item.right, sources)?;
        }
    }
    Ok(())
}

fn render_complexity(
    writer: &mut impl Write,
    report: &Report,
    style: Styles,
) -> Result<(), ReportError> {
    section(writer, "Complexity", report.complexity.len(), style)?;
    if report.complexity.is_empty() {
        writeln!(writer, "  None")?;
        return Ok(());
    }
    for (index, item) in report.complexity.iter().enumerate() {
        if index > 0 {
            writeln!(writer)?;
        }
        writeln!(writer, "{}{}{}", style.finding(), item.id, style.reset())?;
        writeln!(
            writer,
            "  Score {} (threshold {})",
            item.score, item.threshold
        )?;
        render_location(writer, "Location", &item.location)?;
    }
    Ok(())
}

fn render_diagnostics(
    writer: &mut impl Write,
    report: &Report,
    style: Styles,
) -> Result<(), ReportError> {
    section(writer, "Diagnostics", report.diagnostics.len(), style)?;
    if report.diagnostics.is_empty() {
        writeln!(writer, "  None")?;
        return Ok(());
    }
    for (index, item) in report.diagnostics.iter().enumerate() {
        if index > 0 {
            writeln!(writer)?;
        }
        writeln!(writer, "{:?}  {}", item.category, item.path.display())?;
        writeln!(writer, "  {}", item.message)?;
    }
    Ok(())
}

fn render_summary(
    writer: &mut impl Write,
    report: &Report,
    style: Styles,
) -> Result<(), ReportError> {
    writeln!(writer, "\n{}Summary{}", style.section(), style.reset())?;
    writeln!(writer, "  Scanned files: {}", report.summary.scanned_files)?;
    writeln!(
        writer,
        "  Analyzed blocks: {}",
        report.summary.analyzed_blocks
    )?;
    writeln!(
        writer,
        "  Duplicate findings: {}",
        report.summary.duplicate_count
    )?;
    writeln!(
        writer,
        "  Complexity violations: {}",
        report.summary.complexity_violation_count
    )?;
    Ok(())
}

fn section(
    writer: &mut impl Write,
    name: &str,
    count: usize,
    style: Styles,
) -> std::io::Result<()> {
    writeln!(
        writer,
        "\n{}{} ({}){}",
        style.section(),
        name,
        count,
        style.reset()
    )
}

fn render_location(
    writer: &mut impl Write,
    label: &str,
    location: &SourceLocation,
) -> std::io::Result<()> {
    let line_count = location
        .end_line
        .saturating_sub(location.start_line)
        .saturating_add(1);
    writeln!(
        writer,
        "  {label:<10} {}:{}",
        location.path.display(),
        location.start_line
    )?;
    writeln!(
        writer,
        "  {:<10} lines {}–{} ({line_count} lines)",
        "", location.start_line, location.end_line
    )
}

fn render_excerpt(
    writer: &mut impl Write,
    label: &str,
    location: &SourceLocation,
    sources: &BTreeMap<PathBuf, SourceText>,
) -> Result<(), ReportError> {
    let Some(source) = sources.get(&location.path) else {
        return Ok(());
    };
    writeln!(writer, "  {label} code")?;
    let width = location.end_line.to_string().len();
    for line_number in location.start_line..=location.end_line {
        let Some(line) = source.line(line_number) else {
            continue;
        };
        writeln!(
            writer,
            "    {line_number:>width$} │ {}",
            String::from_utf8_lossy(line)
        )?;
    }
    Ok(())
}

fn load_sources(
    root: &Path,
    duplicates: &[CloneMatch],
) -> Result<BTreeMap<PathBuf, SourceText>, ReportError> {
    let mut sources = BTreeMap::new();
    for location in duplicates.iter().flat_map(|item| [&item.left, &item.right]) {
        if sources.contains_key(&location.path) {
            continue;
        }
        let path = root.join(&location.path);
        let bytes = fs::read(&path).map_err(|source| ReportError::Source {
            path: path.clone(),
            source,
        })?;
        sources.insert(location.path.clone(), SourceText::new(bytes));
    }
    Ok(sources)
}

struct SourceText {
    bytes: Vec<u8>,
    line_starts: Vec<usize>,
}

impl SourceText {
    fn new(bytes: Vec<u8>) -> Self {
        let mut line_starts = vec![0];
        for (index, byte) in bytes.iter().enumerate() {
            if *byte == b'\n' && index + 1 < bytes.len() {
                line_starts.push(index + 1);
            }
        }
        Self { bytes, line_starts }
    }

    fn line(&self, line_number: usize) -> Option<&[u8]> {
        let start = *self.line_starts.get(line_number.checked_sub(1)?)?;
        let end = self
            .line_starts
            .get(line_number)
            .copied()
            .unwrap_or(self.bytes.len());
        Some(self.bytes[start..end].trim_ascii_end())
    }
}

#[derive(Clone, Copy)]
struct Styles {
    enabled: bool,
}

impl Styles {
    const fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    const fn title(self) -> &'static str {
        if self.enabled { "\x1b[1;36m" } else { "" }
    }

    const fn section(self) -> &'static str {
        if self.enabled { "\x1b[1m" } else { "" }
    }

    const fn finding(self) -> &'static str {
        if self.enabled { "\x1b[1;33m" } else { "" }
    }

    const fn reset(self) -> &'static str {
        if self.enabled { "\x1b[0m" } else { "" }
    }
}
