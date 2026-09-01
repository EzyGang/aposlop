use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;
use xxhash_rust::xxh3::xxh3_64;

use crate::analysis::{AnalyzedBlock, AnalyzedFile, SourceLocation, analyze};
use crate::config::Config;
use crate::detection::shingles::{build_shingles, jaccard};
use crate::detection::{CloneKind, detect};
use crate::ingest::{FileIdentity, discover};
use crate::language::{LanguageId, LanguageRegistry};

use super::configuration::load_config;
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
    let config = load_config(
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
fn exact_candidates_are_verified_at_the_jaccard_boundary() -> TestResult {
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
fn zero_threshold_includes_disjoint_shingle_sets() -> TestResult {
    let left = manual_file("left.ts", &[1, 2], 1);
    let right = manual_file("right.ts", &[8, 9], 2);

    let matches = detect(&[left, right], &permissive_config(0.0)?);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].similarity, 0.0);
    Ok(())
}

#[test]
fn insufficient_similarity_and_extension_thresholds_are_excluded() -> TestResult {
    let disjoint_left = manual_file("left.ts", &[1, 2], 1);
    let disjoint_right = manual_file("right.ts", &[8, 9], 2);
    assert!(detect(&[disjoint_left, disjoint_right], &permissive_config(0.1)?).is_empty());

    let ts = manual_file("left.ts", &[1, 2, 3, 4], 3);
    let tsx = manual_file("right.tsx", &[1, 2, 3, 5], 4);
    let config = load_config(
        "[core]\nmin_lines = 1\nmin_nodes = 1\n[duplicates_detection]\ntype_3_threshold = 0.5\n[extensions.tsx]\ntype_3_threshold = 0.7",
    )?;
    assert!(detect(&[ts, tsx], &config).is_empty());
    let ts = manual_file("included.ts", &[1, 2, 3, 4], 5);
    let tsx = manual_file("included.tsx", &[1, 2, 3, 5], 6);
    let config = load_config(
        "[core]\nmin_lines = 1\nmin_nodes = 1\n[duplicates_detection]\ntype_3_threshold = 0.5\n[extensions.tsx]\ntype_3_threshold = 0.6",
    )?;
    assert_eq!(detect(&[ts, tsx], &config).len(), 1);
    Ok(())
}

#[test]
fn duplicate_prefix_hits_and_shuffled_input_produce_one_stable_result() -> TestResult {
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
fn shingles_and_jaccard_are_repeatable() -> TestResult {
    let tokens = [2, 3, 5, 7, 11, 13];
    assert_eq!(build_shingles(&tokens), build_shingles(&tokens));
    assert_eq!(jaccard(&[1, 2, 3], &[2, 3, 4]), 0.5);
    Ok(())
}

#[test]
fn exact_join_recovers_the_formerly_missed_operator_change() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(
        fixture.path().join("operators.py"),
        concat!(
            "def find_max_price(prices):\n",
            "    most_expensive = 0\n",
            "    for price in prices:\n",
            "        if price > most_expensive:\n",
            "            most_expensive = price\n",
            "    return most_expensive\n",
            "\n",
            "def find_min_age(ages):\n",
            "    youngest = 999\n",
            "    for age in ages:\n",
            "        if age < youngest:\n",
            "            youngest = age\n",
            "    return youngest\n",
        ),
    )?;
    let config = permissive_config(0.55)?;
    let registry = LanguageRegistry::compile()?;
    let discovery = discover(fixture.path(), &config, &registry)?;
    let files = analyze(discovery.files, &registry)?;
    let exact = detect(&files, &config);

    assert_eq!(exact.len(), 1);
    assert_eq!(exact[0].kind, CloneKind::Type3);
    assert!(exact[0].similarity >= 0.55);
    Ok(())
}

#[test]
fn exact_join_matches_brute_force_at_supported_thresholds() -> TestResult {
    let files = oracle_files(240, 64);

    for threshold in [0.55, 0.70, 0.85, 0.90] {
        let exact = detect(&files, &permissive_config(threshold)?);
        let oracle_matches = brute_force_match_count(&files, threshold);
        assert_eq!(exact.len(), oracle_matches, "threshold {threshold}");
    }

    Ok(())
}

fn oracle_files(block_count: usize, shingle_count: usize) -> Vec<AnalyzedFile> {
    let mut blocks = Vec::with_capacity(block_count);

    for index in 0..block_count {
        let group = index / 20;
        let variant = index % 20;
        let mut shingles: Vec<_> = (0..shingle_count)
            .map(|token| (group * 10_000 + token) as u64)
            .collect();
        for mutation in 0..variant.min(shingle_count / 3) {
            shingles[shingle_count - mutation - 1] =
                1_000_000_000 + (index * shingle_count + mutation) as u64;
        }
        shingles.sort_unstable();

        let exact = index.to_le_bytes().to_vec();
        let mut normalized = b"normalized".to_vec();
        normalized.extend_from_slice(&index.to_le_bytes());
        blocks.push(AnalyzedBlock {
            location: SourceLocation {
                path: PathBuf::from("benchmark.ts"),
                start_line: index * 10 + 1,
                end_line: index * 10 + 5,
            },
            start_byte: index * 100,
            end_byte: index * 100 + 99,
            line_count: 5,
            named_node_count: 100,
            exact_hash: xxh3_64(&exact),
            normalized_hash: xxh3_64(&normalized),
            exact,
            normalized,
            shingles,
            complexity: 1,
        });
    }

    vec![AnalyzedFile {
        identity: FileIdentity {
            path: PathBuf::from("benchmark.ts"),
            size: 1,
            modified_seconds: 0,
            modified_nanoseconds: 0,
            language: LanguageId::TypeScript,
        },
        blocks,
        diagnostics: Vec::new(),
    }]
}

fn permissive_config(threshold: f64) -> TestResult<Config> {
    Ok(load_config(&format!(
        "[core]\nmin_lines = 1\nmin_nodes = 1\n[duplicates_detection]\ntype_3_threshold = {threshold}"
    ))?)
}

fn brute_force_match_count(files: &[AnalyzedFile], threshold: f64) -> usize {
    let blocks = &files[0].blocks;
    let mut count = 0;

    for left in 0..blocks.len() {
        for right in left + 1..blocks.len() {
            if jaccard(&blocks[left].shingles, &blocks[right].shingles) >= threshold {
                count += 1;
            }
        }
    }

    count
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
            shingles: shingles.to_vec(),
            complexity: 1,
        }],
        diagnostics: Vec::new(),
    }
}
