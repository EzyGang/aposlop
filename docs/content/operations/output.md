# Output formats

Aposlop builds one sorted report before writing either output format.
Terminal and JSON output therefore describe the same report data.

## Terminal output

Terminal output is the default:

```bash
aposlop .
```

Terminal output contains these sections:

1. The `Duplicate groups` section lists connected duplicate groups.
2. The `Complexity` section lists complexity violations.
3. The `Diagnostics` section lists recoverable problems.
4. The `Summary` section lists aggregate counts.

Each duplicate group contains a finding ID, type, minimum accepted similarity, and two or more source instances.
Each instance uses a navigable `path:line` anchor and a separate line range.
Each complexity block contains a finding ID, score, threshold, and source location.

Use source-code output when you need to compare every instance:

```bash
aposlop . --terminal-output code
```

Code output prints every instance with its original line numbers.
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

1. Aposlop sorts instances within each group by path and line.
2. Aposlop sorts groups by their complete ordered instance lists.
3. Aposlop sorts otherwise equal groups by duplicate type.

Aposlop sorts complexity violations by location and score.
Aposlop sorts diagnostics by path, category, and message.

Parallel worker order does not affect either output format.

## Manual exclusions

Every duplicate group and complexity violation has a deterministic five-character finding ID.
A duplicate group ID depends on every relative source path and line range in the group.
Adding or removing a group instance changes its finding ID.

Allow a finding from the target directory:

```bash
aposlop allow aB7_x
```

Aposlop writes the ID to `.aposlopignore` in that target directory.
Blank lines and lines beginning with `#` are permitted.
Both terminal and JSON reports omit manually excluded duplicate groups and complexity findings.
Remove the ID from `.aposlopignore` to restore the finding.

Commit `.aposlopignore` when accepted findings are shared project policy.

## Paths

Every report path is relative to the target directory.
Aposlop does not expose absolute source paths in reports.
