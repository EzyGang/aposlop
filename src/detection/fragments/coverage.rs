use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::Path;

use crate::detection::{BlockId, EligibleBlock, Pair};

/// The source bytes and size of one run.
#[derive(Clone, Debug)]
pub(super) struct Span<'a> {
    pub(super) path: &'a Path,
    pub(super) bytes: Range<usize>,
    pub(super) named_node_count: usize,
    pub(super) min_nodes: usize,
}

/// Spans of accepted run relations, keyed by their ordered path pair.
#[derive(Default)]
pub(super) struct AcceptedSpans<'a> {
    pairs: HashMap<(&'a Path, &'a Path), Vec<[Span<'a>; 2]>>,
}

/// Eligible blocks of each file, used to find block relations that cover a run relation.
pub(super) struct BlockIndex<'a> {
    files: HashMap<&'a Path, Vec<&'a EligibleBlock<'a>>>,
}

impl Span<'_> {
    pub(super) fn overlaps(&self, other: &Self) -> bool {
        self.path == other.path
            && self.bytes.start < other.bytes.end
            && other.bytes.start < self.bytes.end
    }

    /// Reports whether `block` contains this span.
    fn is_inside(&self, block: &EligibleBlock<'_>) -> bool {
        block.block.start_byte <= self.bytes.start && block.block.end_byte >= self.bytes.end
    }

    /// Reports whether this span contains `block` and too few other named nodes to qualify alone.
    fn wraps(&self, block: &EligibleBlock<'_>) -> bool {
        self.bytes.start <= block.block.start_byte
            && self.bytes.end >= block.block.end_byte
            && self
                .named_node_count
                .saturating_sub(block.block.named_node_count)
                < self.min_nodes
    }
}

impl<'a> AcceptedSpans<'a> {
    /// Records a relation unless an accepted relation overlaps both of its spans.
    pub(super) fn insert(&mut self, left: Span<'a>, right: Span<'a>) -> bool {
        let pair = if left.path <= right.path {
            [left, right]
        } else {
            [right, left]
        };
        let known = self.pairs.entry((pair[0].path, pair[1].path)).or_default();
        if known.iter().any(|[first, second]| {
            first.overlaps(&pair[0]) && second.overlaps(&pair[1])
                || first.overlaps(&pair[1]) && second.overlaps(&pair[0])
        }) {
            return false;
        }
        known.push(pair);
        true
    }
}

impl<'a> BlockIndex<'a> {
    pub(super) fn new(blocks: &'a [EligibleBlock<'a>]) -> Self {
        let mut files: HashMap<&Path, Vec<&EligibleBlock<'_>>> = HashMap::new();
        for block in blocks {
            files
                .entry(block.block.location.path.as_path())
                .or_default()
                .push(block);
        }
        Self { files }
    }

    /// Reports whether a classified block pair already describes the run relation.
    ///
    /// A block pair describes it when the blocks contain both spans, or when both spans only wrap the blocks.
    pub(super) fn covers(
        &self,
        classified: &HashSet<Pair>,
        left: &Span<'_>,
        right: &Span<'_>,
    ) -> bool {
        self.relates(classified, left, right, |span, block| span.is_inside(block))
            || self.relates(classified, left, right, |span, block| span.wraps(block))
    }

    fn relates<F>(
        &self,
        classified: &HashSet<Pair>,
        left: &Span<'_>,
        right: &Span<'_>,
        test: F,
    ) -> bool
    where
        F: Fn(&Span<'_>, &EligibleBlock<'_>) -> bool,
    {
        let left_blocks = self.matching(left, &test);
        self.matching(right, &test).iter().any(|&right| {
            left_blocks
                .iter()
                .any(|&left| classified.contains(&Pair::new(left, right)))
        })
    }

    fn matching<F>(&self, span: &Span<'_>, test: &F) -> Vec<BlockId>
    where
        F: Fn(&Span<'_>, &EligibleBlock<'_>) -> bool,
    {
        self.files.get(span.path).map_or_else(Vec::new, |blocks| {
            blocks
                .iter()
                .filter(|block| test(span, block))
                .map(|block| block.id)
                .collect()
        })
    }
}
