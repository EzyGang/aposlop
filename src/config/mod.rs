mod defaults;
mod validation;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

use self::validation::{validate_excludes, validate_keys};

const LANGUAGE_KEYS: &[&str] = &["rust", "python", "typescript"];
const EXTENSION_KEYS: &[&str] = &["rs", "py", "ts", "tsx"];

#[derive(Clone, Debug)]
pub(crate) struct Config {
    core: CoreConfig,
    duplicates: DuplicateConfig,
    metrics: MetricsConfig,
    languages: BTreeMap<String, RuleOverride>,
    extensions: BTreeMap<String, RuleOverride>,
    cli: CliOverrides,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct CoreConfig {
    pub(crate) min_lines: usize,
    pub(crate) min_nodes: usize,
    pub(crate) exclude: Vec<PathBuf>,
    pub(crate) use_cache: bool,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct DuplicateConfig {
    pub(crate) type_1: bool,
    pub(crate) type_2: bool,
    pub(crate) type_3: bool,
    pub(crate) type_3_threshold: f64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct MetricsConfig {
    pub(crate) calculate_complexity: bool,
    pub(crate) complexity_threshold: usize,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct RuleOverride {
    pub(crate) min_lines: Option<usize>,
    pub(crate) min_nodes: Option<usize>,
    pub(crate) type_1: Option<bool>,
    pub(crate) type_2: Option<bool>,
    pub(crate) type_3: Option<bool>,
    pub(crate) type_3_threshold: Option<f64>,
    pub(crate) calculate_complexity: Option<bool>,
    pub(crate) complexity_threshold: Option<usize>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CliOverrides {
    pub(crate) rules: RuleOverride,
    pub(crate) exclude: Option<Vec<PathBuf>>,
    pub(crate) use_cache: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct EffectiveRules {
    pub(crate) min_lines: usize,
    pub(crate) min_nodes: usize,
    pub(crate) type_1: bool,
    pub(crate) type_2: bool,
    pub(crate) type_3: bool,
    pub(crate) type_3_threshold: f64,
    pub(crate) calculate_complexity: bool,
    pub(crate) complexity_threshold: usize,
}

#[derive(Debug, Error)]
pub(crate) enum ConfigError {
    #[error("failed to read configuration {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse configuration {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("unknown language override `{0}`")]
    UnknownLanguage(String),
    #[error("unsupported extension override `{0}`")]
    UnsupportedExtension(String),
    #[error("{field} must be greater than zero")]
    ZeroThreshold { field: &'static str },
    #[error("type_3_threshold must be a finite value in the range 0.0..=1.0")]
    Similarity,
    #[error("exclude path `{0}` must be relative and cannot contain `..`")]
    InvalidExclude(PathBuf),
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct FileConfig {
    core: CoreConfig,
    duplicates_detection: DuplicateConfig,
    metrics: MetricsConfig,
    languages: BTreeMap<String, RuleOverride>,
    extensions: BTreeMap<String, RuleOverride>,
}

impl Config {
    pub(crate) fn load(root: &Path) -> Result<Self, ConfigError> {
        let path = root.join(".aposlop.toml");
        if !path.exists() {
            return Self::from_file(FileConfig::default());
        }

        let text = fs::read_to_string(&path).map_err(|source| ConfigError::Read {
            path: path.clone(),
            source,
        })?;
        let parsed = toml::from_str(&text).map_err(|source| ConfigError::Parse { path, source })?;
        Self::from_file(parsed)
    }

    #[cfg(test)]
    pub(crate) fn parse(text: &str) -> Result<Self, ConfigError> {
        let path = PathBuf::from(".aposlop.toml");
        let parsed = toml::from_str(text).map_err(|source| ConfigError::Parse { path, source })?;
        Self::from_file(parsed)
    }

    pub(crate) fn apply_cli(mut self, cli: CliOverrides) -> Result<Self, ConfigError> {
        cli.rules.validate()?;
        if let Some(excludes) = &cli.exclude {
            validate_excludes(excludes)?;
        }

        self.cli = cli;
        Ok(self)
    }

    #[must_use]
    pub(crate) fn rules(&self, language: &str, extension: &str) -> EffectiveRules {
        let mut rules = EffectiveRules {
            min_lines: self.core.min_lines,
            min_nodes: self.core.min_nodes,
            type_1: self.duplicates.type_1,
            type_2: self.duplicates.type_2,
            type_3: self.duplicates.type_3,
            type_3_threshold: self.duplicates.type_3_threshold,
            calculate_complexity: self.metrics.calculate_complexity,
            complexity_threshold: self.metrics.complexity_threshold,
        };

        if let Some(values) = self.languages.get(language) {
            values.apply(&mut rules);
        }
        if let Some(values) = self.extensions.get(extension) {
            values.apply(&mut rules);
        }
        self.cli.rules.apply(&mut rules);
        rules
    }

    #[must_use]
    pub(crate) fn excludes(&self) -> &[PathBuf] {
        self.cli.exclude.as_deref().unwrap_or(&self.core.exclude)
    }

    #[must_use]
    pub(crate) fn use_cache(&self) -> bool {
        self.cli.use_cache.unwrap_or(self.core.use_cache)
    }

    fn from_file(file: FileConfig) -> Result<Self, ConfigError> {
        validate_keys(&file.languages, LANGUAGE_KEYS, true)?;
        validate_keys(&file.extensions, EXTENSION_KEYS, false)?;
        validate_excludes(&file.core.exclude)?;

        let global = RuleOverride {
            min_lines: Some(file.core.min_lines),
            min_nodes: Some(file.core.min_nodes),
            type_1: Some(file.duplicates_detection.type_1),
            type_2: Some(file.duplicates_detection.type_2),
            type_3: Some(file.duplicates_detection.type_3),
            type_3_threshold: Some(file.duplicates_detection.type_3_threshold),
            calculate_complexity: Some(file.metrics.calculate_complexity),
            complexity_threshold: Some(file.metrics.complexity_threshold),
        };
        global.validate()?;

        for values in file.languages.values().chain(file.extensions.values()) {
            values.validate()?;
        }

        Ok(Self {
            core: file.core,
            duplicates: file.duplicates_detection,
            metrics: file.metrics,
            languages: file.languages,
            extensions: file.extensions,
            cli: CliOverrides::default(),
        })
    }
}

impl RuleOverride {
    fn apply(self, rules: &mut EffectiveRules) {
        macro_rules! apply {
            ($field:ident) => {
                if let Some(value) = self.$field {
                    rules.$field = value;
                }
            };
        }

        apply!(min_lines);
        apply!(min_nodes);
        apply!(type_1);
        apply!(type_2);
        apply!(type_3);
        apply!(type_3_threshold);
        apply!(calculate_complexity);
        apply!(complexity_threshold);
    }
}
