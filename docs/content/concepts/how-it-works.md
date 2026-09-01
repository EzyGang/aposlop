# How Aposlop works

Aposlop combines syntax-aware analysis with deterministic candidate generation.
This page explains the underlying approach without requiring knowledge of the codebase.

```mermaid
flowchart LR
  files[Supported source files] --> parse[Tree-sitter parsing]
  parse --> tokens[Canonical and normalized tokens]
  tokens --> exact[Exact duplicate candidates]
  tokens --> shingles[Shingles and MinHash]
  shingles --> lsh[LSH candidates]
  exact --> verify[Equality verification]
  lsh --> jaccard[Jaccard verification]
  verify --> report[Sorted report]
  jaccard --> report
  parse --> complexity[Complexity captures]
  complexity --> report
```

## File discovery

Aposlop walks one target directory and respects standard ignore files.
It keeps only Rust, Python, TypeScript, and TSX files.

Discovery and analysis run in parallel.
Aposlop sorts data at stage boundaries so worker order never changes output.

## Syntax parsing

Tree-sitter parses each supported file.
Language-specific queries select function-like blocks, identifiers, literals, comments, and complexity decisions.

A file can contain syntax errors without failing the complete command.
Aposlop keeps valid blocks and reports one file diagnostic.

## Canonical and normalized tokens

Canonical tokens ignore whitespace and comments while preserving identifiers and literals.
Equal canonical streams produce Type-1 matches.

Normalized tokens replace identifiers and literals with category markers.
Equal normalized streams produce Type-2 matches when canonical streams differ.

## Near-miss candidates

Aposlop creates five-token shingles from normalized tokens.
It builds fixed MinHash signatures and groups similar signatures with Locality-Sensitive Hashing.

LSH does not prove a duplicate.
Aposlop verifies every candidate with exact Jaccard similarity before reporting Type-3.

This avoids comparing every block with every other block.

## Complexity

Language queries capture branches, loops, alternatives, exception paths, conditional expressions, and short-circuit Boolean operations.

Aposlop counts each syntax range once.
Every block starts with complexity `1`.

## Cache

The local cache stores analysis results for unchanged files.
Configuration thresholds are not cached, so report settings can change without reparsing unchanged source.

Cache entries include file metadata and analysis schema versions.
Aposlop replaces the cache atomically after successful output.

## Deterministic reports

Aposlop sorts files, blocks, duplicate matches, complexity violations, and diagnostics.
Terminal and JSON output use the same report data.
