# Exclusions and ignore files

Aposlop applies ignore rules during traversal before file metadata and parsing work.

## Standard ignore behavior

Traversal uses the `ignore` crate with standard filters enabled.
Aposlop respects:

- `.gitignore`
- repository exclude rules
- global Git ignore rules
- parent ignore rules
- hidden-file filtering

Ignored supported files do not enter analysis or the cache.

## Configured exclusions

`core.exclude` contains target-relative paths:

```toml
[core]
exclude = ["fixtures/", "generated/", "third_party/"]
```

Directory exclusions skip the complete subtree.
File exclusions skip the selected path.

Aposlop rejects:

- absolute paths
- paths containing `..`
- paths that otherwise escape the target root

## Command-line exclusions

Repeat `--exclude` to define one replacement list:

```bash
aposlop . \
  --exclude fixtures/ \
  --exclude generated/
```

The command-line list replaces `core.exclude` for that run.

## Cache file

Add `.aposlop_cache` to the target project's `.gitignore`:

```gitignore
.aposlop_cache
```

Aposlop does not modify ignore files automatically.
