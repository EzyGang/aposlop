use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;
use thiserror::Error;

use crate::detection::{FindingId, ParseFindingIdError};

pub(crate) const ALLOW_LIST_NAME: &str = ".aposlopignore";

#[derive(Debug, Default)]
pub(crate) struct AllowList {
    ids: BTreeSet<FindingId>,
}

pub(crate) struct AllowListUsage<'a> {
    configured: &'a BTreeSet<FindingId>,
    used: BTreeSet<FindingId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AddOutcome {
    Added,
    AlreadyPresent,
}

#[derive(Debug, Error)]
pub(crate) enum AllowListError {
    #[error("failed to read allow list {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid finding ID in {path} at line {line}: {source}")]
    InvalidId {
        path: PathBuf,
        line: usize,
        #[source]
        source: ParseFindingIdError,
    },
    #[error("failed to create a temporary allow list in {path}: {source}")]
    Create {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write a temporary allow list in {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to atomically replace allow list {path}: {source}")]
    Persist {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl AllowList {
    pub(crate) fn load(root: &Path) -> Result<Self, AllowListError> {
        let path = root.join(ALLOW_LIST_NAME);
        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(source) => return Err(AllowListError::Read { path, source }),
        };
        let mut ids = BTreeSet::new();
        for (index, line) in contents.lines().enumerate() {
            let value = line.trim();
            if value.is_empty() || value.starts_with('#') {
                continue;
            }
            let id = value.parse().map_err(|source| AllowListError::InvalidId {
                path: path.clone(),
                line: index + 1,
                source,
            })?;
            ids.insert(id);
        }
        Ok(Self { ids })
    }

    #[must_use]
    pub(crate) fn usage(&self) -> AllowListUsage<'_> {
        AllowListUsage {
            configured: &self.ids,
            used: BTreeSet::new(),
        }
    }

    pub(crate) fn add(root: &Path, id: FindingId) -> Result<AddOutcome, AllowListError> {
        let mut allow_list = Self::load(root)?;
        if !allow_list.ids.insert(id) {
            return Ok(AddOutcome::AlreadyPresent);
        }
        allow_list.write(root)?;
        Ok(AddOutcome::Added)
    }

    fn write(&self, root: &Path) -> Result<(), AllowListError> {
        let path = root.join(ALLOW_LIST_NAME);
        let mut temporary =
            NamedTempFile::new_in(root).map_err(|source| AllowListError::Create {
                path: root.to_path_buf(),
                source,
            })?;
        writeln!(temporary, "# Manually excluded Aposlop findings.")
            .and_then(|()| writeln!(temporary, "# Remove an ID to report that finding again."))
            .and_then(|()| {
                for id in &self.ids {
                    writeln!(temporary, "{id}")?;
                }
                temporary.as_file().sync_all()
            })
            .map_err(|source| AllowListError::Write {
                path: root.to_path_buf(),
                source,
            })?;
        temporary
            .persist(&path)
            .map_err(|error| AllowListError::Persist {
                path,
                source: error.error,
            })?;
        Ok(())
    }
}
impl AllowListUsage<'_> {
    #[must_use]
    pub(crate) fn allows(&mut self, id: FindingId) -> bool {
        if !self.configured.contains(&id) {
            return false;
        }
        self.used.insert(id);
        true
    }

    #[must_use]
    pub(crate) fn unused(self) -> Vec<FindingId> {
        self.configured.difference(&self.used).copied().collect()
    }
}
