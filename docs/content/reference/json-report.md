# JSON report reference

Select JSON with `--format json`.
The document ends with one newline.

## Top-level fields

| Field | Meaning |
| --- | --- |
| `schema_version` | Report schema version. The current value is `1`. |
| `summary` | Aggregate file, block, duplicate, and complexity counts. |
| `duplicates` | Sorted duplicate matches. |
| `complexity` | Sorted complexity violations. |
| `diagnostics` | Sorted recoverable diagnostics. |

## Duplicate fields

Each duplicate contains:

- `kind`: `type_1`, `type_2`, or `type_3`
- `similarity`: exact or verified Jaccard similarity
- `left`: first source location
- `right`: second source location

Each location contains a root-relative `path`, one-based `start_line`, and one-based `end_line`.

## Complexity fields

Each violation contains:

- `location`: root-relative source location
- `score`: calculated cyclomatic complexity
- `threshold`: effective threshold for that block

## Diagnostic fields

Each diagnostic contains:

- `path`: root-relative source or cache path
- `category`: `analysis`, `cache`, or `ingestion`
- `message`: deterministic diagnostic text

## Example

```json
{
  "schema_version": 1,
  "summary": {
    "scanned_files": 2,
    "analyzed_blocks": 2,
    "duplicate_count": 1,
    "complexity_violation_count": 0
  },
  "duplicates": [
    {
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
  "complexity": [],
  "diagnostics": []
}
```
