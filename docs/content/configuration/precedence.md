# Configuration precedence

Aposlop resolves every configuration field independently.
A language table can supply one field while the global section supplies another field.

## Resolution order

Aposlop applies configuration layers in this order:

1. Aposlop starts with the built-in default.
2. Aposlop applies the global `.aposlop.toml` value.
3. Aposlop applies the matching language table.
4. Aposlop applies the matching extension table.
5. Aposlop applies the command-line option.

A later layer wins over an earlier layer.
An omitted command-line option preserves the fully resolved configuration-file value.

## Example

```toml
[core]
min_lines = 5

[languages.typescript]
min_lines = 8

[extensions.tsx]
min_lines = 12
```

The resulting values are:

| File | Effective `min_lines` |
| --- | ---: |
| `module.ts` | `8` |
| `component.tsx` | `12` |
| `module.py` | `5` |

This command makes every supported file use `3`:

```bash
aposlop . --min-lines 3
```

## Boolean overrides

Boolean command-line options require an explicit value.
This lets `false` override a configured `true`.

```bash
aposlop . --type-2 false --calculate-complexity false
```

## Replacement lists

A `core.exclude` value replaces the built-in list.
One or more `--exclude` options replace the configured list.
Aposlop does not merge exclusion lists across layers.
