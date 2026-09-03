# JSON report reference

Select JSON with `--format json`.
JSON output ends with one newline.

## Top-level fields

| Field | Meaning |
| --- | --- |
| `schema_version` | The report schema version is `6`. |
| `summary` | Aggregate file, block, duplicate-group, complexity, and file-length counts. |
| `duplicates` | Sorted duplicate groups. |
| `complexity` | Sorted complexity violations. |
| `file_length` | Sorted file-length violations. |
| `diagnostics` | Sorted recoverable diagnostics. |
| `unused_ignores` | Sorted valid ignore IDs that match no current finding. |

## Duplicate group fields

Each duplicate group contains these fields:

- `id` contains the deterministic finding ID accepted by `aposlop allow`.
- `kind` contains `type_1`, `type_2`, or `type_3`.
- `minimum_similarity` contains the lowest accepted relation similarity in the group.
- `instances` contains every source location in deterministic order.

The group kind is the broadest relation required to connect its instances.
Type-3 groups are connected components and do not guarantee that every instance pair meets the threshold.

Each location contains a path relative to the target directory and one-based line numbers.

## Complexity fields

Each complexity violation contains these fields:

- `id` contains the deterministic finding ID accepted by `aposlop allow`.
- `location` contains the source location.
- `score` contains calculated cyclomatic complexity.
- `threshold` contains the effective threshold for that block.

## File-length fields

Each file-length violation contains these fields:

- `path` contains the source path relative to the target directory.
- `lines` contains the physical source-file line count.
- `max_lines` contains the effective global, language, extension, or command-line limit.

File-length violations have no finding ID.
`file_length.exclude` is the only suppression that leaves the file available to other checks.

## Diagnostic fields

Each diagnostic contains these fields:

- `path` contains a source path or cache path relative to the target directory.
- `category` contains `analysis`, `cache`, or `ingestion`.
- `message` contains deterministic diagnostic text.

## Unused ignores

`unused_ignores` contains finding IDs from `.aposlopignore` that matched no duplicate group or complexity violation.
The array is empty when every configured ID suppresses a current finding.
Unused ignores are informational and do not change the process exit code.

## Example

```json
{
  "schema_version": 6,
  "summary": {
    "scanned_files": 3,
    "analyzed_blocks": 3,
    "duplicate_count": 1,
    "complexity_violation_count": 1,
    "file_length_violation_count": 1
  },
  "duplicates": [
    {
      "id": "aB7_x",
      "kind": "type_1",
      "minimum_similarity": 1.0,
      "instances": [
        {
          "path": "src/first.rs",
          "start_line": 1,
          "end_line": 6
        },
        {
          "path": "src/second.rs",
          "start_line": 1,
          "end_line": 6
        },
        {
          "path": "src/third.rs",
          "start_line": 1,
          "end_line": 6
        }
      ]
    }
  ],
  "complexity": [
    {
      "id": "p9_K2",
      "location": {
        "path": "src/complex.rs",
        "start_line": 8,
        "end_line": 24
      },
      "score": 18,
      "threshold": 15
    }
  ],
  "file_length": [
    {
      "path": "src/large.rs",
      "lines": 342,
      "max_lines": 300
    }
  ],
  "diagnostics": [],
  "unused_ignores": ["z9_Xq"]
}
```
