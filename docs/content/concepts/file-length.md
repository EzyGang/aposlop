# File length

Aposlop reports supported source files that exceed their effective line limit.
The built-in limit is `300` lines.
A file with exactly `300` lines does not violate the default.
Blank and comment lines count toward the file total.

## Configuration

Set the global limit and check-specific exclusions in `.aposlop.toml`:

```toml
[file_length]
max_lines = 300
exclude = [
    "generated/",
    "**/fixtures/**",
]
```

`file_length.exclude` uses the same gitignore-style syntax as `core.exclude`.
These patterns suppress only file-length violations.
Matching files still participate in duplicate and complexity analysis.

Set language and extension limits with `max_file_lines`:

```toml
[languages.python]
max_file_lines = 500

[extensions.tsx]
max_file_lines = 400
```

The extension value wins over the language value.

Override every language and extension for one run:

```bash
aposlop . --max-file-lines 350
```

The command-line value has the highest precedence.

## Findings

Terminal and JSON reports include the path, observed line count, and effective maximum.
The CI command fails when one or more file-length violations remain.

File-length violations have no finding ID.
`aposlop allow` and `.aposlopignore` cannot suppress them.
Use `file_length.exclude` when project policy intentionally exempts a file.
