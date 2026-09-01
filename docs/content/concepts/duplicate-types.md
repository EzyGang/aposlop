# Duplicate types

Aposlop classifies block pairs into three mutually exclusive duplicate types.
Every duplicate match contains two locations relative to the target directory and one similarity value.

## Type-1

A Type-1 duplicate match contains blocks with equal canonical streams.
The canonical stream preserves identifiers and literals but ignores whitespace and comments.

Formatting-only and comment-only changes preserve a Type-1 duplicate match.

## Type-2

A Type-2 duplicate match contains different canonical streams and equal normalized streams.
Aposlop replaces captured identifiers and literals with category markers during normalization.

Renaming variables or changing literal values can produce a Type-2 duplicate match.
Operator and control-flow changes alter the normalized stream.

## Type-3

A Type-3 duplicate match contains different normalized streams and meets the effective Jaccard threshold.

Aposlop builds five-token shingles and runs an exact prefix-filtered similarity join.
A two-pointer Jaccard comparison verifies every surviving Type-3 candidate pair.

## Precedence

Aposlop checks duplicate types in this order:

1. Aposlop checks Type-1.
2. Aposlop checks Type-2.
3. Aposlop checks Type-3.

Aposlop does not report one block pair under multiple duplicate types.
Disabling an earlier duplicate type suppresses its duplicate match without reclassifying the block pair.

## Block eligibility

Both blocks must meet their effective `min_lines` and `min_nodes` values.
Both blocks must enable the applicable duplicate type.

A Type-3 duplicate match uses the larger effective threshold from its two blocks.
Blocks can match within one file, but a block never matches itself.

Aposlop compares blocks only when they use the same language identity.
TypeScript and TSX share one language identity and can match each other.
