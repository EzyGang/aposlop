use std::collections::HashSet;

use super::super::groups::GroupBuilder;
use super::super::shingles::jaccard;
use super::super::{CloneKind, EligibleBlock, Pair};

pub(super) fn verify_pair(
    blocks: &[EligibleBlock<'_>],
    pair: Pair,
    threshold: f64,
    classified: &mut HashSet<Pair>,
    groups: &mut GroupBuilder,
) {
    if pair.is_containment(blocks) || classified.contains(&pair) {
        return;
    }

    let left = &blocks[pair.0.0];
    let right = &blocks[pair.1.0];

    let similarity = jaccard(&left.block.shingles, &right.block.shingles);
    if similarity < threshold {
        return;
    }

    classified.insert(pair);
    groups.add(pair, CloneKind::Type3, similarity);
}
