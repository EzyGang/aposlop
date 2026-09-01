# Aposlop documentation

Aposlop detects duplicated code and reports cyclomatic complexity across Rust, Python, TypeScript, and TSX projects.

It finds exact, renamed, and near-miss duplicates without comparing every block against every other block.

## Start here

- [Getting started](getting-started/index.md): Install Aposlop and analyze a project.
- [Configuration](configuration/index.md): Set global, language, and extension rules.
- [Duplicate results](concepts/duplicate-types.md): Understand Type-1, Type-2, and Type-3 findings.
- [Languages](languages/index.md): Review supported files and analysis behavior.
- [Output formats](operations/output.md): Use terminal or JSON reports.
- [CLI reference](reference/cli.md): Review every command-line option.
- [Under the hood](concepts/how-it-works.md): Learn how Tree-sitter, exact similarity joins, and Jaccard verification work together.

## Supported analysis

| Capability | Support |
| --- | --- |
| Languages | Rust, Python, TypeScript, and TSX |
| Duplicate classes | Type-1, Type-2, and verified Type-3 |
| Complexity | Cyclomatic complexity for function-like blocks |
| Output | Deterministic terminal and JSON reports |
| Cache | Versioned local cache with atomic replacement |
| Ignore handling | Standard ignore files and configured root-relative exclusions |

Aposlop reports findings without changing the successful exit code.
Operational and configuration failures return a non-zero exit code.
