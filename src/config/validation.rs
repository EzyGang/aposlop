use std::collections::BTreeMap;
use std::path::{Component, PathBuf};

use super::{ConfigError, RuleOverride};

impl RuleOverride {
    pub(super) fn validate(self) -> Result<(), ConfigError> {
        for (field, value) in [
            ("min_lines", self.min_lines),
            ("min_nodes", self.min_nodes),
            ("complexity_threshold", self.complexity_threshold),
        ] {
            if value == Some(0) {
                return Err(ConfigError::ZeroThreshold { field });
            }
        }

        if self
            .type_3_threshold
            .is_some_and(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
        {
            return Err(ConfigError::Similarity);
        }

        Ok(())
    }
}

pub(super) fn validate_keys(
    values: &BTreeMap<String, RuleOverride>,
    allowed: &[&str],
    language: bool,
) -> Result<(), ConfigError> {
    for key in values.keys() {
        if !allowed.contains(&key.as_str()) {
            return if language {
                Err(ConfigError::UnknownLanguage(key.clone()))
            } else {
                Err(ConfigError::UnsupportedExtension(key.clone()))
            };
        }
    }

    Ok(())
}

pub(super) fn validate_excludes(excludes: &[PathBuf]) -> Result<(), ConfigError> {
    for exclude in excludes {
        let invalid = exclude.is_absolute()
            || exclude.components().any(|part| {
                matches!(
                    part,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            });

        if invalid {
            return Err(ConfigError::InvalidExclude(exclude.clone()));
        }
    }

    Ok(())
}
