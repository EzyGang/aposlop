# Getting started

Aposlop analyzes one directory per command.
The target defaults to the current directory.

## Basic workflow

1. [Install Aposlop](installation.md).
2. Add `.aposlop_cache` to the target project's `.gitignore`.
3. Run Aposlop from the project root.
4. Review duplicate, complexity, and diagnostic sections.
5. Add `.aposlop.toml` when the defaults need adjustment.

```bash
aposlop .
```

Use JSON when another tool consumes the report:

```bash
aposlop . --format json
```

## What Aposlop scans

Aposlop discovers `.rs`, `.py`, `.ts`, and `.tsx` files.
It respects standard ignore files and configured exclusions.
Unsupported extensions do not enter analysis or the cache.

Continue with the [quick start](quickstart.md) for a complete configuration example.
