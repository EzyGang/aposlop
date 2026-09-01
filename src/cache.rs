use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use thiserror::Error;

use crate::analysis::AnalyzedFile;
use crate::ingest::SourceFile;

pub(crate) const CACHE_FORMAT_VERSION: u32 = 1;
pub(crate) const ANALYSIS_SCHEMA_VERSION: u32 = 4;
const CACHE_NAME: &str = ".aposlop_cache";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CacheFile {
    pub(crate) format_version: u32,
    pub(crate) entries: Vec<CacheEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CacheEntry {
    pub(crate) analysis_schema_version: u32,
    pub(crate) analyzed: AnalyzedFile,
}

#[derive(Debug)]
pub(crate) struct CacheResolution {
    pub(crate) hits: Vec<AnalyzedFile>,
    pub(crate) misses: Vec<SourceFile>,
    pub(crate) diagnostics: Vec<CacheDiagnostic>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CacheDiagnostic {
    pub(crate) path: PathBuf,
    pub(crate) message: String,
}

#[derive(Debug, Error)]
pub(crate) enum CacheError {
    #[error("failed to encode cache: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("failed to create a temporary cache in {path}: {source}")]
    Create {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write temporary cache in {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to atomically replace cache {path}: {source}")]
    Persist {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Serialize)]
struct CacheFileRef<'a> {
    format_version: u32,
    entries: Vec<CacheEntryRef<'a>>,
}

#[derive(Serialize)]
struct CacheEntryRef<'a> {
    analysis_schema_version: u32,
    analyzed: &'a AnalyzedFile,
}

#[must_use]
pub(crate) fn resolve(root: &Path, enabled: bool, files: Vec<SourceFile>) -> CacheResolution {
    if !enabled {
        return CacheResolution {
            hits: Vec::new(),
            misses: files,
            diagnostics: Vec::new(),
        };
    }
    let path = root.join(CACHE_NAME);
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return cold(files, Vec::new());
        }
        Err(error) => {
            return cold(files, vec![diagnostic(error.to_string())]);
        }
    };
    let decoded =
        bincode::serde::decode_from_slice::<CacheFile, _>(&bytes, bincode::config::standard());
    let (cache, used) = match decoded {
        Ok(value) => value,
        Err(error) => return cold(files, vec![diagnostic(error.to_string())]),
    };
    if used != bytes.len() {
        return cold(
            files,
            vec![diagnostic("cache contains trailing bytes".to_owned())],
        );
    }
    if cache.format_version != CACHE_FORMAT_VERSION {
        return cold(files, Vec::new());
    }
    let mut entries: BTreeMap<_, _> = cache
        .entries
        .into_iter()
        .map(|entry| (entry.analyzed.identity.path.clone(), entry))
        .collect();
    let mut hits = Vec::new();
    let mut misses = Vec::new();
    for source in files {
        let hit = entries.remove(&source.identity.path).filter(|entry| {
            entry.analysis_schema_version == ANALYSIS_SCHEMA_VERSION
                && entry.analyzed.identity == source.identity
        });
        match hit {
            Some(entry) => hits.push(entry.analyzed),
            None => misses.push(source),
        }
    }
    CacheResolution {
        hits,
        misses,
        diagnostics: Vec::new(),
    }
}

pub(crate) fn write(root: &Path, enabled: bool, files: &[AnalyzedFile]) -> Result<(), CacheError> {
    if !enabled {
        return Ok(());
    }
    let mut sorted: Vec<_> = files.iter().collect();
    sorted.sort_unstable_by(|left, right| left.identity.path.cmp(&right.identity.path));
    let cache = CacheFileRef {
        format_version: CACHE_FORMAT_VERSION,
        entries: sorted
            .into_iter()
            .map(|analyzed| CacheEntryRef {
                analysis_schema_version: ANALYSIS_SCHEMA_VERSION,
                analyzed,
            })
            .collect(),
    };
    let bytes = bincode::serde::encode_to_vec(&cache, bincode::config::standard())?;
    let path = root.join(CACHE_NAME);
    let mut temporary = NamedTempFile::new_in(root).map_err(|source| CacheError::Create {
        path: root.to_path_buf(),
        source,
    })?;
    temporary
        .write_all(&bytes)
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(|source| CacheError::Write {
            path: root.to_path_buf(),
            source,
        })?;
    temporary
        .persist(&path)
        .map_err(|error| CacheError::Persist {
            path,
            source: error.error,
        })?;
    Ok(())
}

fn cold(files: Vec<SourceFile>, diagnostics: Vec<CacheDiagnostic>) -> CacheResolution {
    CacheResolution {
        hits: Vec::new(),
        misses: files,
        diagnostics,
    }
}

fn diagnostic(message: String) -> CacheDiagnostic {
    CacheDiagnostic {
        path: PathBuf::from(CACHE_NAME),
        message: format!("cache is unusable and will be replaced: {message}"),
    }
}
