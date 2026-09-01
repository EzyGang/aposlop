# Getting started

Aposlop analyzes one target directory per command.
The target directory defaults to the current directory.

## Basic workflow

1. [Install Aposlop](installation.md).
2. Add `.aposlop_cache` to the target directory's `.gitignore`.
3. Run Aposlop from the target directory.
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
Aposlop respects standard ignore files and configured exclusions.
Aposlop keeps unsupported extensions out of analysis and the cache.

Continue with the [quick start](quickstart.md) for a complete configuration example.
