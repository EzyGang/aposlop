# Diagnostics

Diagnostics describe recoverable file, traversal, and cache problems.
They do not stop valid files from completing.

## Categories

| Category | Meaning |
| --- | --- |
| `analysis` | A source file could not be read, fully parsed, or analyzed. |
| `cache` | Existing cache data could not be used. |
| `ingestion` | Traversal or metadata work failed for one path. |

## Partial syntax trees

A Tree-sitter root containing errors is not a process failure.
Aposlop analyzes captured blocks that contain no error or missing node.
It skips invalid blocks and emits one file diagnostic.

## File failures

A file read or parser failure becomes an analysis diagnostic.
Other supported files continue through detection and reporting.

## Fatal errors

These conditions stop the command:

- invalid command-line usage
- invalid configuration
- invalid embedded queries
- a missing or non-directory target
- traversal startup failure
- cache write or atomic persistence failure
- report output failure

Fatal operational errors include the target path and owning operation.

## Exit behavior

Recoverable diagnostics can appear with exit code `0`.
Fatal operational errors return exit code `1`.
Invalid command-line usage returns exit code `2`.
