# Exit codes

Aposlop separates operational failures from reportable findings.
Terminal and JSON output do not fail because findings exist.
CI output fails when duplicate, complexity, or file-length violations exist.
Unused ignores are informational and do not change any exit code.

| Code | Meaning |
| ---: | --- |
| `0` | Aposlop completed successfully, and CI output found no reportable findings. |
| `1` | Aposlop failed operationally, or CI output found duplicate, complexity, or file-length violations. |
| `2` | Aposlop rejected invalid command-line usage. |

## Automation

Use CI output when findings must fail an automated check:

```bash
aposlop ci .
```

The command prints its status, duplicate count, complexity violation count, and unused-ignore count.
It returns exit code `1` when either finding count is nonzero.

Use JSON output when automation needs complete report data:

```bash
aposlop . --format json > aposlop-report.json
```

JSON output returns exit code `0` after successful analysis even when findings exist.
