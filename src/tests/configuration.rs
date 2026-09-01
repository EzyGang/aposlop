use std::fs;
use std::path::PathBuf;

use crate::config::{Config, ConfigError};

pub(super) fn load_config(text: &str) -> Result<Config, ConfigError> {
    let fixture = tempfile::tempdir().map_err(|source| ConfigError::Read {
        path: PathBuf::from(".aposlop.toml"),
        source,
    })?;
    let path = fixture.path().join(".aposlop.toml");
    fs::write(&path, text).map_err(|source| ConfigError::Read { path, source })?;
    Config::load(fixture.path())
}
