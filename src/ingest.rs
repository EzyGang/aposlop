use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

use ignore::{DirEntry, WalkBuilder, WalkState};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::Config;
use crate::language::{LanguageId, LanguageRegistry};

#[derive(Clone, Debug)]
pub(crate) struct SourceFile {
    pub(crate) read_path: PathBuf,
    pub(crate) identity: FileIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct FileIdentity {
    pub(crate) path: PathBuf,
    pub(crate) size: u64,
    pub(crate) modified_seconds: u64,
    pub(crate) modified_nanoseconds: u32,
    pub(crate) language: LanguageId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IngestDiagnostic {
    pub(crate) path: PathBuf,
    pub(crate) message: String,
}

#[derive(Debug)]
pub(crate) struct Discovery {
    pub(crate) files: Vec<SourceFile>,
    pub(crate) diagnostics: Vec<IngestDiagnostic>,
}

#[derive(Debug, Error)]
pub(crate) enum IngestError {
    #[error("failed to resolve target root {path}: {source}")]
    Root {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl FileIdentity {
    #[must_use]
    pub(crate) fn extension(&self) -> &str {
        self.path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
    }
}

#[derive(Default)]
struct Collected {
    files: Vec<SourceFile>,
    diagnostics: Vec<IngestDiagnostic>,
}

struct Worker<'a> {
    root: &'a Path,
    config: &'a Config,
    registry: &'a LanguageRegistry,
    shared: Arc<Mutex<Collected>>,
    local: Collected,
}

pub(crate) fn discover(
    root: &Path,
    config: &Config,
    registry: &LanguageRegistry,
) -> Result<Discovery, IngestError> {
    let root = root.canonicalize().map_err(|source| IngestError::Root {
        path: root.to_path_buf(),
        source,
    })?;
    let shared = Arc::new(Mutex::new(Collected::default()));
    let mut builder = WalkBuilder::new(&root);
    builder
        .standard_filters(true)
        .hidden(true)
        .parents(true)
        .ignore(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true);
    builder.build_parallel().run(|| {
        let mut worker = Worker {
            root: &root,
            config,
            registry,
            shared: Arc::clone(&shared),
            local: Collected::default(),
        };
        Box::new(move |result| worker.visit(result))
    });

    let mut collected = match Arc::try_unwrap(shared) {
        Ok(mutex) => mutex
            .into_inner()
            .unwrap_or_else(|error| error.into_inner()),
        Err(shared) => {
            let mut guard = shared.lock().unwrap_or_else(|error| error.into_inner());
            std::mem::take(&mut *guard)
        }
    };
    collected
        .files
        .sort_unstable_by(|left, right| left.identity.path.cmp(&right.identity.path));
    collected.diagnostics.sort_unstable_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.message.cmp(&right.message))
    });
    Ok(Discovery {
        files: collected.files,
        diagnostics: collected.diagnostics,
    })
}

impl Worker<'_> {
    fn visit(&mut self, result: Result<DirEntry, ignore::Error>) -> WalkState {
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => {
                self.local.diagnostics.push(IngestDiagnostic {
                    path: PathBuf::from("."),
                    message: error.to_string(),
                });
                return WalkState::Continue;
            }
        };
        let relative = match entry.path().strip_prefix(self.root) {
            Ok(path) => path,
            Err(_) => return WalkState::Continue,
        };
        if relative.as_os_str().is_empty() {
            return WalkState::Continue;
        }
        let file_type = entry.file_type();
        let is_dir = file_type.is_some_and(|kind| kind.is_dir());
        if self.is_excluded(relative, is_dir) {
            return if is_dir {
                WalkState::Skip
            } else {
                WalkState::Continue
            };
        }
        if !file_type.is_some_and(|kind| kind.is_file()) {
            return WalkState::Continue;
        }
        self.add_file(&entry, relative);
        WalkState::Continue
    }

    fn is_excluded(&self, relative: &Path, is_dir: bool) -> bool {
        self.config.is_excluded(relative, is_dir)
    }

    fn add_file(&mut self, entry: &DirEntry, relative: &Path) {
        let Some(extension) = relative.extension().and_then(|value| value.to_str()) else {
            return;
        };
        let Some(language) = self.registry.get(extension).map(|provider| provider.id) else {
            return;
        };
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                self.push_metadata_error(relative, error.to_string());
                return;
            }
        };
        let modified = match metadata.modified().and_then(|time| {
            time.duration_since(UNIX_EPOCH)
                .map_err(std::io::Error::other)
        }) {
            Ok(modified) => modified,
            Err(error) => {
                self.push_metadata_error(relative, error.to_string());
                return;
            }
        };
        self.local.files.push(SourceFile {
            read_path: entry.path().to_path_buf(),
            identity: FileIdentity {
                path: relative.to_path_buf(),
                size: metadata.len(),
                modified_seconds: modified.as_secs(),
                modified_nanoseconds: modified.subsec_nanos(),
                language,
            },
        });
    }

    fn push_metadata_error(&mut self, path: &Path, message: String) {
        self.local.diagnostics.push(IngestDiagnostic {
            path: path.to_path_buf(),
            message: format!("failed to read file metadata: {message}"),
        });
    }
}

impl Drop for Worker<'_> {
    fn drop(&mut self) {
        if self.local.files.is_empty() && self.local.diagnostics.is_empty() {
            return;
        }
        let mut shared = self
            .shared
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        shared.files.append(&mut self.local.files);
        shared.diagnostics.append(&mut self.local.diagnostics);
    }
}
