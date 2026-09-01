# Exit codes

Aposlop separates operational failures from reportable findings.
Terminal and JSON output do not fail because findings exist.
CI output fails when duplicates or complexity violations exist.

| Code | Meaning |
| ---: | --- |
| `0` | Aposlop completed successfully, and CI output found no reportable findings. |
| `1` | Aposlop failed operationally, or CI output found duplicates or complexity violations. |
| `2` | Aposlop rejected invalid command-line usage. |

## Automation

Use CI output when findings must fail an automated check:

```bash
aposlop ci .
```

The command prints only its status, duplicate count, and complexity violation count.
It returns exit code `1` when either count is nonzero.

Use JSON output when automation needs complete report data:

```bash
aposlop . --format json > aposlop-report.json
```

JSON output returns exit code `0` after successful analysis even when findings exist.
