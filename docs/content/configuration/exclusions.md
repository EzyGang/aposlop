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

`core.exclude` contains [Rust regular expressions](https://docs.rs/regex/latest/regex/#syntax).
Aposlop matches each expression against the complete root-relative path.
Aposlop normalizes path separators to `/` before matching.
Regular expressions use unanchored search unless they contain anchors such as `^` or `$`.

```toml
[core]
exclude = [
    "(^|/)fixtures(?:/|$)",
    "^generated/",
    "(^|/)third_party(?:/|$)",
]
```

A directory-boundary expression excludes the matching directory and its complete subtree.
An invalid regular expression stops configuration loading.

## Command-line exclusions

Repeat `--exclude` to define one replacement list:

```bash
aposlop . \
  --exclude '(^|/)fixtures(?:/|$)' \
  --exclude '^generated/'
```

The command-line list replaces `core.exclude` for that run.

## Cache file

Add `.aposlop_cache` to the target directory's `.gitignore`:

```gitignore
.aposlop_cache
```

Aposlop does not modify ignore files automatically.
