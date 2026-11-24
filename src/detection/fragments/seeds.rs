use std::collections::HashMap;

use xxhash_rust::xxh3::Xxh3;

use crate::language::LanguageId;

use super::{Position, Sequence};

/// Groups each position by the normalized hash of its shortest run that reaches the seed node count.
///
/// The seed node count is the smallest `min_nodes` value of the language, so equal runs share one seed.
pub(super) fn seed_groups(sequences: &[Sequence<'_>]) -> HashMap<(LanguageId, u64), Vec<Position>> {
    let mut seed_nodes: HashMap<LanguageId, usize> = HashMap::new();
    for sequence in sequences {
        let required = seed_nodes.entry(sequence.language).or_insert(usize::MAX);
        *required = (*required).min(sequence.rules.min_nodes.max(1));
    }

    let mut groups: HashMap<(LanguageId, u64), Vec<Position>> = HashMap::new();
    for (index, sequence) in sequences.iter().enumerate() {
        let required = seed_nodes.get(&sequence.language).copied().unwrap_or(1);
        let statements = sequence.statements;
        let mut end = 0;
        let mut nodes = 0;
        for start in 0..statements.len() {
            while nodes < required && end < statements.len() {
                nodes += statements[end].named_node_count;
                end += 1;
            }
            if nodes < required {
                break;
            }

            let mut hash = Xxh3::new();
            for statement in &statements[start..end] {
                hash.update(&statement.normalized_hash.to_le_bytes());
            }
            groups
                .entry((sequence.language, hash.digest()))
                .or_default()
                .push(Position {
                    sequence: index,
                    statement: start,
                });
            nodes -= statements[start].named_node_count;
        }
    }
    groups
}
