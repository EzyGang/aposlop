# Exit codes

Aposlop separates findings from command failures.
Duplicate and complexity findings do not make a successful analysis fail.

| Code | Meaning |
| ---: | --- |
| `0` | Analysis and report output completed. The report can contain findings or recoverable diagnostics. |
| `1` | Configuration, traversal, cache, analysis, or output failed. |
| `2` | Command-line usage was invalid. |

## Automation

Use exit status to detect whether Aposlop completed.
Read terminal or JSON report data to enforce project-specific finding policies.

Example:

```bash
aposlop . --format json > aposlop-report.json
```

A zero exit status means `aposlop-report.json` contains a complete report.
It does not mean the duplicate and complexity arrays are empty.
