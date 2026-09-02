# Quick start

Create `.aposlop.toml` in the target directory:

```toml
[core]
min_lines = 5
min_nodes = 30
exclude = ["tests/", "vendor/", "node_modules/", "target/"]
use_cache = true

[duplicates_detection]
type_1 = true
type_2 = true
type_3 = true
type_3_threshold = 0.85

[metrics]
calculate_complexity = true
complexity_threshold = 15
```

Add the cache file to `.gitignore`:

```gitignore
.aposlop_cache
```

Run the terminal report:

```bash
aposlop .
```

Run the JSON report:

```bash
aposlop . --format json
```

Override selected values for one run:

```bash
aposlop . \
  --min-lines 8 \
  --type-3-threshold 0.90 \
  --calculate-complexity true \
  --complexity-threshold 20
```

A command-line value wins over global, language, and extension configuration.
An omitted option preserves the resolved configuration-file value.

## Read the result

The report contains four ordered sections:

1. The first section lists duplicate groups.
2. The second section lists complexity violations.
3. The third section lists file and cache diagnostics.
4. The fourth section lists summary counts.

Findings keep exit code `0`.
Use the report data rather than the process status to enforce project-specific thresholds.
