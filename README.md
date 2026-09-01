# Aposlop

Aposlop detects duplicate code and reports cyclomatic complexity in Rust, Python, TypeScript, and TSX projects.

## Requirements

Aposlop requires a stable Rust toolchain that supports edition 2024.

## Installation

Install Aposlop from this checkout:

```text
cargo install --path .
```

Confirm the installation:

```text
aposlop --version
aposlop --help
```

## Documentation

The complete user guide starts at [`docs/content/index.md`](docs/content/index.md).

Preview the documentation site:

```text
cd docs
uv sync --group dev
uv run zensical serve
```

## Command line

```text
aposlop [PATH] [OPTIONS]
aposlop allow <FINDING> [PATH]
```

`PATH` identifies the target directory.
The target directory defaults to the current directory.

| Option | Purpose |
| --- | --- |
| `--format <terminal\|json\|ci>` | Select terminal, JSON, or CI output with `terminal` as the default. |
| `--terminal-output <locations\|code>` | Select duplicate locations or source excerpts for terminal output. |
| `--min-lines <N>` | Override `core.min_lines`. |
| `--min-nodes <N>` | Override `core.min_nodes`. |
| `--exclude <PATH>` | Replace `core.exclude` with one or more repeated options. |
| `--use-cache <BOOL>` | Override `core.use_cache` with `true` or `false`. |
| `--type-1 <BOOL>` | Override `duplicates_detection.type_1`. |
| `--type-2 <BOOL>` | Override `duplicates_detection.type_2`. |
| `--type-3 <BOOL>` | Override `duplicates_detection.type_3`. |
| `--type-3-threshold <RATIO>` | Override `duplicates_detection.type_3_threshold`. |
| `--calculate-complexity <BOOL>` | Override `metrics.calculate_complexity`. |
| `--complexity-threshold <N>` | Override `metrics.complexity_threshold`. |
| `-h`, `--help` | Print command help. |
| `-V`, `--version` | Print the Aposlop version. |

Boolean options require an explicit `true` or `false` value.
Repeated `--exclude` options form one replacement list.

## Supported files

| Language | Extension |
| --- | --- |
| Rust | `.rs` |
| Python | `.py` |
| TypeScript | `.ts` |
| TSX | `.tsx` |

Aposlop skips unsupported extensions.
Aposlop respects standard ignore files, including `.gitignore`.

## Configuration

Aposlop loads `<PATH>/.aposlop.toml` from the target directory.
Aposlop uses built-in defaults when this file does not exist.

```toml
[core]
min_lines = 5
min_nodes = 30
exclude = ["tests/", "vendor/", "node_modules/", "target/"]
use_cache = true

[duplicates_detection]
type_1 = true
type_2 = true
type_3 = true
type_3_threshold = 0.85

[metrics]
calculate_complexity = true
complexity_threshold = 15
```

Configuration rejects unknown fields, zero count thresholds, invalid ratios, absolute exclusions, and exclusions containing `..`.
The similarity ratio must be finite and within `0.0..=1.0`.

See the [configuration guide](docs/content/configuration/index.md) for language overrides, extension overrides, and precedence.

## Duplicate types

A block qualifies when it meets its effective minimum line count and named-node count.
Both blocks must enable a duplicate type before Aposlop reports a duplicate match.

- A Type-1 duplicate match contains equal canonical streams.
- A Type-2 duplicate match contains different canonical streams and equal normalized streams.
- A Type-3 duplicate match meets the verified Jaccard similarity threshold.

Aposlop applies Type-1, Type-2, and Type-3 precedence.
Aposlop reports each duplicate match once.
TypeScript and TSX blocks can form one duplicate match.

## Complexity

Aposlop calculates cyclomatic complexity as one plus the unique decisions inside each block.
Decisions include branches, loops, alternatives, exception branches, conditional expressions, and short-circuit Boolean operations.

Aposlop reports a violation when the score exceeds `complexity_threshold`.
Set `calculate_complexity = false` to hide complexity violations.

## Manual exclusions

Each duplicate and complexity finding has a deterministic five-character ID.
Save any finding ID in the target directory's manual exclusions file:

```text
aposlop allow aB7_x .
```

Aposlop stores allowed IDs in `<PATH>/.aposlopignore`.
Each non-comment line contains one manually excluded finding ID.
Subsequent scans omit matching duplicate and complexity findings from all reports.
Remove an ID from `.aposlopignore` to report that finding again.

Keep `.aposlopignore` local and add it to `.gitignore`.

## Cache

Aposlop stores versioned analysis data in `<PATH>/.aposlop_cache`.
A cache hit requires unchanged file metadata, language identity, cache format, and analysis schema values.

Aposlop treats missing, stale, incompatible, or corrupt entries as cache misses.
Aposlop reports corrupt cache data and replaces the cache after successful analysis.
Disabled caching performs no cache read or write.

Add `.aposlop_cache` to `.gitignore`.
Keep `.aposlop_cache` ignored.

## Output formats

Terminal output contains structured duplicate, complexity, diagnostic, and summary sections.
Duplicate locations use `path:line` anchors and show their complete line ranges.
Use `--terminal-output code` to print both source ranges with line numbers.

JSON output contains `schema_version`, `summary`, `duplicates`, `complexity`, and `diagnostics`.
Each JSON duplicate and complexity violation includes the same ID used by `aposlop allow`.
Aposlop sorts the shared report data before either output format writes it.
JSON output ends with one newline.

CI output contains only duplicate and complexity counts.
`--format ci` returns exit code `1` when either count is nonzero.

Reports use paths relative to the target directory.
Partial syntax errors produce diagnostics while valid sibling blocks continue through analysis.

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Aposlop completed successfully, and CI output found no reportable findings. |
| `1` | Aposlop failed operationally, or CI output found duplicates or complexity violations. |
| `2` | Command-line usage was invalid. |
