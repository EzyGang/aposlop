use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;

use crate::analysis::AnalyzedFile;
use crate::cache::{
    ANALYSIS_SCHEMA_VERSION, CACHE_FORMAT_VERSION, CacheFile, CacheHeader, resolve, write,
};
use crate::ingest::{FileIdentity, SourceFile};
use crate::language::LanguageId;
type TestResult<T = ()> = anyhow::Result<T>;

#[test]
fn equal_identity_hits_and_changed_identity_misses() -> TestResult {
    let fixture = TempDir::new()?;
    let source = source("code.rs", LanguageId::Rust);
    write(fixture.path(), true, &[analyzed(&source)])?;

    let hit = resolve(fixture.path(), true, vec![source.clone()]);
    assert_eq!(hit.hits.len(), 1);
    assert!(hit.misses.is_empty());

    for changed in [
        SourceFile {
            identity: FileIdentity {
                size: 11,
                ..source.identity.clone()
            },
            ..source.clone()
        },
        SourceFile {
            identity: FileIdentity {
                modified_seconds: 8,
                ..source.identity.clone()
            },
            ..source.clone()
        },
        SourceFile {
            identity: FileIdentity {
                language: LanguageId::Python,
                ..source.identity.clone()
            },
            ..source.clone()
        },
    ] {
        let resolution = resolve(fixture.path(), true, vec![changed]);
        assert!(resolution.hits.is_empty());
        assert_eq!(resolution.misses.len(), 1);
    }
    Ok(())
}

#[test]
fn missing_and_incompatible_caches_miss() -> TestResult {
    let fixture = TempDir::new()?;
    let source = source("code.rs", LanguageId::Rust);
    assert_eq!(
        resolve(fixture.path(), true, vec![source.clone()])
            .misses
            .len(),
        1
    );

    for header in [
        CacheHeader {
            format_version: CACHE_FORMAT_VERSION + 1,
            analysis_schema_version: ANALYSIS_SCHEMA_VERSION,
        },
        CacheHeader {
            format_version: CACHE_FORMAT_VERSION,
            analysis_schema_version: ANALYSIS_SCHEMA_VERSION + 1,
        },
    ] {
        encode(
            &fixture,
            CacheFile {
                header,
                entries: vec![analyzed(&source)],
            },
        )?;
        let resolution = resolve(fixture.path(), true, vec![source.clone()]);
        assert_eq!(resolution.misses.len(), 1);
        assert!(resolution.diagnostics.is_empty());
    }
    Ok(())
}

#[test]
fn other_format_versions_miss_before_their_entries_decode() -> TestResult {
    let fixture = TempDir::new()?;
    let source = source("code.rs", LanguageId::Rust);
    let older_format = u8::try_from(CACHE_FORMAT_VERSION)? - 1;
    let mut bytes = vec![older_format, 1, 1, 0xFD];
    bytes.extend_from_slice(&[0xFF; 8]);
    fs::write(fixture.path().join(".aposlop_cache"), bytes)?;

    let resolution = resolve(fixture.path(), true, vec![source]);
    assert_eq!(resolution.misses.len(), 1);
    assert!(resolution.diagnostics.is_empty());
    Ok(())
}

#[test]
fn corrupt_cache_is_diagnostic_and_is_replaced() -> TestResult {
    let fixture = TempDir::new()?;
    let source = source("code.rs", LanguageId::Rust);
    let path = fixture.path().join(".aposlop_cache");
    write(fixture.path(), true, &[analyzed(&source)])?;
    let mut bytes = fs::read(&path)?;
    bytes.truncate(bytes.len() - 1);
    fs::write(&path, bytes)?;

    let resolution = resolve(fixture.path(), true, vec![source.clone()]);
    assert_eq!(resolution.diagnostics.len(), 1);
    assert_eq!(resolution.misses.len(), 1);
    write(fixture.path(), true, &[analyzed(&source)])?;

    let recovered = resolve(fixture.path(), true, vec![source]);
    assert_eq!(recovered.hits.len(), 1);
    assert!(recovered.diagnostics.is_empty());
    Ok(())
}

#[test]
fn disabled_cache_does_not_read_or_write() -> TestResult {
    let fixture = TempDir::new()?;
    let path = fixture.path().join(".aposlop_cache");
    fs::write(&path, b"unchanged")?;
    let source = source("code.rs", LanguageId::Rust);

    let resolution = resolve(fixture.path(), false, vec![source.clone()]);
    assert_eq!(resolution.misses.len(), 1);
    assert!(resolution.diagnostics.is_empty());
    write(fixture.path(), false, &[analyzed(&source)])?;
    assert_eq!(fs::read(path)?, b"unchanged");
    Ok(())
}

#[test]
fn serialization_order_and_atomic_replacement_are_deterministic() -> TestResult {
    let fixture = TempDir::new()?;
    let first = analyzed(&source("a.rs", LanguageId::Rust));
    let second = analyzed(&source("z.py", LanguageId::Python));

    write(fixture.path(), true, &[second.clone(), first.clone()])?;
    let forward = fs::read(fixture.path().join(".aposlop_cache"))?;
    write(fixture.path(), true, &[first.clone(), second.clone()])?;
    let reverse = fs::read(fixture.path().join(".aposlop_cache"))?;
    assert_eq!(forward, reverse);

    let resolved = resolve(
        fixture.path(),
        true,
        vec![
            SourceFile {
                read_path: PathBuf::from("unused"),
                identity: first.identity,
            },
            SourceFile {
                read_path: PathBuf::from("unused"),
                identity: second.identity,
            },
        ],
    );
    assert_eq!(resolved.hits.len(), 2);
    Ok(())
}

fn source(path: &str, language: LanguageId) -> SourceFile {
    SourceFile {
        read_path: PathBuf::from("unused"),
        identity: FileIdentity {
            path: PathBuf::from(path),
            size: 10,
            modified_seconds: 7,
            modified_nanoseconds: 9,
            language,
        },
    }
}

fn analyzed(source: &SourceFile) -> AnalyzedFile {
    AnalyzedFile {
        identity: source.identity.clone(),
        line_count: 1,
        blocks: Vec::new(),
        diagnostics: Vec::new(),
    }
}

fn encode(fixture: &TempDir, cache: CacheFile) -> TestResult {
    let bytes = bincode::serde::encode_to_vec(cache, bincode::config::standard())?;
    fs::write(fixture.path().join(".aposlop_cache"), bytes)?;
    Ok(())
}
