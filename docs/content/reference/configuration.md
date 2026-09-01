# Configuration reference

Aposlop deserializes `.aposlop.toml` into typed sections and rejects unknown fields.

## `[core]`

| Field | Type | Default | Validation |
| --- | --- | --- | --- |
| `min_lines` | Positive integer | `5` | Must be greater than zero. |
| `min_nodes` | Positive integer | `30` | Must be greater than zero. |
| `exclude` | Array of relative paths | `tests/`, `vendor/`, `node_modules/`, `target/` | Paths cannot be absolute or contain `..`. |
| `use_cache` | Boolean | `true` | Global only. |

## `[duplicates_detection]`

| Field | Type | Default | Validation |
| --- | --- | --- | --- |
| `type_1` | Boolean | `true` | Enables Type-1 report entries. |
| `type_2` | Boolean | `true` | Enables Type-2 report entries. |
| `type_3` | Boolean | `true` | Enables Type-3 report entries. |
| `type_3_threshold` | Number | `0.85` | Must be finite and within `0.0..=1.0`. |

## `[metrics]`

| Field | Type | Default | Validation |
| --- | --- | --- | --- |
| `calculate_complexity` | Boolean | `true` | Controls report entries, not analysis. |
| `complexity_threshold` | Positive integer | `15` | Must be greater than zero. |

## `[languages.<name>]`

Supported names:

- `rust`
- `python`
- `typescript`

Each language table accepts these optional scalar fields:

| Field | Type |
| --- | --- |
| `min_lines` | Positive integer |
| `min_nodes` | Positive integer |
| `type_1` | Boolean |
| `type_2` | Boolean |
| `type_3` | Boolean |
| `type_3_threshold` | Number within `0.0..=1.0` |
| `calculate_complexity` | Boolean |
| `complexity_threshold` | Positive integer |

## `[extensions.<extension>]`

Supported extensions are `rs`, `py`, `ts`, and `tsx`.
Use keys without a leading dot.
Extension tables accept the same fields as language tables.

## Complete layered example

```toml
[core]
min_lines = 5
min_nodes = 30
exclude = ["tests/", "vendor/"]
use_cache = true

[duplicates_detection]
type_1 = true
type_2 = true
type_3 = true
type_3_threshold = 0.85

[metrics]
calculate_complexity = true
complexity_threshold = 15

[languages.typescript]
min_lines = 8
complexity_threshold = 20

[extensions.tsx]
min_lines = 12
type_3_threshold = 0.90
```
