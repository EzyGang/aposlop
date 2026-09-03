use std::io::Write;

use crate::report::{Report, ReportError};
use crate::report_terminal::{Styles, section};

pub(crate) fn render(
    writer: &mut impl Write,
    report: &Report,
    style: Styles,
) -> Result<(), ReportError> {
    section(writer, "File length", report.file_length.len(), style)?;
    if report.file_length.is_empty() {
        writeln!(writer, "  None")?;
        return Ok(());
    }

    for item in &report.file_length {
        writeln!(
            writer,
            "  {}: {} lines (maximum {})",
            item.path.display(),
            item.lines,
            item.max_lines
        )?;
    }
    Ok(())
}
