# Output formats

Aposlop builds one sorted report model before selecting a renderer.
Terminal and JSON output therefore describe the same findings.

## Terminal output

Terminal output is the default:

```bash
aposlop .
```

It contains these sections:

1. `Duplicates`
2. `Complexity`
3. `Diagnostics`
4. `Summary`

Duplicate rows contain type, similarity, left location, and right location.
Complexity rows contain score, effective threshold, and location.

## JSON output

Select JSON explicitly:

```bash
aposlop . --format json
```

JSON output is pretty-printed and ends with one newline.
Field names and finding type values are stable within the report schema version.

See the [JSON report reference](../reference/json-report.md) for every field.

## Ordering

Aposlop sorts duplicate matches by:

1. clone type
2. left path and line
3. right path and line

It sorts complexity violations by location and score.
It sorts diagnostics by path, category, and message.

Traversal and Rayon worker order do not affect either output format.

## Paths

Every report path is relative to the analyzed target root.
Aposlop does not expose absolute source paths in findings.
