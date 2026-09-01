mod analysis;
mod cache;
mod config;
mod detection;
mod ingest;
mod language;
mod report;

#[cfg(test)]
mod tests;

use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, bail};
use clap::{ArgAction, Parser, ValueEnum};

use crate::config::{CliOverrides, Config, RuleOverride};
use crate::language::LanguageRegistry;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    #[default]
    Terminal,
    Json,
}

#[derive(Debug, Parser)]
#[command(
    name = "aposlop",
    version,
    about = "Detect duplicate code and report cyclomatic complexity"
)]
struct Cli {
    /// Directory to analyze.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Select the output format.
    #[arg(long, value_enum, default_value_t)]
    format: OutputFormat,

    /// Override the minimum block line count.
    #[arg(long, value_name = "N")]
    min_lines: Option<usize>,

    /// Override the minimum named-node count.
    #[arg(long, value_name = "N")]
    min_nodes: Option<usize>,

    /// Replace configured root-relative exclusions.
    #[arg(long, value_name = "PATH")]
    exclude: Vec<PathBuf>,

    /// Enable or disable the analysis cache.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    use_cache: Option<bool>,

    /// Enable or disable Type-1 clone reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    type_1: Option<bool>,

    /// Enable or disable Type-2 clone reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set)]
    type_2: Option<bool>,

    /// Enable or disable Type-3 clone reporting.
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
    let mut stdout = io::stdout().lock();
    match run(Cli::parse(), &mut stdout) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli, writer: &mut impl io::Write) -> anyhow::Result<()> {
    let metadata = std::fs::metadata(&cli.path)
        .with_context(|| format!("failed to inspect target {}", cli.path.display()))?;
    if !metadata.is_dir() {
        bail!("target {} is not a directory", cli.path.display());
    }
    let config = Config::load(&cli.path)
        .with_context(|| format!("failed to configure target {}", cli.path.display()))?
        .apply_cli(cli.overrides())
        .with_context(|| format!("failed to configure target {}", cli.path.display()))?;
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
        discovery.diagnostics,
        resolution.diagnostics,
    );
    report::render(writer, &report, cli.format)
        .with_context(|| format!("failed to render report for {}", cli.path.display()))?;
    cache::write(&cli.path, config.use_cache(), &files)
        .with_context(|| format!("failed to persist cache for {}", cli.path.display()))?;
    Ok(())
}
