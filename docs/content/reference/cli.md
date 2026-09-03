# CLI reference

Aposlop scans one optional target directory, runs a CI finding check, or updates manual exclusions.

```text
aposlop [PATH] [OPTIONS]
aposlop ci [PATH] [OPTIONS]
aposlop allow <FINDING> [PATH]
```

`PATH` identifies the target directory and defaults to `.`.

## Output

| Option | Type | Default | Purpose |
| --- | --- | --- | --- |
| `--format <FORMAT>` | `terminal` or `json` | `terminal` | Select the report format. |
| `--terminal-output <OUTPUT>` | `locations` or `code` | `locations` | Select terminal duplicate detail. |

## Eligibility

| Option | Type | Purpose |
| --- | --- | --- |
| `--min-lines <N>` | Positive integer | Override `core.min_lines`. |
| `--min-nodes <N>` | Positive integer | Override `core.min_nodes`. |
| `--exclude <GLOB>` | Gitignore-style pattern | Replace `core.exclude` with one or more repeated patterns. |

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

## File length

| Option | Type | Purpose |
| --- | --- | --- |
| `--max-file-lines <N>` | Positive integer | Override every configured file-length limit. |

## CI command

`aposlop ci [PATH]` prints duplicate, complexity, and file-length finding counts.
It returns exit code `1` when any finding count is nonzero.
Analysis override options can follow the command.

```bash
aposlop ci ../project --exclude 'generated/'
```

## Allow command

`aposlop allow <FINDING> [PATH]` saves one duplicate or complexity finding ID in `<PATH>/.aposlopignore`.
`PATH` defaults to `.`.
The command does not run analysis.
File-length violations have no finding ID and cannot be used with this command.

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

Run a concise finding check for CI:

```bash
aposlop ci .
```

Disable Type-2 duplicate relations and the cache for one run:

```bash
aposlop . --type-2 false --use-cache false
```

Replace configured exclusions:

```bash
aposlop . --exclude 'generated/' --exclude 'fixtures/'
```

Allow a finding in another target:

```bash
aposlop allow aB7_x ../project
```

Aposlop reports invalid command-line usage and returns exit code `2`.
