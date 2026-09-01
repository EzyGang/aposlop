use std::collections::{HashMap, HashSet};

use crate::language::LanguageId;

use super::{BlockId, CloneKind, CloneMatch, EligibleBlock, Pair};

pub(super) struct ExactResult {
    pub(super) classified: HashSet<Pair>,
    pub(super) matches: Vec<CloneMatch>,
}

pub(super) fn classify(blocks: &[EligibleBlock<'_>]) -> ExactResult {
    let mut result = ExactResult {
        classified: HashSet::new(),
        matches: Vec::new(),
    };
    classify_groups(blocks, true, &mut result);
    classify_groups(blocks, false, &mut result);
    result
}

fn classify_groups(blocks: &[EligibleBlock<'_>], exact: bool, result: &mut ExactResult) {
    let mut groups: HashMap<(LanguageId, u64), Vec<BlockId>> = HashMap::new();
    for block in blocks {
        let hash = if exact {
            block.block.exact_hash
        } else {
            block.block.normalized_hash
        };
        groups
            .entry((block.language, hash))
            .or_default()
            .push(block.id);
    }
    for group in groups.values() {
        for left_index in 0..group.len() {
            for right_index in left_index + 1..group.len() {
                let pair = Pair::new(group[left_index], group[right_index]);
                if result.classified.contains(&pair) {
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
                result.classified.insert(pair);
                let enabled = if exact {
                    left.rules.type_1 && right.rules.type_1
                } else {
                    left.rules.type_2 && right.rules.type_2
                };
                if enabled {
                    result.matches.push(CloneMatch::new(
                        if exact {
                            CloneKind::Type1
                        } else {
                            CloneKind::Type2
                        },
                        1.0,
                        left.block,
                        right.block,
                    ));
                }
            }
        }
    }
}
