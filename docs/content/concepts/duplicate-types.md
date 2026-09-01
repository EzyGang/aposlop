# Duplicate types

Aposlop classifies function-like blocks into three mutually exclusive duplicate types.
Every match contains two root-relative locations and a similarity value.

## Type-1

Type-1 blocks have equal canonical token streams.
Canonical tokens preserve identifiers and literals but ignore whitespace and comments.

Formatting-only and comment-only changes remain Type-1 matches.

## Type-2

Type-2 blocks have different canonical streams and equal normalized streams.
Normalization replaces provider-captured identifiers and literals with category markers.

Renaming variables or changing literal values can produce a Type-2 match.
Operator and control-flow changes alter the normalized stream.

## Type-3

Type-3 blocks have different normalized streams and verified Jaccard similarity at or above the effective threshold.

Aposlop builds five-token shingles, fixed MinHash signatures, and LSH buckets.
LSH only generates candidates.
A two-pointer Jaccard comparison verifies every reported candidate.

## Precedence

Aposlop classifies pairs in this order:

1. Type-1
2. Type-2
3. Type-3

A pair classified at an earlier level never appears at a later level.
Disabling an earlier type suppresses its report entry without reclassifying the pair.

## Pair eligibility

Both blocks must meet their effective `min_lines` and `min_nodes` values.
Both blocks must enable the reported duplicate type.

A Type-3 pair uses the larger effective threshold from its two blocks.
Blocks can match within one file, but a block never matches itself.

Aposlop compares blocks only when they use the same language provider.
TypeScript and TSX share the TypeScript provider and can match each other.
