use std::collections::BTreeSet;
use std::ops::Range;

use tree_sitter::{Node, QueryCursor, StreamingIterator};

use crate::language::CompiledLanguage;

#[derive(Default)]
pub(super) struct CaptureRanges {
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

pub(super) fn capture_ranges(
    provider: &CompiledLanguage,
    block: Node<'_>,
    source: &[u8],
) -> CaptureRanges {
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
                _ => (),
            }
        }
    }

    ranges
}

pub(super) fn complexity(provider: &CompiledLanguage, block: Node<'_>, source: &[u8]) -> usize {
    let mut captures = BTreeSet::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&provider.metrics, block, source);

    while let Some(query_match) = matches.next() {
        for capture in query_match.captures {
            captures.insert((capture.node.start_byte(), capture.node.end_byte()));
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
