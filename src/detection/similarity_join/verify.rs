use std::collections::HashSet;

use super::super::shingles::jaccard;
use super::super::{CloneKind, CloneMatch, EligibleBlock, Pair};

pub(super) fn verify_pair(
    blocks: &[EligibleBlock<'_>],
    pair: Pair,
    threshold: f64,
    classified: &mut HashSet<Pair>,
    matches: &mut Vec<CloneMatch>,
) {
    if classified.contains(&pair) {
        return;
    }

    let left = &blocks[pair.0.0];
    let right = &blocks[pair.1.0];

    let similarity = jaccard(&left.block.shingles, &right.block.shingles);
    if similarity < threshold {
        return;
    }

    classified.insert(pair);
    matches.push(CloneMatch::new(
        CloneKind::Type3,
        similarity,
        left.block,
        right.block,
    ));
}
