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

`core.exclude` contains paths relative to the target directory:

```toml
[core]
exclude = ["fixtures/", "generated/", "third_party/"]
```

Directory exclusions skip the complete subtree.
File exclusions skip the selected path.

Aposlop rejects these exclusion paths:

- Aposlop rejects absolute paths.
- Aposlop rejects paths containing `..`.
- Aposlop rejects paths that otherwise escape the target directory.

## Command-line exclusions

Repeat `--exclude` to define one replacement list:

```bash
aposlop . \
  --exclude fixtures/ \
  --exclude generated/
```

The command-line list replaces `core.exclude` for that run.

## Cache file

Add `.aposlop_cache` to the target directory's `.gitignore`:

```gitignore
.aposlop_cache
```

Aposlop does not modify ignore files automatically.
