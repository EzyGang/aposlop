use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

use crate::config::{CliOverrides, RuleOverride};
use crate::detection::FindingId;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub(crate) enum OutputFormat {
    #[default]
    Terminal,
    Json,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub(crate) enum TerminalOutput {
    #[default]
    Locations,
    Code,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Save a finding ID in the target's manual exclusions file.
    Allow {
        /// Duplicate or complexity finding ID to allow.
        finding: FindingId,

        /// Directory that owns the .aposlopignore file.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Print a concise finding summary and fail when findings exist.
    Ci {
        /// Directory to analyze.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Install bundled agent skills with npx or pnpm.
    InstallSkills,
}

#[derive(Debug, Parser)]
#[command(
    name = "aposlop",
    version,
    about = "Detect duplicate code, excessive file length, and cyclomatic complexity"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Command>,

    /// Directory to analyze.
    #[arg(default_value = ".")]
    pub(crate) path: PathBuf,

    /// Select the output format.
    #[arg(long, value_enum, default_value_t)]
    pub(crate) format: OutputFormat,
    /// Select location-only or source-code terminal findings.
    #[arg(long, value_enum, default_value_t)]
    pub(crate) terminal_output: TerminalOutput,

    /// Override the minimum block line count.
    #[arg(long, value_name = "N", global = true)]
    min_lines: Option<usize>,
    /// Override the minimum named-node count.
    #[arg(long, value_name = "N", global = true)]
    min_nodes: Option<usize>,

    /// Replace configured gitignore-style exclusion patterns.
    #[arg(long, value_name = "GLOB", global = true)]
    pub(crate) exclude: Vec<String>,

    /// Enable or disable the analysis cache.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set, global = true)]
    pub(crate) use_cache: Option<bool>,

    /// Enable or disable Type-1 duplicate reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set, global = true)]
    type_1: Option<bool>,
    /// Enable or disable Type-2 duplicate reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set, global = true)]
    type_2: Option<bool>,
    /// Enable or disable Type-3 duplicate reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set, global = true)]
    type_3: Option<bool>,

    /// Override the Type-3 Jaccard similarity threshold.
    #[arg(long, value_name = "RATIO", global = true)]
    type_3_threshold: Option<f64>,

    /// Enable or disable complexity reporting.
    #[arg(long, value_name = "BOOL", action = ArgAction::Set, global = true)]
    calculate_complexity: Option<bool>,

    /// Override the complexity violation threshold.
    #[arg(long, value_name = "N", global = true)]
    complexity_threshold: Option<usize>,

    /// Override the maximum source-file line count.
    #[arg(long, value_name = "N", global = true)]
    max_file_lines: Option<usize>,
}

impl Cli {
    pub(crate) fn overrides(&self) -> CliOverrides {
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
                max_file_lines: self.max_file_lines,
            },
            exclude: (!self.exclude.is_empty()).then(|| self.exclude.clone()),
            use_cache: self.use_cache,
        }
    }
}
