mod candidates;
mod verify;

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::language::LanguageId;

use super::{BlockId, CloneMatch, EligibleBlock, Pair};

struct ThresholdGroup {
    threshold: f64,
    ids: Vec<BlockId>,
}

pub(super) fn classify(
    blocks: &[EligibleBlock<'_>],
    classified: &mut HashSet<Pair>,
    matches: &mut Vec<CloneMatch>,
) {
    let languages = threshold_groups(blocks);

    for groups in languages.into_values() {
        let ids: Vec<_> = groups
            .values()
            .flat_map(|group| group.iter().copied())
            .collect();
        let ranked = rank_shingles(blocks, &ids);
        let groups: Vec<_> = groups
            .into_iter()
            .map(|(bits, ids)| ThresholdGroup {
                threshold: f64::from_bits(bits),
                ids,
            })
            .collect();

        for left in 0..groups.len() {
            for right in left..groups.len() {
                candidates::join_groups(
                    blocks,
                    &ranked,
                    &groups[left],
                    &groups[right],
                    left == right,
                    classified,
                    matches,
                );
            }
        }
    }
}

fn threshold_groups(
    blocks: &[EligibleBlock<'_>],
) -> BTreeMap<LanguageId, BTreeMap<u64, Vec<BlockId>>> {
    let mut languages: BTreeMap<LanguageId, BTreeMap<u64, Vec<BlockId>>> = BTreeMap::new();

    for block in blocks {
        if !block.rules.type_3 {
            continue;
        }
        let threshold = if block.rules.type_3_threshold == 0.0 {
            0.0
        } else {
            block.rules.type_3_threshold
        };
        languages
            .entry(block.language)
            .or_default()
            .entry(threshold.to_bits())
            .or_default()
            .push(block.id);
    }

    languages
}

fn rank_shingles(blocks: &[EligibleBlock<'_>], ids: &[BlockId]) -> Vec<Vec<usize>> {
    let mut frequencies = HashMap::new();
    for &id in ids {
        for &shingle in &blocks[id.0].block.shingles {
            *frequencies.entry(shingle).or_insert(0usize) += 1;
        }
    }

    let mut ordered: Vec<_> = frequencies.into_iter().collect();
    ordered.sort_unstable_by_key(|(shingle, frequency)| (*frequency, *shingle));
    let ranks: HashMap<_, _> = ordered
        .into_iter()
        .enumerate()
        .map(|(rank, (shingle, _))| (shingle, rank))
        .collect();

    let mut ranked = vec![Vec::new(); blocks.len()];
    for &id in ids {
        let tokens = &mut ranked[id.0];
        tokens.reserve(blocks[id.0].block.shingles.len());
        tokens.extend(
            blocks[id.0]
                .block
                .shingles
                .iter()
                .filter_map(|shingle| ranks.get(shingle).copied()),
        );
        tokens.sort_unstable();
    }

    ranked
}
