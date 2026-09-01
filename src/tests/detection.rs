use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;
use xxhash_rust::xxh3::xxh3_64;

use crate::analysis::{AnalyzedBlock, AnalyzedFile, SourceLocation, analyze};
use crate::config::Config;
use crate::detection::near_miss::{build_signature, jaccard};
use crate::detection::{CloneKind, detect};
use crate::ingest::{FileIdentity, discover};
use crate::language::{LanguageId, LanguageRegistry};
type TestResult<T = ()> = anyhow::Result<T>;

#[test]
fn exact_precedence_emits_each_pair_once() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("a.rs"),
        "fn value(x: i32) -> i32 { x + 1 }\n",
    )?;
    fs::write(
        fixture.path().join("b.rs"),
        "fn value(x: i32) -> i32 { x + 1 }\n",
    )?;
    fs::write(
        fixture.path().join("c.rs"),
        "fn renamed(y: i32) -> i32 { y + 9 }\n",
    )?;
    let config = permissive_config(0.85)?;
    let registry = LanguageRegistry::compile()?;
    let discovery = discover(fixture.path(), &config, &registry)?;
    let files = analyze(discovery.files, &registry)?;

    let matches = detect(&files, &config);
    assert_eq!(
        matches
            .iter()
            .filter(|item| item.kind == CloneKind::Type1)
            .count(),
        1
    );
    assert_eq!(
        matches
            .iter()
            .filter(|item| item.kind == CloneKind::Type2)
            .count(),
        2
    );
    assert_eq!(matches.len(), 3);
    Ok(())
}

#[test]
fn hash_collisions_require_byte_equality() -> TestResult {
    let config = Config::parse(
        "[core]\nmin_lines = 1\nmin_nodes = 1\n[duplicates_detection]\ntype_3 = false",
    )?;
    let mut left = manual_file("left.ts", &[1, 2], 1);
    let mut right = manual_file("right.ts", &[8, 9], 2);
    left.blocks[0].exact_hash = 42;
    right.blocks[0].exact_hash = 42;
    left.blocks[0].normalized_hash = 84;
    right.blocks[0].normalized_hash = 84;

    assert!(detect(&[left, right], &config).is_empty());
    Ok(())
}

#[test]
fn lsh_candidates_are_verified_at_the_jaccard_boundary() -> TestResult {
    let left = manual_file("left.ts", &[1, 2, 3, 4], 1);
    let right = manual_file("right.ts", &[1, 2, 3, 5], 2);

    let included = detect(&[left.clone(), right.clone()], &permissive_config(0.60)?);
    assert_eq!(included.len(), 1);
    assert_eq!(included[0].kind, CloneKind::Type3);
    assert_eq!(included[0].similarity, 0.60);
    assert!(detect(&[left, right], &permissive_config(0.61)?).is_empty());
    Ok(())
}

#[test]
fn insufficient_similarity_and_extension_thresholds_are_excluded() -> TestResult {
    let disjoint_left = manual_file("left.ts", &[1, 2], 1);
    let disjoint_right = manual_file("right.ts", &[8, 9], 2);
    assert!(detect(&[disjoint_left, disjoint_right], &permissive_config(0.1)?).is_empty());

    let ts = manual_file("left.ts", &[1, 2, 3, 4], 3);
    let tsx = manual_file("right.tsx", &[1, 2, 3, 5], 4);
    let config = Config::parse(
        "[core]\nmin_lines = 1\nmin_nodes = 1\n[duplicates_detection]\ntype_3_threshold = 0.5\n[extensions.tsx]\ntype_3_threshold = 0.7",
    )?;
    assert!(detect(&[ts, tsx], &config).is_empty());
    Ok(())
}

#[test]
fn repeated_bands_and_shuffled_input_produce_one_stable_result() -> TestResult {
    let left = manual_file("left.ts", &[1, 2, 3, 4], 1);
    let right = manual_file("right.ts", &[1, 2, 3, 5], 2);
    let config = permissive_config(0.6)?;

    let forward = detect(&[left.clone(), right.clone()], &config);
    let reverse = detect(&[right, left], &config);
    assert_eq!(forward, reverse);
    assert_eq!(forward.len(), 1);
    Ok(())
}

#[test]
fn same_file_blocks_match_without_self_pairs() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("same.py"),
        "def first(value):\n    return value + 1\n\ndef second(item):\n    return item + 9\n",
    )?;
    let config = permissive_config(0.85)?;
    let registry = LanguageRegistry::compile()?;
    let discovery = discover(fixture.path(), &config, &registry)?;
    let files = analyze(discovery.files, &registry)?;

    let matches = detect(&files, &config);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].kind, CloneKind::Type2);
    assert_eq!(matches[0].left.path, matches[0].right.path);
    assert_ne!(matches[0].left.start_line, matches[0].right.start_line);
    Ok(())
}

#[test]
fn minhash_seeds_and_jaccard_are_repeatable() -> TestResult {
    let shingles = [2, 3, 5, 7, 11];
    assert_eq!(build_signature(&shingles), build_signature(&shingles));
    assert_eq!(jaccard(&[1, 2, 3], &[2, 3, 4]), 0.5);
    Ok(())
}

fn permissive_config(threshold: f64) -> TestResult<Config> {
    Ok(Config::parse(&format!(
        "[core]\nmin_lines = 1\nmin_nodes = 1\n[duplicates_detection]\ntype_3_threshold = {threshold}"
    ))?)
}

fn manual_file(path: &str, shingles: &[u64], discriminator: u8) -> AnalyzedFile {
    let exact = vec![b'e', discriminator];
    let normalized = vec![b'n', discriminator];
    AnalyzedFile {
        identity: FileIdentity {
            path: PathBuf::from(path),
            size: 1,
            modified_seconds: 0,
            modified_nanoseconds: 0,
            language: LanguageId::TypeScript,
        },
        blocks: vec![AnalyzedBlock {
            location: SourceLocation {
                path: PathBuf::from(path),
                start_line: 1,
                end_line: 5,
            },
            start_byte: 0,
            end_byte: 10,
            line_count: 5,
            named_node_count: 30,
            exact_hash: xxh3_64(&exact),
            normalized_hash: xxh3_64(&normalized),
            exact,
            normalized,
            token_hashes: Vec::new(),
            shingles: shingles.to_vec(),
            signature: vec![7; 100],
            complexity: 1,
        }],
        diagnostics: Vec::new(),
    }
}
