# CLI reference

Aposlop uses one command with one optional positional target.

```text
aposlop [PATH] [OPTIONS]
```

`PATH` must identify a directory and defaults to `.`.

## Output

| Option | Type | Default | Purpose |
| --- | --- | --- | --- |
| `--format <FORMAT>` | `terminal` or `json` | `terminal` | Select the report renderer. |

## Eligibility

| Option | Type | Purpose |
| --- | --- | --- |
| `--min-lines <N>` | Positive integer | Override `core.min_lines`. |
| `--min-nodes <N>` | Positive integer | Override `core.min_nodes`. |
| `--exclude <PATH>` | Relative path | Replace `core.exclude`. Repeat for multiple paths. |

## Cache

| Option | Type | Purpose |
| --- | --- | --- |
| `--use-cache <BOOL>` | `true` or `false` | Override `core.use_cache`. |

## Duplicate detection

| Option | Type | Purpose |
| --- | --- | --- |
| `--type-1 <BOOL>` | `true` or `false` | Override `duplicates_detection.type_1`. |
| `--type-2 <BOOL>` | `true` or `false` | Override `duplicates_detection.type_2`. |
| `--type-3 <BOOL>` | `true` or `false` | Override `duplicates_detection.type_3`. |
| `--type-3-threshold <RATIO>` | Finite number from `0.0` through `1.0` | Override `duplicates_detection.type_3_threshold`. |

## Complexity

| Option | Type | Purpose |
| --- | --- | --- |
| `--calculate-complexity <BOOL>` | `true` or `false` | Override `metrics.calculate_complexity`. |
| `--complexity-threshold <N>` | Positive integer | Override `metrics.complexity_threshold`. |

## General options

| Option | Purpose |
| --- | --- |
| `-h`, `--help` | Print command help. |
| `-V`, `--version` | Print the Aposlop version. |

## Examples

Analyze the current directory:

```bash
aposlop
```

Analyze another directory as JSON:

```bash
aposlop ../project --format json
```

Disable Type-2 findings and the cache for one run:

```bash
aposlop . --type-2 false --use-cache false
```

Replace configured exclusions:

```bash
aposlop . --exclude generated/ --exclude fixtures/
```

Clap owns invalid usage diagnostics and exit code `2`.
