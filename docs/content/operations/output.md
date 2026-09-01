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

Each duplicate block contains a finding ID, similarity, and two source locations.
Each location uses a navigable `path:line` anchor and a separate line range.
The terminal report does not display the internal duplicate classification.
Each complexity block contains a finding ID, score, threshold, and source location.

Use source-code output when you need to compare both ranges:

```bash
aposlop . --terminal-output code
```

Code output prints the left and right ranges with their original line numbers.
`--terminal-output` does not change JSON output.

## JSON output

Select JSON explicitly:

```bash
aposlop . --format json
```

Aposlop formats JSON output with indentation and one trailing newline.
Field names and duplicate type values remain stable within the report schema version.

See the [JSON report reference](../reference/json-report.md) for every field.

## CI output

Select concise CI output explicitly:

```bash
aposlop ci .
```

The `ci` command prints a pass or fail status and both finding counts.
It omits paths, source code, diagnostics, and aggregate scan statistics.
Aposlop returns exit code `1` when duplicates or complexity violations exist.
It returns exit code `0` when both finding counts are zero.

## Ordering

1. Aposlop sorts by the left path and line first.
2. Aposlop sorts by the right path and line second.
3. Aposlop sorts otherwise equal matches by internal duplicate type.

Aposlop sorts complexity violations by location and score.
Aposlop sorts diagnostics by path, category, and message.

Parallel worker order does not affect either output format.

## Manual exclusions

Every duplicate and complexity violation has a deterministic five-character finding ID.
The ID does not depend on scan order, duplicate type, score, or threshold.
It identifies the finding's relative source path and line range.

Allow a finding from the target directory:

```bash
aposlop allow aB7_x
```

Aposlop writes the ID to `.aposlopignore` in that target directory.
Blank lines and lines beginning with `#` are permitted.
Both terminal and JSON reports omit manually excluded duplicate and complexity findings.
Remove the ID from `.aposlopignore` to restore the finding.

Add `.aposlopignore` to the target repository's `.gitignore`.

## Paths

Every report path is relative to the target directory.
Aposlop does not expose absolute source paths in reports.
