mod coverage;
mod seeds;

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::analysis::{AnalyzedBlock, AnalyzedFile, SourceLocation, Statement};
use crate::config::{Config, EffectiveRules};
use crate::language::LanguageId;

use self::coverage::{AcceptedSpans, BlockIndex, Span};
use self::seeds::seed_groups;
use super::groups::GroupBuilder;
use super::{CloneGroup, CloneKind, EligibleBlock, Pair};

/// Reports maximal equal statement runs that no block relation or larger run relation already covers.
///
/// A run relation is Type-1 when the canonical streams are equal and Type-2 when only the normalized streams are equal.
pub(super) fn detect(
    files: &[AnalyzedFile],
    config: &Config,
    blocks: &[EligibleBlock<'_>],
    classified: &HashSet<Pair>,
) -> Vec<CloneGroup> {
    let sequences = sequences(files, config);
    let mut relations = Vec::new();
    for positions in seed_groups(&sequences).values() {
        for (index, &left) in positions.iter().enumerate() {
            for &right in &positions[index + 1..] {
                relations.extend(Relation::maximal(&sequences, left, right));
            }
        }
    }
    relations.sort_unstable_by_key(|relation| {
        let left = sequences[relation.left.sequence].origin(relation.left);
        let right = sequences[relation.right.sequence].origin(relation.right);
        (
            Reverse(relation.node_count),
            left.min(right),
            left.max(right),
        )
    });

    let blocks = BlockIndex::new(blocks);
    let mut spans = AcceptedSpans::default();
    let mut accepted = Vec::new();
    for relation in relations {
        let left = sequences[relation.left.sequence].span(relation.left, relation.node_count);
        let right = sequences[relation.right.sequence].span(relation.right, relation.node_count);
        if !blocks.covers(classified, &left, &right) && spans.insert(left, right) {
            accepted.push(relation);
        }
    }

    group(&sequences, &accepted)
}

/// One statement sequence with the context that its runs need.
struct Sequence<'a> {
    path: &'a Path,
    language: LanguageId,
    rules: EffectiveRules,
    block: &'a AnalyzedBlock,
    statements: &'a [Statement],
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    sequence: usize,
    statement: usize,
}

/// The statements `start..end` of one sequence.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Run {
    sequence: usize,
    start: usize,
    end: usize,
}

struct Relation {
    left: Run,
    right: Run,
    kind: CloneKind,
    node_count: usize,
}

impl<'a> Sequence<'a> {
    fn same_statement(&self, index: usize, other: &Self, other_index: usize) -> bool {
        let left = &self.statements[index];
        let right = &other.statements[other_index];
        left.normalized_hash == right.normalized_hash
            && self.block.normalized[left.normalized.clone()]
                == other.block.normalized[right.normalized.clone()]
    }

    fn same_canonical(&self, index: usize, other: &Self, other_index: usize) -> bool {
        self.block.exact[self.statements[index].exact.clone()]
            == other.block.exact[other.statements[other_index].exact.clone()]
    }

    fn span(&self, run: Run, named_node_count: usize) -> Span<'a> {
        let (path, start, end) = self.origin(run);
        Span {
            path,
            bytes: start..end,
            named_node_count,
            min_nodes: self.rules.min_nodes,
        }
    }

    /// Identifies a run by source position, independent of input order.
    fn origin(&self, run: Run) -> (&'a Path, usize, usize) {
        (
            self.path,
            self.statements[run.start].start_byte,
            self.statements[run.end - 1].end_byte,
        )
    }

    fn location(&self, run: Run) -> SourceLocation {
        SourceLocation {
            path: self.path.to_path_buf(),
            start_line: self.statements[run.start].start_line,
            end_line: self.statements[run.end - 1].end_line,
        }
    }

    fn is_eligible(&self, run: Run, node_count: usize) -> bool {
        let lines =
            self.statements[run.end - 1].end_line - self.statements[run.start].start_line + 1;
        lines >= self.rules.min_lines && node_count >= self.rules.min_nodes
    }
}

impl Relation {
    /// Extends two equal seed positions into one maximal run relation.
    ///
    /// Positions whose preceding statements are equal return `None` because an earlier seed covers them.
    fn maximal(sequences: &[Sequence<'_>], left: Position, right: Position) -> Option<Self> {
        let first = &sequences[left.sequence];
        let second = &sequences[right.sequence];
        if left.statement > 0
            && right.statement > 0
            && first.same_statement(left.statement - 1, second, right.statement - 1)
        {
            return None;
        }

        let limit = if left.sequence == right.sequence {
            right.statement - left.statement
        } else {
            usize::MAX
        };
        let mut length = 0;
        while length < limit
            && left.statement + length < first.statements.len()
            && right.statement + length < second.statements.len()
            && first.same_statement(left.statement + length, second, right.statement + length)
        {
            length += 1;
        }
        if length == 0 {
            return None;
        }

        let left_run = Run {
            sequence: left.sequence,
            start: left.statement,
            end: left.statement + length,
        };
        let right_run = Run {
            sequence: right.sequence,
            start: right.statement,
            end: right.statement + length,
        };
        let statements = &first.statements[left_run.start..left_run.end];
        let node_count = statements
            .iter()
            .map(|statement| statement.named_node_count)
            .sum();
        if first
            .span(left_run, node_count)
            .overlaps(&second.span(right_run, node_count))
            || !first.is_eligible(left_run, node_count)
            || !second.is_eligible(right_run, node_count)
            || is_repetition(statements)
        {
            return None;
        }

        let canonical = (0..length).all(|offset| {
            first.same_canonical(left.statement + offset, second, right.statement + offset)
        });
        let (kind, enabled) = if canonical {
            (CloneKind::Type1, first.rules.type_1 && second.rules.type_1)
        } else {
            (CloneKind::Type2, first.rules.type_2 && second.rules.type_2)
        };
        enabled.then_some(Self {
            left: left_run,
            right: right_run,
            kind,
            node_count,
        })
    }
}

fn sequences<'a>(files: &'a [AnalyzedFile], config: &Config) -> Vec<Sequence<'a>> {
    let mut sequences = Vec::new();
    for file in files {
        let rules = config.rules(file.identity.language.key(), file.identity.extension());
        if !rules.statement_runs || !rules.type_1 && !rules.type_2 {
            continue;
        }
        for block in &file.blocks {
            for statements in &block.sequences {
                sequences.push(Sequence {
                    path: &file.identity.path,
                    language: file.identity.language,
                    rules,
                    block,
                    statements,
                });
            }
        }
    }
    sequences
}

fn group(sequences: &[Sequence<'_>], relations: &[Relation]) -> Vec<CloneGroup> {
    let mut members: HashMap<Run, usize> = HashMap::new();
    let mut runs = Vec::new();
    let mut intern = |run: Run| {
        *members.entry(run).or_insert_with(|| {
            runs.push(run);
            runs.len() - 1
        })
    };
    let edges: Vec<_> = relations
        .iter()
        .map(|relation| (intern(relation.left), intern(relation.right), relation.kind))
        .collect();

    let mut builder = GroupBuilder::new(runs.len());
    for (left, right, kind) in edges {
        builder.add(left, right, kind, 1.0);
    }
    builder.finish(|index| sequences[runs[index].sequence].location(runs[index]))
}

/// Reports whether the statements repeat one shorter unit of normalized statements at least twice.
fn is_repetition(statements: &[Statement]) -> bool {
    (1..=statements.len() / 2).any(|period| {
        statements
            .iter()
            .zip(&statements[period..])
            .all(|(left, right)| left.normalized_hash == right.normalized_hash)
    })
}
