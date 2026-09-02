mod exact;
mod groups;
pub(crate) mod shingles;
mod similarity_join;

use std::{collections::HashSet, fmt, str::FromStr};

use serde::{Serialize, Serializer};
use thiserror::Error;
use xxhash_rust::xxh3::Xxh3;

use crate::analysis::{AnalyzedBlock, AnalyzedFile, SourceLocation};
use crate::config::{Config, EffectiveRules};
use crate::language::LanguageId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct BlockId(usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct FindingId(u32);

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error(
    "finding ID must be five characters using A-Z, a-z, 0-9, -, or _, with an alphanumeric first character"
)]
pub(crate) struct ParseFindingIdError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum CloneKind {
    #[serde(rename = "type_1")]
    Type1,
    #[serde(rename = "type_2")]
    Type2,
    #[serde(rename = "type_3")]
    Type3,
}

impl CloneKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Type1 => "Type-1",
            Self::Type2 => "Type-2",
            Self::Type3 => "Type-3",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct CloneGroup {
    pub(crate) id: FindingId,
    pub(crate) kind: CloneKind,
    pub(crate) minimum_similarity: f64,
    pub(crate) instances: Vec<SourceLocation>,
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
pub(crate) fn detect(files: &[AnalyzedFile], config: &Config) -> Vec<CloneGroup> {
    let blocks = eligible_blocks(files, config);
    classify(&blocks).finish(&blocks)
}

#[cfg(test)]
#[must_use]
pub(crate) fn detected_relation_count(files: &[AnalyzedFile], config: &Config) -> usize {
    let blocks = eligible_blocks(files, config);
    classify(&blocks).relation_count()
}

fn classify(blocks: &[EligibleBlock<'_>]) -> groups::GroupBuilder {
    let mut classified = HashSet::new();
    let mut groups = groups::GroupBuilder::new(blocks.len());
    exact::classify(blocks, &mut classified, &mut groups);
    similarity_join::classify(blocks, &mut classified, &mut groups);
    groups
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

impl Pair {
    fn new(left: BlockId, right: BlockId) -> Self {
        if left < right {
            Self(left, right)
        } else {
            Self(right, left)
        }
    }

    fn is_containment(self, blocks: &[EligibleBlock<'_>]) -> bool {
        let left = blocks[self.0.0].block;
        let right = blocks[self.1.0].block;
        if left.location.path != right.location.path {
            return false;
        }

        left.start_byte <= right.start_byte && left.end_byte >= right.end_byte
            || right.start_byte <= left.start_byte && right.end_byte >= left.end_byte
    }
}

const FIRST_ID_ALPHABET: &[u8; 62] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const ID_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const FINDING_ID_SPACE: u128 = 62 * 64 * 64 * 64 * 64;

impl FindingId {
    #[must_use]
    pub(crate) fn for_duplicate_locations(locations: &[SourceLocation]) -> Self {
        let mut locations: Vec<_> = locations.iter().collect();
        locations.sort_unstable();
        let mut hash = Xxh3::new();
        hash.update(b"aposlop duplicate group finding v1\0");
        for location in locations {
            hash_location(&mut hash, location);
            hash.update(&[u8::MAX]);
        }
        Self((hash.digest128() % FINDING_ID_SPACE) as u32)
    }

    #[must_use]
    pub(crate) fn for_complexity_location(location: &SourceLocation) -> Self {
        let mut hash = Xxh3::new();
        hash.update(b"aposlop complexity finding v1\0");
        hash_location(&mut hash, location);
        Self((hash.digest128() % FINDING_ID_SPACE) as u32)
    }
}

impl fmt::Display for FindingId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut remaining = self.0;
        let first = FIRST_ID_ALPHABET[(remaining % 62) as usize];
        remaining /= 62;
        let second = ID_ALPHABET[(remaining % 64) as usize];
        remaining /= 64;
        let third = ID_ALPHABET[(remaining % 64) as usize];
        remaining /= 64;
        let fourth = ID_ALPHABET[(remaining % 64) as usize];
        remaining /= 64;
        let fifth = ID_ALPHABET[(remaining % 64) as usize];
        write!(
            formatter,
            "{}{}{}{}{}",
            char::from(first),
            char::from(second),
            char::from(third),
            char::from(fourth),
            char::from(fifth)
        )
    }
}

impl FromStr for FindingId {
    type Err = ParseFindingIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = value.as_bytes();
        if bytes.len() != 5 {
            return Err(ParseFindingIdError);
        }
        let first = alphabet_index(FIRST_ID_ALPHABET, bytes[0])?;
        let mut remaining = 0u32;
        for byte in bytes[1..].iter().rev() {
            let digit = alphabet_index(ID_ALPHABET, *byte)?;
            remaining = remaining * 64 + digit;
        }
        Ok(Self(first + 62 * remaining))
    }
}

fn alphabet_index(alphabet: &[u8], byte: u8) -> Result<u32, ParseFindingIdError> {
    alphabet
        .iter()
        .position(|candidate| *candidate == byte)
        .map(|index| index as u32)
        .ok_or(ParseFindingIdError)
}

impl Serialize for FindingId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl CloneGroup {
    fn new(kind: CloneKind, minimum_similarity: f64, mut instances: Vec<SourceLocation>) -> Self {
        instances.sort_unstable();
        Self {
            id: FindingId::for_duplicate_locations(&instances),
            kind,
            minimum_similarity,
            instances,
        }
    }
}

fn hash_location(hash: &mut Xxh3, location: &SourceLocation) {
    for component in location.path.components() {
        hash.update(component.as_os_str().to_string_lossy().as_bytes());
        hash.update(&[0]);
    }
    hash.update(&location.start_line.to_le_bytes());
    hash.update(&location.end_line.to_le_bytes());
}
