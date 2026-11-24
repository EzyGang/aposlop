use std::ops::Range;
use std::path::Path;

use tree_sitter::Node;
use xxhash_rust::xxh3::xxh3_64;

use crate::detection::shingles::build_shingles;
use crate::language::CompiledLanguage;

use super::query::{capture_ranges, complexity};
use super::{AnalyzedBlock, SourceLocation, Statement};

pub(super) fn build_block(
    provider: &CompiledLanguage,
    block: Node<'_>,
    source: &[u8],
    path: &Path,
) -> AnalyzedBlock {
    let ranges = capture_ranges(provider, block, source);
    let mut exact = Vec::new();
    let mut normalized = Vec::new();
    let mut token_hashes = Vec::new();
    let mut leaves = Vec::new();
    let mut named_node_count = 0;
    let mut stack = vec![block];

    while let Some(node) = stack.pop() {
        named_node_count += usize::from(node.is_named());

        if ranges.ignored.iter().any(|range| contains(range, node)) {
            continue;
        }

        if node.child_count() == 0 {
            leaves.push(Leaf {
                start_byte: node.start_byte(),
                exact: exact.len(),
                normalized: normalized.len(),
            });
            append_record(&mut exact, node.kind().as_bytes(), node_text(source, node));
            let start = normalized.len();
            let role = if ranges.identifiers.iter().any(|range| contains(range, node)) {
                b"<IDENT>".as_slice()
            } else if ranges.literals.iter().any(|range| contains(range, node)) {
                b"<LITERAL>".as_slice()
            } else {
                node_text(source, node)
            };

            append_record(&mut normalized, node.kind().as_bytes(), role);
            token_hashes.push(xxh3_64(&normalized[start..]));
            continue;
        }

        for index in (0..node.child_count()).rev() {
            if let Some(child) = node.child(index) {
                stack.push(child);
            }
        }
    }

    leaves.push(Leaf {
        start_byte: usize::MAX,
        exact: exact.len(),
        normalized: normalized.len(),
    });
    let block_range = block.byte_range();
    let sequences = ranges
        .sequences
        .iter()
        .filter(|sequence| !ranges.is_in_nested_block(&block_range, &sequence.byte_range()))
        .map(|&sequence| statements(sequence, &leaves, &normalized))
        .filter(|statements| !statements.is_empty())
        .collect();
    let shingles = build_shingles(&token_hashes);
    let start_line = block.start_position().row + 1;
    let end_line = block.end_position().row + 1;

    AnalyzedBlock {
        location: SourceLocation {
            path: path.to_path_buf(),
            start_line,
            end_line,
        },
        start_byte: block.start_byte(),
        end_byte: block.end_byte(),
        line_count: end_line - start_line + 1,
        named_node_count,
        exact_hash: xxh3_64(&exact),
        normalized_hash: xxh3_64(&normalized),
        exact,
        normalized,
        shingles,
        complexity: complexity(provider, block, source, &ranges),
        sequences,
    }
}

/// Stream offsets where one leaf record starts.
struct Leaf {
    start_byte: usize,
    exact: usize,
    normalized: usize,
}

/// Builds the statements of `sequence` from the leaf offsets of the owning block.
///
/// The final leaf in `leaves` is a sentinel that holds the stream lengths.
fn statements(sequence: Node<'_>, leaves: &[Leaf], normalized: &[u8]) -> Vec<Statement> {
    let mut cursor = sequence.walk();
    sequence
        .named_children(&mut cursor)
        .filter(|statement| !statement.is_extra())
        .map(|statement| {
            let first =
                &leaves[leaves.partition_point(|leaf| leaf.start_byte < statement.start_byte())];
            let last =
                &leaves[leaves.partition_point(|leaf| leaf.start_byte < statement.end_byte())];
            let normalized_range = first.normalized..last.normalized;
            Statement {
                start_byte: statement.start_byte(),
                end_byte: statement.end_byte(),
                start_line: statement.start_position().row + 1,
                end_line: statement.end_position().row + 1,
                named_node_count: subtree_named_node_count(statement),
                exact: first.exact..last.exact,
                normalized_hash: xxh3_64(&normalized[normalized_range.clone()]),
                normalized: normalized_range,
            }
        })
        .collect()
}

fn subtree_named_node_count(root: Node<'_>) -> usize {
    let mut cursor = root.walk();
    let mut count = 0;
    loop {
        count += usize::from(cursor.node().is_named());
        if cursor.goto_first_child() || cursor.goto_next_sibling() {
            continue;
        }
        loop {
            if !cursor.goto_parent() {
                return count;
            }
            if cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn contains(range: &Range<usize>, node: Node<'_>) -> bool {
    range.start <= node.start_byte() && range.end >= node.end_byte()
}

fn node_text<'a>(source: &'a [u8], node: Node<'_>) -> &'a [u8] {
    source.get(node.byte_range()).unwrap_or_default()
}

fn append_record(output: &mut Vec<u8>, kind: &[u8], value: &[u8]) {
    output.extend_from_slice(&(kind.len() as u64).to_le_bytes());
    output.extend_from_slice(kind);
    output.extend_from_slice(&(value.len() as u64).to_le_bytes());
    output.extend_from_slice(value);
}
