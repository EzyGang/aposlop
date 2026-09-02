# Aposlop configuration

Read this reference when creating or modifying `.aposlop.toml`.
Aposlop rejects unknown fields and invalid values.

## Precedence

Aposlop resolves each configurable field independently.
Later layers override earlier layers in this order:

1. Built-in default
2. Global section
3. Matching language table
4. Matching extension table
5. Command-line option

An omitted command-line option preserves the resolved file value.
Boolean command-line options require an explicit `true` or `false` value.

`core.exclude` replaces the built-in exclusion list.
Repeated `--exclude` options replace the configured exclusion list for that run.
Preserve every required default exclusion when defining either replacement list.

## Global sections

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

| Field | Type | Default | Constraint |
| --- | --- | ---: | --- |
| `core.min_lines` | integer | `5` | Greater than zero |
| `core.min_nodes` | integer | `30` | Greater than zero |
| `core.exclude` | gitignore-style pattern array | standard build and dependency paths | Each value follows `.gitignore` pattern syntax |
| `core.use_cache` | Boolean | `true` | Global only |
| `duplicates_detection.type_1` | Boolean | `true` | None |
| `duplicates_detection.type_2` | Boolean | `true` | None |
| `duplicates_detection.type_3` | Boolean | `true` | None |
| `duplicates_detection.type_3_threshold` | number | `0.85` | Finite and within `0.0..=1.0` |
| `metrics.calculate_complexity` | Boolean | `true` | Controls reporting, not parsing |
| `metrics.complexity_threshold` | integer | `15` | Greater than zero |

A complexity violation requires `score > complexity_threshold`.
A score equal to the threshold does not violate the policy.

## Language and extension overrides

Supported language keys are `go`, `rust`, `python`, and `typescript`.
The `typescript` language key applies to TypeScript and TSX.

Supported extension keys are `go`, `rs`, `py`, `ts`, and `tsx`.
Do not include the leading dot in an extension key.

Language and extension tables accept these fields:

- `min_lines`
- `min_nodes`
- `type_1`
- `type_2`
- `type_3`
- `type_3_threshold`
- `calculate_complexity`
- `complexity_threshold`

Example:

```toml
[languages.typescript]
min_lines = 8
complexity_threshold = 20

[extensions.tsx]
min_lines = 12
type_3_threshold = 0.90
```

With this configuration, `.ts` files use `min_lines = 8` and `.tsx` files use `min_lines = 12`.
Other languages retain the global or built-in value.

## Traversal exclusions

Aposlop applies standard ignore sources before parsing:

- `.gitignore`
- Repository Git exclude rules
- Global Git ignore rules
- Parent ignore rules
- Hidden-file filtering

Configured exclusions use the same pattern syntax as one `.gitignore` line.
A directory pattern such as `tests/` matches that directory name at any depth.
Use `/tests/` for the target root only or `**/tests/**` for explicit recursive matching.

Add `.aposlop_cache` to `.gitignore` when the cache is enabled.
Aposlop does not change ignore files automatically.

## Policy selection

Start with built-in defaults and inspect a baseline report.
Change a threshold only when repository policy requires a different signal boundary.
Use language or extension overrides instead of weakening the policy for all source files.
Exclude generated, vendored, or irrelevant paths rather than accepted first-party findings.
Use `.aposlopignore` for reviewed individual findings that must remain visible as policy decisions.
