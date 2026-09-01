use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use semver::Version;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use thiserror::Error;

const RELEASE_URL: &str = "https://api.github.com/repos/EzyGang/aposlop/releases/latest";
const RELEASE_PAGE: &str = "https://github.com/EzyGang/aposlop/releases/latest";
const CACHE_NAME: &str = "update.json";
const CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);
const REQUEST_TIMEOUT_SECONDS: u64 = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AvailableUpdate {
    pub(crate) version: Version,
    pub(crate) url: String,
}

#[derive(Debug, Error)]
pub(crate) enum UpdateError {
    #[error("failed to determine the current time: {0}")]
    Clock(#[from] std::time::SystemTimeError),
    #[error("failed to parse version {version}: {source}")]
    Version {
        version: String,
        #[source]
        source: semver::Error,
    },
    #[error("failed to read update cache {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to request the latest release: {0}")]
    Request(#[from] minreq::Error),
    #[error("failed to decode update data: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed to create update cache directory {path}: {source}")]
    CreateDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to create temporary update cache in {path}: {source}")]
    Create {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write temporary update cache in {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to replace update cache {path}: {source}")]
    Persist {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct CacheEntry {
    checked_at: u64,
    tag_name: String,
    html_url: String,
}

#[derive(Debug, Deserialize)]
struct ReleaseResponse {
    tag_name: String,
    html_url: String,
}

pub(crate) fn check() -> Result<Option<AvailableUpdate>, UpdateError> {
    let current = parse_version(env!("CARGO_PKG_VERSION"))?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let Some(path) = cache_path() else {
        return fetch_update(&current).map(|(update, _)| update);
    };

    match read_fresh_cache(&path, now)? {
        Some(entry) => available_update(&current, &entry.tag_name, entry.html_url),
        None => {
            let (update, entry) = fetch_update(&current)?;
            write_cache(
                &path,
                &CacheEntry {
                    checked_at: now,
                    ..entry
                },
            )?;
            Ok(update)
        }
    }
}

#[cfg(test)]
#[must_use]
pub(crate) fn is_newer(current: &str, latest: &str) -> bool {
    let Ok(current) = Version::parse(current.trim_start_matches('v')) else {
        return false;
    };
    let Ok(latest) = Version::parse(latest.trim_start_matches('v')) else {
        return false;
    };
    latest > current
}

fn fetch_update(current: &Version) -> Result<(Option<AvailableUpdate>, CacheEntry), UpdateError> {
    let response = minreq::get(RELEASE_URL)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("User-Agent", "aposlop-update-check")
        .with_timeout(REQUEST_TIMEOUT_SECONDS)
        .send()?;
    let release: ReleaseResponse = serde_json::from_slice(response.as_bytes())?;
    let update = available_update(current, &release.tag_name, release.html_url.clone())?;
    let entry = CacheEntry {
        checked_at: 0,
        tag_name: release.tag_name,
        html_url: release.html_url,
    };
    Ok((update, entry))
}

fn available_update(
    current: &Version,
    latest: &str,
    url: String,
) -> Result<Option<AvailableUpdate>, UpdateError> {
    let latest = parse_version(latest)?;
    Ok((latest > *current).then_some(AvailableUpdate {
        version: latest,
        url: if url.is_empty() {
            RELEASE_PAGE.to_owned()
        } else {
            url
        },
    }))
}

fn parse_version(value: &str) -> Result<Version, UpdateError> {
    Version::parse(value.trim_start_matches('v')).map_err(|source| UpdateError::Version {
        version: value.to_owned(),
        source,
    })
}

fn read_fresh_cache(path: &Path, now: u64) -> Result<Option<CacheEntry>, UpdateError> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(UpdateError::Read {
                path: path.to_path_buf(),
                source,
            });
        }
    };
    let Ok(entry) = serde_json::from_slice::<CacheEntry>(&bytes) else {
        return Ok(None);
    };
    Ok((now.saturating_sub(entry.checked_at) < CHECK_INTERVAL.as_secs()).then_some(entry))
}

fn write_cache(path: &Path, entry: &CacheEntry) -> Result<(), UpdateError> {
    let directory = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(directory).map_err(|source| UpdateError::CreateDirectory {
        path: directory.to_path_buf(),
        source,
    })?;
    let mut temporary = NamedTempFile::new_in(directory).map_err(|source| UpdateError::Create {
        path: directory.to_path_buf(),
        source,
    })?;
    let bytes = serde_json::to_vec(entry)?;
    temporary
        .write_all(&bytes)
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(|source| UpdateError::Write {
            path: directory.to_path_buf(),
            source,
        })?;
    temporary
        .persist(path)
        .map_err(|error| UpdateError::Persist {
            path: path.to_path_buf(),
            source: error.error,
        })?;
    Ok(())
}

fn cache_path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("APOSLOP_UPDATE_CACHE_DIR") {
        return Some(PathBuf::from(path).join(CACHE_NAME));
    }
    platform_cache_root().map(|root| root.join("aposlop").join(CACHE_NAME))
}

#[cfg(windows)]
fn platform_cache_root() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
}

#[cfg(not(windows))]
fn platform_cache_root() -> Option<PathBuf> {
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
}
