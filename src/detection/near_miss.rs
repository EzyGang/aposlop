use std::collections::{HashMap, HashSet};

use xxhash_rust::xxh3::{Xxh3, xxh3_64_with_seed};

use crate::language::LanguageId;

use super::{BlockId, CloneKind, CloneMatch, EligibleBlock, Pair, candidate_pairs};

pub(crate) const SHINGLE_WIDTH: usize = 5;
pub(crate) const SIGNATURE_LENGTH: usize = 100;
pub(crate) const BAND_COUNT: usize = 20;
pub(crate) const ROWS_PER_BAND: usize = 5;
const SEED_BASE: u64 = 0xA9_05_10_50_4C_4F_50_01;

#[must_use]
pub(crate) fn build_shingles(tokens: &[u64]) -> Vec<u64> {
    if tokens.is_empty() {
        return Vec::new();
    }
    let mut shingles = Vec::with_capacity(tokens.len().saturating_sub(SHINGLE_WIDTH) + 1);
    if tokens.len() < SHINGLE_WIDTH {
        shingles.push(hash_window(tokens));
    } else {
        for window in tokens.windows(SHINGLE_WIDTH) {
            shingles.push(hash_window(window));
        }
    }
    shingles.sort_unstable();
    shingles.dedup();
    shingles
}

#[must_use]
pub(crate) fn build_signature(shingles: &[u64]) -> Vec<u64> {
    let mut signature = vec![u64::MAX; SIGNATURE_LENGTH];
    for (index, minimum) in signature.iter_mut().enumerate() {
        let seed = seed(index);
        for &shingle in shingles {
            *minimum = (*minimum).min(xxh3_64_with_seed(&shingle.to_le_bytes(), seed));
        }
    }
    signature
}

#[must_use]
pub(crate) fn band_hash(signature: &[u64], band: usize) -> Option<u64> {
    let start = band.checked_mul(ROWS_PER_BAND)?;
    let values = signature.get(start..start + ROWS_PER_BAND)?;
    Some(hash_window(values))
}

#[must_use]
pub(crate) fn jaccard(left: &[u64], right: &[u64]) -> f64 {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut intersection = 0usize;
    while left_index < left.len() && right_index < right.len() {
        match left[left_index].cmp(&right[right_index]) {
            std::cmp::Ordering::Less => left_index += 1,
            std::cmp::Ordering::Greater => right_index += 1,
            std::cmp::Ordering::Equal => {
                intersection += 1;
                left_index += 1;
                right_index += 1;
            }
        }
    }
    let union = left.len() + right.len() - intersection;
    if union == 0 {
        return 1.0;
    }
    intersection as f64 / union as f64
}

pub(super) fn classify(
    blocks: &[EligibleBlock<'_>],
    classified: &mut HashSet<Pair>,
    matches: &mut Vec<CloneMatch>,
) {
    let mut buckets: HashMap<(LanguageId, usize, u64), Vec<BlockId>> = HashMap::new();
    for block in blocks {
        if !block.rules.type_3 {
            continue;
        }
        for band in 0..BAND_COUNT {
            if let Some(hash) = band_hash(&block.block.signature, band) {
                buckets
                    .entry((block.language, band, hash))
                    .or_default()
                    .push(block.id);
            }
        }
    }
    let mut candidates: Vec<_> = candidate_pairs(buckets.into_values()).into_iter().collect();
    candidates.sort_unstable_by_key(|pair| (pair.0.0, pair.1.0));
    for pair in candidates {
        if classified.contains(&pair) {
            continue;
        }
        let left = &blocks[pair.0.0];
        let right = &blocks[pair.1.0];
        let similarity = jaccard(&left.block.shingles, &right.block.shingles);
        let threshold = left
            .rules
            .type_3_threshold
            .max(right.rules.type_3_threshold);
        if similarity < threshold {
            continue;
        }
        classified.insert(pair);
        matches.push(CloneMatch::new(
            CloneKind::Type3,
            similarity,
            left.block,
            right.block,
        ));
    }
}

fn hash_window(values: &[u64]) -> u64 {
    let mut hash = Xxh3::new();
    for value in values {
        hash.update(&value.to_le_bytes());
    }
    hash.digest()
}

fn seed(index: usize) -> u64 {
    let mut value = SEED_BASE.wrapping_add(index as u64);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}
