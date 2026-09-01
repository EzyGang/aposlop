# Aposlop

Aposlop detects duplicated code and reports cyclomatic complexity in Rust, Python, TypeScript, and TSX projects.

## Documentation

The complete user guide starts at [`docs/content/index.md`](docs/content/index.md).

Preview the documentation site locally:

```text
cd docs
uv sync --group dev
uv run zensical serve
```

## Installation

Build and install Aposlop from this checkout:

```text
cargo install --path .
```

Aposlop requires a Rust toolchain that supports edition 2024.

## Command line

```text
aposlop [PATH] [OPTIONS]
```

`PATH` is the directory to analyze.
It defaults to the current directory.

| Option | Meaning |
| --- | --- |
| `--format <terminal\|json>` | Select output format. The default is `terminal`. |
| `--min-lines <N>` | Override `core.min_lines`. |
| `--min-nodes <N>` | Override `core.min_nodes`. |
| `--exclude <PATH>` | Replace `core.exclude`. Repeat the option for multiple paths. |
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
One or more `--exclude` options form one replacement list.

## Supported files

Aposlop analyzes these extensions:

- Rust: `.rs`
- Python: `.py`
- TypeScript: `.ts`
- TSX: `.tsx`

Other extensions do not enter the analysis pipeline.
Aposlop respects standard ignore files, including `.gitignore`.

## Configuration

Aposlop loads `<PATH>/.aposlop.toml` when the file exists.
A missing file uses these defaults:

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

Configuration rejects unknown fields, zero count thresholds, invalid ratios, absolute excludes, and excludes containing `..`.
The similarity ratio must be finite and within `0.0..=1.0`.

### Language and extension overrides

Language tables accept `rust`, `python`, and `typescript` keys.
Extension tables accept `rs`, `py`, `ts`, and `tsx` keys without a leading dot.

Each table can override these scalar fields:

- `min_lines`
- `min_nodes`
- `type_1`
- `type_2`
- `type_3`
- `type_3_threshold`
- `calculate_complexity`
- `complexity_threshold`

The following example applies one rule to TypeScript files and a stronger rule to TSX files:

```toml
[languages.typescript]
min_lines = 6

[extensions.tsx]
min_lines = 15
min_nodes = 50
type_3_threshold = 0.90
```

Aposlop resolves each field independently in this order:

1. Built-in default
2. Global configuration section
3. Matching language table
4. Matching extension table
5. Command-line override

A later layer wins over an earlier layer.
An omitted command-line option preserves the resolved configuration value.

## Duplicate classification

A block qualifies only when it meets its effective minimum line and named-node counts.
Both blocks must enable a clone type before Aposlop reports that pair.

- Type-1 blocks have equal canonical token streams after whitespace and comments are removed.
- Type-2 blocks differ as Type-1 streams but match after identifiers and literals are anonymized.
- Type-3 blocks differ as normalized streams but meet the verified Jaccard similarity threshold.

Aposlop applies Type-1, Type-2, then Type-3 precedence.
It emits each block pair once.
TypeScript and TSX blocks can match because they share one language provider.

## Complexity

Cyclomatic complexity is one plus the unique provider-specific decisions inside a block.
Decisions include supported branches, loops, alternatives, exception branches, conditional expressions, and short-circuit Boolean operations.

Aposlop reports a violation only when the score is greater than `complexity_threshold`.
Set `calculate_complexity = false` to hide violations without invalidating cached analysis.

## Cache

Aposlop stores versioned analysis data in `<PATH>/.aposlop_cache` when caching is enabled.
A cache hit requires unchanged path, size, modification time, language, cache format, and analysis schema values.

Missing, stale, incompatible, or corrupt entries become cache misses.
A corrupt cache produces one diagnostic and is replaced after successful analysis.
Disabled caching performs no cache read or write.

Add `.aposlop_cache` to the project `.gitignore` and keep it ignored.

## Output

Terminal output contains deterministic duplicate, complexity, diagnostic, and summary sections.
JSON output contains `schema_version`, `summary`, `duplicates`, `complexity`, and `diagnostics` fields.
Both formats use the same sorted report data.
JSON output ends with one newline.

Source paths in reports are relative to `PATH`.
Partial syntax errors produce diagnostics and do not stop valid sibling blocks from being analyzed.

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Analysis and output completed. Findings do not change this code. |
| `1` | Configuration, traversal, cache, analysis, or output failed. |
| `2` | Command-line usage was invalid. |
