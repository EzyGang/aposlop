mod allow_list;
mod analysis;
mod cache;
mod config;
mod detection;
mod ingest;
mod language;
mod report;
mod report_terminal;

#[cfg(test)]
mod tests;

use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, bail};
use clap::{ArgAction, Parser, Subcommand, ValueEnum};

use crate::allow_list::{AddOutcome, AllowList};
use crate::config::{CliOverrides, Config, RuleOverride};
use crate::detection::FindingId;
use crate::language::LanguageRegistry;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    #[default]
    Terminal,
    Json,
    Ci,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
enum TerminalOutput {
    #[default]
    Locations,
    Code,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandStatus {
    Success,
    Findings,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Save a finding ID in the target's manual exclusions file.
    Allow {
        /// Duplicate or complexity finding ID to allow.
        finding: FindingId,

        /// Directory that owns the .aposlopignore file.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Debug, Parser)]
#[command(
    name = "aposlop",
    version,
    about = "Detect duplicate code and report cyclomatic complexity"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Directory to analyze.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Select the output format.
    #[arg(long, value_enum, default_value_t)]
    format: OutputFormat,
    /// Select location-only or source-code terminal findings.
    #[arg(long, value_enum, default_value_t)]
    terminal_output: TerminalOutput,

    /// Override the minimum block line count.
    #[arg(long, value_name = "N")]
    min_lines: Option<usize>,

    /// Override the minimum named-node count.
    #[arg(long, value_name = "N")]
    min_nodes: Option<usize>,

    /// Replace configured exclusions relative to the target directory.
    #[arg(long, value_name = "PATH")]
    exclude: Vec<PathBuf>,

    /// Enable or disable the analysis cache.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    use_cache: Option<bool>,

    /// Enable or disable Type-1 duplicate reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    type_1: Option<bool>,

    /// Enable or disable Type-2 duplicate reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    type_2: Option<bool>,

    /// Enable or disable Type-3 duplicate reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    type_3: Option<bool>,

    /// Override the Type-3 Jaccard similarity threshold.
    #[arg(long, value_name = "RATIO")]
    type_3_threshold: Option<f64>,

    /// Enable or disable complexity reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    calculate_complexity: Option<bool>,

    /// Override the complexity violation threshold.
    #[arg(long, value_name = "N")]
    complexity_threshold: Option<usize>,
}

impl Cli {
    fn overrides(&self) -> CliOverrides {
        CliOverrides {
            rules: RuleOverride {
                min_lines: self.min_lines,
                min_nodes: self.min_nodes,
                type_1: self.type_1,
                type_2: self.type_2,
                type_3: self.type_3,
                type_3_threshold: self.type_3_threshold,
                calculate_complexity: self.calculate_complexity,
                complexity_threshold: self.complexity_threshold,
            },
            exclude: (!self.exclude.is_empty()).then(|| self.exclude.clone()),
            use_cache: self.use_cache,
        }
    }
}

fn main() -> ExitCode {
    let stdout = io::stdout();
    let color = stdout.is_terminal() && std::env::var_os("NO_COLOR").is_none();
    let mut stdout = stdout.lock();
    match run_with_color(Cli::parse(), &mut stdout, color) {
        Ok(status) => status.exit_code(),
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
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
    if let Some(Command::Allow { finding, path }) = &cli.command {
        return run_allow(path, *finding, writer);
    }
    run_scan(&cli, writer, color)
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

fn run_scan(cli: &Cli, writer: &mut impl io::Write, color: bool) -> anyhow::Result<CommandStatus> {
    ensure_directory(&cli.path)?;
    let config = Config::load(&cli.path)
        .with_context(|| format!("failed to configure target {}", cli.path.display()))?
        .apply_cli(cli.overrides())
        .with_context(|| format!("failed to configure target {}", cli.path.display()))?;
    let allow_list = AllowList::load(&cli.path)
        .with_context(|| format!("failed to load allow list for {}", cli.path.display()))?;
    let registry =
        LanguageRegistry::compile().context("failed to initialize language providers")?;
    let discovery = ingest::discover(&cli.path, &config, &registry)
        .with_context(|| format!("failed to discover files under {}", cli.path.display()))?;
    let resolution = cache::resolve(&cli.path, config.use_cache(), discovery.files);
    let mut files = resolution.hits;
    files.extend(
        analysis::analyze(resolution.misses, &registry)
            .with_context(|| format!("failed to analyze target {}", cli.path.display()))?,
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
    report::render(
        writer,
        &report,
        report::RenderOptions {
            format: cli.format,
            terminal_output: cli.terminal_output,
            root: &cli.path,
            color,
        },
    )
    .with_context(|| format!("failed to render report for {}", cli.path.display()))?;
    cache::write(&cli.path, config.use_cache(), &files)
        .with_context(|| format!("failed to persist cache for {}", cli.path.display()))?;
    if cli.format == OutputFormat::Ci && report.has_findings() {
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
