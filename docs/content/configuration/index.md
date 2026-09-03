# Configuration

Aposlop loads `<target-directory>/.aposlop.toml` when the file exists.
Aposlop uses built-in defaults when the file does not exist.

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

[file_length]
max_lines = 300
exclude = []
```

Unknown sections and fields are errors.
A configuration failure stops analysis and returns exit code `1`.

## Language overrides

Aposlop supports the `go`, `rust`, `python`, and `typescript` language keys.
The `typescript` language table applies to `.ts` and `.tsx` files.

```toml
[languages.go]
min_lines = 8
max_file_lines = 350

[languages.rust]
min_lines = 10

[languages.typescript]
min_lines = 6
complexity_threshold = 20
```

## Extension overrides

Aposlop supports the `go`, `rs`, `py`, `ts`, and `tsx` extension keys.
Do not include a leading dot.

```toml
[extensions.tsx]
min_lines = 15
min_nodes = 50
type_3_threshold = 0.90
max_file_lines = 400
```

Extension tables can set these fields:

- `min_lines` sets the minimum line count.
- `min_nodes` sets the minimum named-node count.
- `type_1` enables Type-1 duplicate relations.
- `type_2` enables Type-2 duplicate relations.
- `type_3` enables Type-3 duplicate relations.
- `type_3_threshold` sets the Type-3 Jaccard threshold.
- `calculate_complexity` enables complexity violations.
- `complexity_threshold` sets the complexity threshold.
- `max_file_lines` sets the source-file line limit.

`core.exclude`, `file_length.exclude`, and `use_cache` are global-only fields.

Review the [configuration reference](../reference/configuration.md) for types, defaults, and validation rules.
