use std::ops::Range;
use std::path::Path;

use tree_sitter::Node;
use xxhash_rust::xxh3::xxh3_64;

use crate::detection::near_miss::{build_shingles, build_signature};
use crate::language::CompiledLanguage;

use super::query::{capture_ranges, complexity};
use super::{AnalyzedBlock, SourceLocation};

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
    let mut named_node_count = 0;
    let mut stack = vec![block];

    while let Some(node) = stack.pop() {
        named_node_count += usize::from(node.is_named());

        if ranges.ignored.iter().any(|range| contains(range, node)) {
            continue;
        }

        if node.child_count() == 0 {
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

    let shingles = build_shingles(&token_hashes);
    let signature = build_signature(&shingles);
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
        token_hashes,
        shingles,
        signature,
        complexity: complexity(provider, block, source),
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
