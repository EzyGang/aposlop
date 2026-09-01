# JSON report reference

Select JSON with `--format json`.
JSON output ends with one newline.

## Top-level fields

| Field | Meaning |
| --- | --- |
| `schema_version` | The report schema version is `3`. |
| `summary` | Aggregate file, block, duplicate, and complexity counts. |
| `duplicates` | Sorted duplicate matches. |
| `complexity` | Sorted complexity violations. |
| `diagnostics` | Sorted recoverable diagnostics. |

## Duplicate fields

Each duplicate match contains these fields:

- `id` contains the deterministic finding ID accepted by `aposlop allow`.
- `kind` contains `type_1`, `type_2`, or `type_3`.
- `similarity` contains exact or verified Jaccard similarity.
- `left` contains the first source location.
- `right` contains the second source location.

Each location contains a path relative to the target directory and one-based line numbers.

## Complexity fields

Each complexity violation contains these fields:

- `id` contains the deterministic finding ID accepted by `aposlop allow`.
- `location` contains the source location.
- `score` contains calculated cyclomatic complexity.
- `threshold` contains the effective threshold for that block.

## Diagnostic fields

Each diagnostic contains these fields:

- `path` contains a source path or cache path relative to the target directory.
- `category` contains `analysis`, `cache`, or `ingestion`.
- `message` contains deterministic diagnostic text.

## Example

```json
{
  "schema_version": 3,
  "summary": {
    "scanned_files": 2,
    "analyzed_blocks": 2,
    "duplicate_count": 1,
    "complexity_violation_count": 1
  },
  "duplicates": [
    {
      "id": "aB7_x",
      "kind": "type_1",
      "similarity": 1.0,
      "left": {
        "path": "src/left.rs",
        "start_line": 1,
        "end_line": 6
      },
      "right": {
        "path": "src/right.rs",
        "start_line": 1,
        "end_line": 6
      }
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
  "diagnostics": []
}
```
