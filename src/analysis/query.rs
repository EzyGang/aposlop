use std::collections::BTreeSet;
use std::ops::Range;

use tree_sitter::{Node, QueryCursor, StreamingIterator};

use crate::language::CompiledLanguage;

#[derive(Default)]
pub(super) struct CaptureRanges<'tree> {
    pub(super) blocks: Vec<Range<usize>>,
    pub(super) sequences: Vec<Node<'tree>>,
    pub(super) ignored: Vec<Range<usize>>,
    pub(super) identifiers: Vec<Range<usize>>,
    pub(super) literals: Vec<Range<usize>>,
}

pub(super) fn block_nodes<'tree>(
    provider: &CompiledLanguage,
    root: Node<'tree>,
    source: &[u8],
) -> Vec<Node<'tree>> {
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&provider.normalization, root, source);
    let names = provider.normalization.capture_names();
    let mut blocks = Vec::new();

    while let Some(query_match) = matches.next() {
        for capture in query_match.captures {
            if names[capture.index as usize] == "block" {
                blocks.push(capture.node);
            }
        }
    }

    blocks.sort_unstable_by_key(|node| (node.start_byte(), node.end_byte()));
    blocks.dedup_by_key(|node| (node.start_byte(), node.end_byte()));
    blocks
}

pub(super) fn capture_ranges<'tree>(
    provider: &CompiledLanguage,
    block: Node<'tree>,
    source: &[u8],
) -> CaptureRanges<'tree> {
    let mut ranges = CaptureRanges::default();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&provider.normalization, block, source);
    let names = provider.normalization.capture_names();

    while let Some(query_match) = matches.next() {
        for capture in query_match.captures {
            let range = capture.node.byte_range();
            match names[capture.index as usize] {
                "ignore" => ranges.ignored.push(range),
                "anonymize.identifier" => ranges.identifiers.push(range),
                "anonymize.literal" => ranges.literals.push(range),
                "block" => ranges.blocks.push(range),
                "sequence" => ranges.sequences.push(capture.node),
                _ => (),
            }
        }
    }

    ranges
}

pub(super) fn complexity(
    provider: &CompiledLanguage,
    block: Node<'_>,
    source: &[u8],
    ranges: &CaptureRanges<'_>,
) -> usize {
    let block_range = block.byte_range();
    let mut captures = BTreeSet::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&provider.metrics, block, source);

    while let Some(query_match) = matches.next() {
        for capture in query_match.captures {
            let range = capture.node.byte_range();
            if ranges.is_in_nested_block(&block_range, &range) {
                continue;
            }
            captures.insert((range.start, range.end));
        }
    }

    1 + captures.len()
}

pub(super) fn contains_invalid(root: Node<'_>) -> bool {
    let mut stack = vec![root];

    while let Some(node) = stack.pop() {
        if node.is_error() || node.is_missing() {
            return true;
        }

        for index in 0..node.child_count() {
            if let Some(child) = node.child(index) {
                stack.push(child);
            }
        }
    }

    false
}

impl CaptureRanges<'_> {
    /// Reports whether `range` belongs to a block nested inside `block`.
    pub(super) fn is_in_nested_block(&self, block: &Range<usize>, range: &Range<usize>) -> bool {
        self.blocks
            .iter()
            .any(|nested| nested != block && nested.start <= range.start && nested.end >= range.end)
    }
}
