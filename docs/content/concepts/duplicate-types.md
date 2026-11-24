---
title: Duplicate types
---

# Duplicate types

Aposlop classifies block relations into three mutually exclusive duplicate types.
It also finds Type-1 and Type-2 relations between statement runs inside larger blocks.
It combines connected relations into one duplicate group with two or more source instances.

## Type-1

A Type-1 relation connects blocks with equal canonical streams.
The canonical stream preserves identifiers and literals but ignores whitespace and comments.

Formatting-only and comment-only changes preserve a Type-1 relation.

## Type-2

A Type-2 relation connects different canonical streams with equal normalized streams.
Aposlop replaces captured identifiers and literals with category markers during normalization.

Renaming variables or changing literal values can produce a Type-2 relation.
Operator and control-flow changes alter the normalized stream.

## Type-3

A Type-3 relation connects different normalized streams that meet the effective Jaccard threshold.

Aposlop builds five-token shingles and runs an exact prefix-filtered similarity join.
A two-pointer Jaccard comparison verifies every surviving Type-3 candidate relation.

## Duplicate groups

Aposlop combines enabled relations into connected components.
Each connected component becomes one duplicate group.

For example, five mutually matching blocks produce one group with five instances instead of ten pair findings.

Type-3 similarity is not transitive.
Two instances in one Type-3 group do not necessarily match each other directly.
Each instance connects to the group through at least one accepted relation.

Each group reports:

- `kind` is the broadest relation required to connect the group.
- `minimum_similarity` is the lowest similarity among accepted relations in the group.
- `instances` contains every source location in deterministic order.

Type-3 is broader than Type-2, and Type-2 is broader than Type-1.
The minimum similarity does not describe untested or rejected pairs within a connected group.

## Precedence

Aposlop checks each block relation in this order:

1. Aposlop checks Type-1.
2. Aposlop checks Type-2.
3. Aposlop checks Type-3.

Aposlop classifies one block relation only once.
Disabling an earlier duplicate type suppresses that relation without reclassifying it as a later type.

## Block eligibility

Both blocks must meet their effective `min_lines` and `min_nodes` values.
Both blocks must enable the applicable duplicate type.

A Type-3 relation uses the larger effective threshold from its two blocks.
Sibling blocks can match within one file.
Two blocks do not match when either contains the other in the same file.

Aposlop compares blocks only when they use the same language identity.
TypeScript and TSX share one language identity and can match each other.

## Statement runs

A statement run is a sequence of adjacent statements in one function body or control-flow body.
The language pages list the bodies that supply statements.

A run relation connects two runs with equal normalized statements.
It is Type-1 when the canonical streams are also equal, and Type-2 otherwise.
Aposlop does not find Type-3 relations between statement runs.

Aposlop extends each run relation to the longest equal run.
For example, a copied loop inside two different functions produces one group with the two loops.

Both runs must meet their effective `min_lines` and `min_nodes` values.
Both runs must enable `statement_runs` and the applicable duplicate type.
Two runs do not match when they overlap in the same file.

Set `statement_runs = false` to disable run relations without changing block relations.
Language and extension tables can override this value.

Aposlop does not report a run relation in these conditions:

- A matched block pair contains the two runs.
- The two runs contain a matched block pair, and the remaining statements of a run have fewer than `min_nodes` named nodes.
- A larger run relation overlaps both runs.
- The run repeats one shorter unit of statements two or more times, such as a list of field assignments with the same shape.

Run instances and block instances do not join the same group.
