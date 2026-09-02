# Aposlop automation contracts

Read this reference before parsing JSON, enforcing findings, or diagnosing a nonzero exit.

## Choose the interface

Use the complete JSON report when automation needs finding details:

```bash
aposlop . --format json
```

Use the concise CI command when any remaining finding must fail validation:

```bash
aposlop ci .
```

The normal terminal and JSON reports return success after completed analysis even when findings exist.
The CI command returns failure when duplicate or complexity findings remain.
Unused ignores are informational and do not change CI status.

## Exit codes

| Code | Meaning |
| ---: | --- |
| `0` | Analysis completed, and CI mode found no reportable findings |
| `1` | Operational failure, or CI mode found reportable findings |
| `2` | Invalid command-line usage |

Do not interpret exit code `1` from CI mode as proof of a finding without reading its output.
An operational failure uses the same code.

## JSON report

The current JSON `schema_version` is `5`.
JSON output ends with one newline.

Top-level fields are:

| Field | Meaning |
| --- | --- |
| `schema_version` | Report contract version |
| `summary` | Aggregate file, block, duplicate-group, and complexity counts |
| `duplicates` | Sorted duplicate groups |
| `complexity` | Sorted complexity violations |
| `diagnostics` | Sorted recoverable diagnostics |
| `unused_ignores` | Sorted valid ignore IDs that match no current finding |

Each duplicate group has these fields:

- `id`: deterministic group ID accepted by `aposlop allow`
- `kind`: broadest connecting relation, as `type_1`, `type_2`, or `type_3`
- `minimum_similarity`: lowest accepted relation similarity in the group
- `instances`: every source location in deterministic order

Type-3 groups are connected components.
Do not assume that every instance pair in one Type-3 group meets the similarity threshold.
Each location contains a target-relative `path` and one-based `start_line` and `end_line` values.

Each complexity violation has these fields:

- `id`: deterministic ID accepted by `aposlop allow`
- `location`: source location
- `score`: calculated cyclomatic complexity
- `threshold`: effective threshold for that block

Each diagnostic has these fields:

- `path`: target-relative source or cache path
- `category`: `analysis`, `cache`, or `ingestion`
- `message`: deterministic diagnostic text

`unused_ignores` contains valid `.aposlopignore` IDs that matched no duplicate group or complexity violation.
Review and remove stale IDs instead of mutating the file automatically.

Check `schema_version` before consuming fields.
Treat a new schema version as a contract change instead of guessing its structure.

## Agent investigation loop

1. Run the JSON report from the configured target root.
2. Read every diagnostic and unused ignore before findings.
3. Remove stale ignore IDs only after confirming their findings no longer exist.
4. Sort finding work by ownership and source location, not by finding kind alone.
5. Inspect every group instance and its complete owning block.
6. Make the smallest behavior-preserving source change.
7. Run the affected behavior or focused checks.
8. Run the JSON report again.
9. Finish with `aposlop ci .` when repository policy requires no findings.

Do not parse the human terminal layout when JSON is available.
Do not infer that a Type-3 match has equivalent semantics.
Do not automatically allow findings that the agent cannot resolve.

## CI example

Use the repository's existing dependency installation method before this step.
Then run:

```bash
aposlop ci .
```

Keep the command target consistent with the directory that owns `.aposlop.toml` and `.aposlopignore`.
Set `APOSLOP_NO_UPDATE_CHECK=1` for explicit non-interactive environments when needed.
Aposlop already skips update checks for non-interactive commands.
