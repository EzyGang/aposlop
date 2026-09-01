# Configuration

Aposlop loads `<PATH>/.aposlop.toml` when the file exists.
A missing file uses built-in defaults.

## Complete global configuration

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

Unknown sections and fields are errors.
A configuration failure stops analysis and returns exit code `1`.

## Language overrides

Supported language keys are `rust`, `python`, and `typescript`.
A TypeScript language table applies to `.ts` and `.tsx` files.

```toml
[languages.rust]
min_lines = 10

[languages.typescript]
min_lines = 6
complexity_threshold = 20
```

## Extension overrides

Supported extension keys are `rs`, `py`, `ts`, and `tsx`.
Do not include a leading dot.

```toml
[extensions.tsx]
min_lines = 15
min_nodes = 50
type_3_threshold = 0.90
```

Extension tables can set these fields:

- `min_lines`
- `min_nodes`
- `type_1`
- `type_2`
- `type_3`
- `type_3_threshold`
- `calculate_complexity`
- `complexity_threshold`

`exclude` and `use_cache` are global-only fields.

Review the [configuration reference](../reference/configuration.md) for types, defaults, and validation rules.
