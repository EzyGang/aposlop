mod allow_list;
mod analysis;
mod cache;
mod cli;
mod config;
mod detection;
mod ingest;
mod install_skills;
mod language;
mod report;
mod report_file_length;
mod report_terminal;
mod update;

#[cfg(test)]
mod tests;

use std::io::{self, IsTerminal};
use std::path::Path;
use std::process::ExitCode;

use anyhow::{Context, bail};
use clap::Parser;

use crate::allow_list::{AddOutcome, AllowList};
use crate::cli::{Cli, Command, OutputFormat, TerminalOutput};
use crate::config::Config;
use crate::detection::FindingId;
use crate::language::LanguageRegistry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandStatus {
    Success,
    Findings,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScanMode {
    Report,
    Ci,
}

fn main() -> ExitCode {
    let stdout = io::stdout();
    let interactive = stdout.is_terminal();
    let color = interactive && std::env::var_os("NO_COLOR").is_none();
    let mut stdout = stdout.lock();
    match run_with_color(Cli::parse(), &mut stdout, color) {
        Ok(status) => {
            if interactive && std::env::var_os("APOSLOP_NO_UPDATE_CHECK").is_none() {
                warn_if_update_available();
            }
            status.exit_code()
        }
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn warn_if_update_available() {
    match update::check() {
        Ok(Some(available)) => {
            eprintln!(
                "warning: aposlop {} is available; update at {}",
                available.version, available.url
            );
        }
        Ok(None) => (),
        Err(error) => eprintln!("warning: update check failed: {error}"),
    }
}

#[cfg(test)]
fn run(cli: Cli, writer: &mut impl io::Write) -> anyhow::Result<CommandStatus> {
    run_with_color(cli, writer, false)
}

fn run_with_color(
    cli: Cli,
    writer: &mut impl io::Write,
    color: bool,
) -> anyhow::Result<CommandStatus> {
    match &cli.command {
        Some(Command::Allow { finding, path }) => run_allow(path, *finding, writer),
        Some(Command::Ci { path }) => {
            if cli.format != OutputFormat::Terminal {
                bail!("--format cannot be used with the ci command");
            }
            if cli.terminal_output != TerminalOutput::Locations {
                bail!("--terminal-output cannot be used with the ci command");
            }
            run_scan(path, &cli, writer, color, ScanMode::Ci)
        }
        Some(Command::InstallSkills) => {
            install_skills::run()?;
            Ok(CommandStatus::Success)
        }
        None => run_scan(&cli.path, &cli, writer, color, ScanMode::Report),
    }
}

fn run_allow(
    root: &Path,
    finding: FindingId,
    writer: &mut impl io::Write,
) -> anyhow::Result<CommandStatus> {
    ensure_directory(root)?;
    let outcome = AllowList::add(root, finding)
        .with_context(|| format!("failed to update allow list for {}", root.display()))?;
    match outcome {
        AddOutcome::Added => writeln!(writer, "Allowed {finding} in {}", root.display())?,
        AddOutcome::AlreadyPresent => {
            writeln!(writer, "{finding} is already allowed in {}", root.display())?;
        }
    }
    Ok(CommandStatus::Success)
}

fn run_scan(
    root: &Path,
    cli: &Cli,
    writer: &mut impl io::Write,
    color: bool,
    mode: ScanMode,
) -> anyhow::Result<CommandStatus> {
    ensure_directory(root)?;
    let config = Config::load(root)
        .with_context(|| format!("failed to configure target {}", root.display()))?
        .apply_cli(cli.overrides())
        .with_context(|| format!("failed to configure target {}", root.display()))?;
    let allow_list = AllowList::load(root)
        .with_context(|| format!("failed to load allow list for {}", root.display()))?;
    let registry =
        LanguageRegistry::compile().context("failed to initialize language providers")?;
    let discovery = ingest::discover(root, &config, &registry)
        .with_context(|| format!("failed to discover files under {}", root.display()))?;
    let resolution = cache::resolve(root, config.use_cache(), discovery.files);
    let mut files = resolution.hits;
    files.extend(
        analysis::analyze(resolution.misses, &registry)
            .with_context(|| format!("failed to analyze target {}", root.display()))?,
    );
    files.sort_unstable_by(|left, right| left.identity.path.cmp(&right.identity.path));
    let duplicates = detection::detect(&files, &config);
    let report = report::build(
        &files,
        duplicates,
        &config,
        &allow_list,
        discovery.diagnostics,
        resolution.diagnostics,
    );
    let render_result = match mode {
        ScanMode::Report => report::render(
            writer,
            &report,
            report::RenderOptions {
                format: cli.format,
                terminal_output: cli.terminal_output,
                root,
                color,
            },
        ),
        ScanMode::Ci => report::render_ci(writer, &report),
    };
    render_result.with_context(|| format!("failed to render report for {}", root.display()))?;
    cache::write(root, config.use_cache(), &files)
        .with_context(|| format!("failed to persist cache for {}", root.display()))?;
    if mode == ScanMode::Ci && report.has_findings() {
        Ok(CommandStatus::Findings)
    } else {
        Ok(CommandStatus::Success)
    }
}

impl CommandStatus {
    fn exit_code(self) -> ExitCode {
        match self {
            Self::Success => ExitCode::SUCCESS,
            Self::Findings => ExitCode::FAILURE,
        }
    }
}

fn ensure_directory(path: &Path) -> anyhow::Result<()> {
    let metadata = std::fs::metadata(path)
        .with_context(|| format!("failed to inspect target {}", path.display()))?;
    if !metadata.is_dir() {
        bail!("target {} is not a directory", path.display());
    }
    Ok(())
}
