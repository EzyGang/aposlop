# Duplicate types

Aposlop classifies block relations into three mutually exclusive duplicate types.
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
Blocks can match within one file, but a block never matches itself.

Aposlop compares blocks only when they use the same language identity.
TypeScript and TSX share one language identity and can match each other.
