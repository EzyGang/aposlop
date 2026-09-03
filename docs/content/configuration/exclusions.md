# Exclusions and ignore files

Aposlop applies ignore rules during traversal before file metadata and parsing work.

## Standard ignore behavior

Aposlop enables standard ignore filters.
Aposlop applies these ignore sources:

- Aposlop reads `.gitignore`.
- Aposlop applies repository exclude rules.
- Aposlop applies global Git ignore rules.
- Aposlop applies parent ignore rules.
- Aposlop filters hidden files.

Ignore rules keep matching supported files out of analysis and the cache.

## Configured exclusions

`core.exclude` contains gitignore-style patterns.
Each array value uses the same syntax as one `.gitignore` line.

```toml
[core]
exclude = [
    "tests/",
    "/generated/",
    "**/fixtures/**",
]
```

`tests/` excludes directories with that name at any depth.
`/generated/` excludes only the target root's `generated` directory.
`**/fixtures/**` excludes files and directories below matching `fixtures` directories.

## Command-line exclusions

Repeat `--exclude` to define one replacement list:

```bash
aposlop . \
  --exclude 'tests/' \
  --exclude '**/fixtures/**'
```

The command-line list replaces `core.exclude` for that run.

## File-length exclusions

`file_length.exclude` suppresses only file-length violations.
Matching files still participate in duplicate detection and complexity analysis.

```toml
[file_length]
exclude = ["generated/", "**/fixtures/**"]
```

File-length violations cannot be suppressed through `aposlop allow` or `.aposlopignore`.

## Cache file

Add `.aposlop_cache` to the target directory's `.gitignore`:

```gitignore
.aposlop_cache
```

Aposlop does not modify ignore files automatically.
