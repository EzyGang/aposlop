use std::path::PathBuf;

use clap::Parser;

use crate::config::{CliOverrides, Config, ConfigError, RuleOverride};
use crate::{Cli, OutputFormat};
type TestResult = anyhow::Result<()>;

#[test]
fn defaults_match_the_v1_contract() -> TestResult {
    let config = Config::parse("")?;
    let rules = config.rules("rust", "rs");

    assert_eq!(rules.min_lines, 5);
    assert_eq!(rules.min_nodes, 30);
    assert!(rules.type_1);
    assert!(rules.type_2);
    assert!(rules.type_3);
    assert_eq!(rules.type_3_threshold, 0.85);
    assert!(rules.calculate_complexity);
    assert_eq!(rules.complexity_threshold, 15);
    assert!(config.use_cache());
    assert_eq!(config.excludes().len(), 4);
    Ok(())
}

#[test]
fn rules_resolve_global_language_extension_then_cli() -> TestResult {
    let config = Config::parse(
        r#"
[core]
min_lines = 4
min_nodes = 8
[duplicates_detection]
type_2 = true
type_3_threshold = 0.70
[metrics]
complexity_threshold = 20
[languages.typescript]
min_lines = 6
type_2 = false
type_3_threshold = 0.80
[extensions.tsx]
min_lines = 9
min_nodes = 12
type_3_threshold = 0.90
"#,
    )?
    .apply_cli(CliOverrides {
        rules: RuleOverride {
            min_nodes: Some(5),
            type_2: Some(true),
            ..RuleOverride::default()
        },
        ..CliOverrides::default()
    })?;

    assert_eq!(config.rules("typescript", "ts").min_lines, 6);
    let tsx = config.rules("typescript", "tsx");
    assert_eq!(tsx.min_lines, 9);
    assert_eq!(tsx.min_nodes, 5);
    assert!(tsx.type_2);
    assert_eq!(tsx.type_3_threshold, 0.90);
    assert_eq!(config.rules("rust", "rs").min_lines, 4);
    Ok(())
}

#[test]
fn explicit_cli_false_and_excludes_replace_file_values() -> TestResult {
    let config = Config::parse(
        r#"
[core]
exclude = ["generated"]
use_cache = true
[duplicates_detection]
type_1 = true
"#,
    )?
    .apply_cli(CliOverrides {
        rules: RuleOverride {
            type_1: Some(false),
            ..RuleOverride::default()
        },
        exclude: Some(vec![PathBuf::from("build"), PathBuf::from("fixtures")]),
        use_cache: Some(false),
    })?;

    assert!(!config.rules("rust", "rs").type_1);
    assert!(!config.use_cache());
    assert_eq!(
        config.excludes(),
        [PathBuf::from("build"), PathBuf::from("fixtures")]
    );
    Ok(())
}

#[test]
fn rejects_unknown_keys_and_invalid_values() -> TestResult {
    assert!(matches!(
        Config::parse("[languages.go]\nmin_lines = 2"),
        Err(ConfigError::UnknownLanguage(_))
    ));
    assert!(matches!(
        Config::parse("[extensions.jsx]\nmin_lines = 2"),
        Err(ConfigError::UnsupportedExtension(_))
    ));
    assert!(matches!(
        Config::parse("[core]\nmin_lines = 0"),
        Err(ConfigError::ZeroThreshold { .. })
    ));
    assert!(matches!(
        Config::parse("[duplicates_detection]\ntype_3_threshold = 1.1"),
        Err(ConfigError::Similarity)
    ));
    assert!(matches!(
        Config::parse("[core]\nexclude = [\"../outside\"]"),
        Err(ConfigError::InvalidExclude(_))
    ));
    assert!(Config::parse("[core]\nunknown = true").is_err());
    Ok(())
}

#[test]
fn cli_parses_all_independent_overrides() -> TestResult {
    let cli = Cli::try_parse_from([
        "aposlop",
        "project",
        "--format",
        "json",
        "--min-lines",
        "2",
        "--min-nodes",
        "5",
        "--exclude",
        "first",
        "--exclude",
        "second",
        "--use-cache",
        "false",
        "--type-1",
        "true",
        "--type-2",
        "false",
        "--type-3",
        "true",
        "--type-3-threshold",
        "0.9",
        "--calculate-complexity",
        "false",
        "--complexity-threshold",
        "3",
    ])?;

    assert_eq!(cli.format, OutputFormat::Json);
    assert_eq!(cli.path, PathBuf::from("project"));
    assert_eq!(cli.exclude.len(), 2);
    assert_eq!(cli.use_cache, Some(false));
    let overrides = cli.overrides();
    assert_eq!(overrides.rules.min_lines, Some(2));
    assert_eq!(overrides.rules.min_nodes, Some(5));
    assert_eq!(overrides.rules.type_2, Some(false));
    assert_eq!(overrides.rules.type_3_threshold, Some(0.9));
    assert_eq!(overrides.rules.calculate_complexity, Some(false));
    assert_eq!(overrides.rules.complexity_threshold, Some(3));
    Ok(())
}
