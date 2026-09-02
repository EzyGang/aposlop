use std::collections::{HashMap, HashSet};

use crate::language::LanguageId;

use super::groups::GroupBuilder;
use super::{BlockId, CloneKind, EligibleBlock, Pair};

pub(super) fn classify(
    blocks: &[EligibleBlock<'_>],
    classified: &mut HashSet<Pair>,
    groups: &mut GroupBuilder,
) {
    classify_groups(blocks, true, classified, groups);
    classify_groups(blocks, false, classified, groups);
}

fn classify_groups(
    blocks: &[EligibleBlock<'_>],
    exact: bool,
    classified: &mut HashSet<Pair>,
    groups: &mut GroupBuilder,
) {
    let mut candidate_groups: HashMap<(LanguageId, u64), Vec<BlockId>> = HashMap::new();
    for block in blocks {
        let hash = if exact {
            block.block.exact_hash
        } else {
            block.block.normalized_hash
        };
        candidate_groups
            .entry((block.language, hash))
            .or_default()
            .push(block.id);
    }
    for group in candidate_groups.values() {
        for left_index in 0..group.len() {
            for right_index in left_index + 1..group.len() {
                let pair = Pair::new(group[left_index], group[right_index]);
                if pair.is_containment(blocks) {
                    continue;
                }
                if classified.contains(&pair) {
                    continue;
                }
                let left = &blocks[pair.0.0];
                let right = &blocks[pair.1.0];
                let equal = if exact {
                    left.block.exact == right.block.exact
                } else {
                    left.block.normalized == right.block.normalized
                };
                if !equal {
                    continue;
                }
                classified.insert(pair);
                let enabled = if exact {
                    left.rules.type_1 && right.rules.type_1
                } else {
                    left.rules.type_2 && right.rules.type_2
                };
                if enabled {
                    groups.add(
                        pair,
                        if exact {
                            CloneKind::Type1
                        } else {
                            CloneKind::Type2
                        },
                        1.0,
                    );
                }
            }
        }
    }
}
