# Diagnostics

Diagnostics describe recoverable file, traversal, and cache problems.
Diagnostics do not stop valid files.

## Categories

| Category | Meaning |
| --- | --- |
| `analysis` | Aposlop could not read, fully parse, or analyze a source file. |
| `cache` | Aposlop could not use existing cache data. |
| `ingestion` | Aposlop encountered a traversal or metadata failure. |

## Partial syntax trees

Tree-sitter root errors do not cause a process failure.
Aposlop analyzes captured blocks that contain no error or missing node.
Aposlop skips invalid blocks and emits one file diagnostic.

## File failures

Aposlop converts a file read or parser failure into an analysis diagnostic.
Aposlop continues detection and reporting for other supported files.

## Fatal errors

These conditions stop the command:

- Invalid command-line usage stops the command.
- Invalid configuration stops the command.
- Invalid embedded queries stop the command.
- A missing or non-directory target directory stops the command.
- A traversal startup failure stops the command.
- A cache write or atomic persistence failure stops the command.
- A report output failure stops the command.

Fatal operational errors identify the target directory and failed operation.

## Exit behavior

Recoverable diagnostics can appear with exit code `0`.
Fatal operational errors return exit code `1`.
Invalid command-line usage returns exit code `2`.
