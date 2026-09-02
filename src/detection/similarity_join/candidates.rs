use std::collections::{HashMap, HashSet};

use super::super::groups::GroupBuilder;
use super::super::{BlockId, EligibleBlock, Pair};
use super::ThresholdGroup;
use super::verify::verify_pair;

#[derive(Clone, Copy)]
struct Posting {
    id: BlockId,
    position: usize,
}

#[derive(Default)]
struct CandidateState {
    overlap: usize,
    pruned: bool,
}

pub(super) fn join_groups(
    blocks: &[EligibleBlock<'_>],
    ranked: &[Vec<usize>],
    left: &ThresholdGroup,
    right: &ThresholdGroup,
    same_group: bool,
    classified: &mut HashSet<Pair>,
    groups: &mut GroupBuilder,
) {
    let threshold = left.threshold.max(right.threshold);
    if threshold == 0.0 {
        join_all_pairs(blocks, left, right, same_group, classified, groups);
        return;
    }

    let mut index: HashMap<usize, Vec<Posting>> = HashMap::new();
    for &id in &right.ids {
        let tokens = &ranked[id.0];
        for (position, &token) in tokens
            .iter()
            .take(prefix_length(tokens.len(), threshold))
            .enumerate()
        {
            index
                .entry(token)
                .or_default()
                .push(Posting { id, position });
        }
    }

    let mut candidates: HashMap<Pair, CandidateState> = HashMap::new();
    for &left_id in &left.ids {
        let left_tokens = &ranked[left_id.0];
        for (left_position, &token) in left_tokens
            .iter()
            .take(prefix_length(left_tokens.len(), threshold))
            .enumerate()
        {
            let Some(postings) = index.get(&token) else {
                continue;
            };
            for posting in postings {
                if left_id == posting.id || same_group && left_id > posting.id {
                    continue;
                }
                let right_tokens = &ranked[posting.id.0];
                if !lengths_can_match(left_tokens.len(), right_tokens.len(), threshold) {
                    continue;
                }

                let pair = Pair::new(left_id, posting.id);
                let state = candidates.entry(pair).or_default();
                if state.pruned {
                    continue;
                }
                state.overlap += 1;

                let remaining = (left_tokens.len() - left_position - 1)
                    .min(right_tokens.len() - posting.position - 1);
                let required = required_overlap(left_tokens.len(), right_tokens.len(), threshold);
                if state.overlap + remaining < required {
                    state.pruned = true;
                }
            }
        }
    }

    add_empty_pairs(blocks, left, right, same_group, &mut candidates);

    for (pair, state) in candidates {
        if !state.pruned {
            verify_pair(blocks, pair, threshold, classified, groups);
        }
    }
}

fn join_all_pairs(
    blocks: &[EligibleBlock<'_>],
    left: &ThresholdGroup,
    right: &ThresholdGroup,
    same_group: bool,
    classified: &mut HashSet<Pair>,
    groups: &mut GroupBuilder,
) {
    for &left_id in &left.ids {
        for &right_id in &right.ids {
            if left_id == right_id || same_group && left_id > right_id {
                continue;
            }
            verify_pair(
                blocks,
                Pair::new(left_id, right_id),
                0.0,
                classified,
                groups,
            );
        }
    }
}

fn add_empty_pairs(
    blocks: &[EligibleBlock<'_>],
    left: &ThresholdGroup,
    right: &ThresholdGroup,
    same_group: bool,
    candidates: &mut HashMap<Pair, CandidateState>,
) {
    for &left_id in &left.ids {
        if !blocks[left_id.0].block.shingles.is_empty() {
            continue;
        }
        for &right_id in &right.ids {
            if left_id == right_id || same_group && left_id > right_id {
                continue;
            }
            if blocks[right_id.0].block.shingles.is_empty() {
                candidates.entry(Pair::new(left_id, right_id)).or_default();
            }
        }
    }
}

fn prefix_length(length: usize, threshold: f64) -> usize {
    length
        .saturating_sub((threshold * length as f64).ceil() as usize)
        .saturating_add(1)
        .min(length)
}

fn lengths_can_match(left: usize, right: usize, threshold: f64) -> bool {
    let maximum = left.max(right);
    maximum == 0 || left.min(right) as f64 / maximum as f64 >= threshold
}

fn required_overlap(left: usize, right: usize, threshold: f64) -> usize {
    let maximum = left.min(right);
    let estimate = threshold * (left + right) as f64 / (1.0 + threshold);
    let mut required = estimate.ceil() as usize;

    while required > 0 && overlap_similarity(required - 1, left, right) >= threshold {
        required -= 1;
    }
    while required <= maximum && overlap_similarity(required, left, right) < threshold {
        required += 1;
    }

    required
}

fn overlap_similarity(overlap: usize, left: usize, right: usize) -> f64 {
    let union = left + right - overlap;
    if union == 0 {
        return 1.0;
    }
    overlap as f64 / union as f64
}
