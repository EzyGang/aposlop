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
3. The `File length` section lists excessive source-file lengths.
4. The `Diagnostics` section lists recoverable problems.
5. The `Summary` section lists aggregate counts.
6. The `Unused ignores` section lists configured IDs that match no current finding.

Each duplicate group contains a finding ID, type, minimum accepted similarity, and two or more source instances.
Each instance uses a navigable `path:line` anchor and a separate line range.
Each complexity block contains a finding ID, score, threshold, and source location.
Each file-length violation contains a path, line count, and effective maximum.

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

The `ci` command prints a status, three finding counts, and the unused-ignore count.
It omits paths, source code, diagnostics, and aggregate scan statistics.
Aposlop returns exit code `1` when duplicate, complexity, or file-length violations exist.
Unused ignores are informational and do not change the exit code.

## Ordering

1. Aposlop sorts instances within each group by path and line.
2. Aposlop sorts groups by their complete ordered instance lists.
3. Aposlop sorts otherwise equal groups by duplicate type.

Aposlop sorts complexity violations by location and score.
Aposlop sorts file-length violations by path and line count.
Aposlop sorts diagnostics by path, category, and message.
Aposlop sorts unused ignore IDs deterministically.

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
File-length violations have no finding ID and cannot be suppressed through `.aposlopignore`.
An unused ignore is a valid configured ID that matches no current finding.
This can occur after source changes, group membership changes, threshold changes, or disabled reporting.
The final report section lists unused ignores without modifying the file.
Remove the ID from `.aposlopignore` to restore the finding.

Commit `.aposlopignore` when accepted findings are shared project policy.

## Paths

Every report path is relative to the target directory.
Aposlop does not expose absolute source paths in reports.
