# Configuration precedence

Aposlop resolves every scalar rule independently.
One field can come from a language table while another comes from the global section.

## Resolution order

The layers apply in this order:

1. Built-in default
2. Global `.aposlop.toml` value
3. Matching language table
4. Matching extension table
5. Command-line override

A later layer wins over an earlier layer.
An omitted command-line field preserves the fully resolved configuration-file value.

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

Configured `core.exclude` replaces the built-in list.
One or more `--exclude` options replace the configured list.
Aposlop does not merge exclusion lists across layers.
