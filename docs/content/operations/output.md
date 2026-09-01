# Output formats

Aposlop builds one sorted report before writing either output format.
Terminal and JSON output therefore describe the same report data.

## Terminal output

Terminal output is the default:

```bash
aposlop .
```

Terminal output contains these sections:

1. The `Duplicates` section lists duplicate matches.
2. The `Complexity` section lists complexity violations.
3. The `Diagnostics` section lists recoverable problems.
4. The `Summary` section lists aggregate counts.

Duplicate rows contain type, similarity, left location, and right location.
Complexity rows contain score, effective threshold, and location.

## JSON output

Select JSON explicitly:

```bash
aposlop . --format json
```

Aposlop formats JSON output with indentation and one trailing newline.
Field names and duplicate type values remain stable within the report schema version.

See the [JSON report reference](../reference/json-report.md) for every field.

## Ordering

Aposlop sorts duplicate matches in this order:

1. Aposlop sorts by duplicate type first.
2. Aposlop sorts by the left path and line second.
3. Aposlop sorts by the right path and line third.

Aposlop sorts complexity violations by location and score.
Aposlop sorts diagnostics by path, category, and message.

Parallel worker order does not affect either output format.

## Paths

Every report path is relative to the target directory.
Aposlop does not expose absolute source paths in reports.
