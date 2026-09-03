use std::path::{Path, PathBuf};

use clap::Parser;

use crate::config::{CliOverrides, ConfigError, RuleOverride};
use crate::{Cli, OutputFormat};

use super::configuration::load_config;
type TestResult = anyhow::Result<()>;

#[test]
fn defaults_match_the_v1_contract() -> TestResult {
    let config = load_config("")?;
    let rules = config.rules("rust", "rs");

    assert_eq!(rules.min_lines, 5);
    assert_eq!(rules.min_nodes, 30);
    assert!(rules.type_1);
    assert!(rules.type_2);
    assert!(rules.type_3);
    assert_eq!(rules.type_3_threshold, 0.85);
    assert!(rules.calculate_complexity);
    assert_eq!(rules.complexity_threshold, 15);
    assert_eq!(rules.max_file_lines, 300);
    assert!(config.use_cache());
    assert!(config.is_excluded(Path::new("crate/tests"), true));
    assert!(config.is_excluded(Path::new("packages/app/node_modules"), true));
    assert!(!config.is_excluded(Path::new("crate/contest"), true));
    assert!(!config.is_file_length_excluded(Path::new("src/large.rs")));
    Ok(())
}

#[test]
fn rules_resolve_global_language_extension_then_cli() -> TestResult {
    let config = load_config(
        r#"
    [core]
    min_lines = 4
    min_nodes = 8
    [duplicates_detection]
    type_2 = true
    type_3_threshold = 0.70
    [metrics]
    complexity_threshold = 20
    [languages.go]
    min_lines = 7
    [extensions.go]
    min_lines = 10
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

    assert_eq!(config.rules("go", "go").min_lines, 10);
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
fn file_length_rules_resolve_global_language_extension_then_cli() -> TestResult {
    let config = load_config(
        r#"
    [file_length]
    max_lines = 400
    exclude = ["generated/"]
    [languages.python]
    max_file_lines = 500
    [extensions.py]
    max_file_lines = 600
    "#,
    )?;

    assert_eq!(config.rules("rust", "rs").max_file_lines, 400);
    assert_eq!(config.rules("python", "py").max_file_lines, 600);
    assert!(config.is_file_length_excluded(Path::new("src/generated/large.py")));
    assert!(!config.is_excluded(Path::new("src/generated/large.py"), false));

    let config = config.apply_cli(CliOverrides {
        rules: RuleOverride {
            max_file_lines: Some(700),
            ..RuleOverride::default()
        },
        ..CliOverrides::default()
    })?;
    assert_eq!(config.rules("python", "py").max_file_lines, 700);
    Ok(())
}

#[test]
fn explicit_cli_false_and_excludes_replace_file_values() -> TestResult {
    let config = load_config(
        r#"
    [core]
    exclude = ["generated/"]
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
        exclude: Some(vec!["build/".to_owned(), "**/fixtures/**".to_owned()]),
        use_cache: Some(false),
    })?;

    assert!(!config.rules("rust", "rs").type_1);
    assert!(!config.use_cache());
    assert!(config.is_excluded(Path::new("build"), true));
    assert!(config.is_excluded(Path::new("packages/app/fixtures/data.py"), false));
    assert!(!config.is_excluded(Path::new("generated"), true));
    Ok(())
}

#[test]
fn rejects_unknown_keys_and_invalid_values() -> TestResult {
    assert!(matches!(
        load_config("[languages.ruby]\nmin_lines = 2"),
        Err(ConfigError::UnknownLanguage(_))
    ));
    assert!(matches!(
        load_config("[extensions.jsx]\nmin_lines = 2"),
        Err(ConfigError::UnsupportedExtension(_))
    ));
    assert!(matches!(
        load_config("[core]\nmin_lines = 0"),
        Err(ConfigError::ZeroThreshold { .. })
    ));
    assert!(matches!(
        load_config("[duplicates_detection]\ntype_3_threshold = 1.1"),
        Err(ConfigError::Similarity)
    ));
    assert!(matches!(
        load_config("[file_length]\nmax_lines = 0"),
        Err(ConfigError::ZeroThreshold { .. })
    ));
    assert!(load_config("[core]\nunknown = true").is_err());
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
        "--max-file-lines",
        "400",
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
    assert_eq!(overrides.rules.max_file_lines, Some(400));
    Ok(())
}
