mod exact;
pub(crate) mod shingles;
mod similarity_join;

use serde::Serialize;

use crate::analysis::{AnalyzedBlock, AnalyzedFile, SourceLocation};
use crate::config::{Config, EffectiveRules};
use crate::language::LanguageId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct BlockId(usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum CloneKind {
    #[serde(rename = "type_1")]
    Type1,
    #[serde(rename = "type_2")]
    Type2,
    #[serde(rename = "type_3")]
    Type3,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct CloneMatch {
    pub(crate) kind: CloneKind,
    pub(crate) similarity: f64,
    pub(crate) left: SourceLocation,
    pub(crate) right: SourceLocation,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct Pair(BlockId, BlockId);

pub(super) struct EligibleBlock<'a> {
    id: BlockId,
    language: LanguageId,
    block: &'a AnalyzedBlock,
    rules: EffectiveRules,
}

#[must_use]
pub(crate) fn detect(files: &[AnalyzedFile], config: &Config) -> Vec<CloneMatch> {
    let blocks = eligible_blocks(files, config);
    let exact::ExactResult {
        mut classified,
        mut matches,
    } = exact::classify(&blocks);
    similarity_join::classify(&blocks, &mut classified, &mut matches);
    sort_matches(&mut matches);
    matches
}

fn eligible_blocks<'a>(files: &'a [AnalyzedFile], config: &Config) -> Vec<EligibleBlock<'a>> {
    let mut source = Vec::new();
    for file in files {
        let rules = config.rules(file.identity.language.key(), file.identity.extension());
        for block in &file.blocks {
            source.push((file.identity.language, block, rules));
        }
    }
    source.sort_unstable_by(|left, right| {
        left.1
            .location
            .cmp(&right.1.location)
            .then(left.1.start_byte.cmp(&right.1.start_byte))
            .then(left.0.cmp(&right.0))
    });
    source
        .into_iter()
        .filter(|(_, block, rules)| {
            block.line_count >= rules.min_lines && block.named_node_count >= rules.min_nodes
        })
        .enumerate()
        .map(|(index, (language, block, rules))| EligibleBlock {
            id: BlockId(index),
            language,
            block,
            rules,
        })
        .collect()
}

fn sort_matches(matches: &mut [CloneMatch]) {
    matches.sort_unstable_by(|left, right| {
        left.kind
            .cmp(&right.kind)
            .then(left.left.cmp(&right.left))
            .then(left.right.cmp(&right.right))
    });
}

impl Pair {
    fn new(left: BlockId, right: BlockId) -> Self {
        if left < right {
            Self(left, right)
        } else {
            Self(right, left)
        }
    }
}

impl CloneMatch {
    fn new(kind: CloneKind, similarity: f64, left: &AnalyzedBlock, right: &AnalyzedBlock) -> Self {
        Self {
            kind,
            similarity,
            left: left.location.clone(),
            right: right.location.clone(),
        }
    }
}
