# Aposlop documentation

Aposlop detects duplicate code and reports cyclomatic complexity across Rust, Python, TypeScript, and TSX projects.

Aposlop reports connected Type-1, Type-2, and Type-3 duplicate groups without unconditional all-pairs comparison.

## Start here

- [Getting started](getting-started/index.md) explains installation and the first analysis.
- [Configuration](configuration/index.md) explains global, language, and extension fields.
- [Duplicate types](concepts/duplicate-types.md) explains Type-1, Type-2, and Type-3 relations and grouping.
- [Languages](languages/index.md) describes supported files and analysis behavior.
- [Output formats](operations/output.md) describes terminal and JSON reports.
- [CLI reference](reference/cli.md) describes every command-line option.
- [Under the hood](concepts/how-it-works.md) explains Tree-sitter, exact similarity joins, and Jaccard verification.

## Supported analysis

| Capability | Support |
| --- | --- |
| Languages | Rust, Python, TypeScript, and TSX |
| Duplicate types | Type-1, Type-2, and Type-3 |
| Complexity | Cyclomatic complexity for function-like blocks |
| Output | Deterministic terminal and JSON reports |
| Cache | Versioned local cache with atomic replacement |
| Ignore handling | Standard ignore files and exclusions relative to the target directory |

Aposlop reports findings with exit code `0`.
Operational and configuration failures return a non-zero exit code.
