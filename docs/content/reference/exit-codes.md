# Exit codes

Aposlop separates findings from command failures.
Duplicate and complexity findings do not make a successful analysis fail.

| Code | Meaning |
| ---: | --- |
| `0` | Aposlop completed analysis and output with any findings or recoverable diagnostics. |
| `1` | Aposlop encountered a configuration, traversal, cache, analysis, or output failure. |
| `2` | Aposlop rejected invalid command-line usage. |

## Automation

Use exit status to detect whether Aposlop completed.
Read terminal or JSON report data to enforce project-specific duplicate and complexity policies.

The following command writes a JSON report:

```bash
aposlop . --format json > aposlop-report.json
```

A zero exit status means `aposlop-report.json` contains a complete report.
It does not mean the duplicate and complexity arrays are empty.
