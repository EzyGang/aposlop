# Cache operations

Aposlop stores analyzed file data in `<target-directory>/.aposlop_cache` when the user enables caching.
Aposlop uses a versioned binary format.

## Cache identity

A cache hit requires these equal values:

- The path relative to the target directory must match.
- The file size must match.
- The modification time seconds must match.
- The modification time nanoseconds must match.
- The language identity must match.
- The cache format version must match.
- The analysis schema version must match.

A missing or stale entry becomes a cache miss.
Aposlop omits unmatched old entries from the next cache file.

## Cached data

Each cached file contains its physical line count.

Each cached block contains this analysis data:

- The cache stores the source location and line span.
- The cache stores the named-node count.
- The cache stores the canonical and normalized streams.
- The cache stores the exact and normalized hashes.
- The cache stores the sorted unique shingles.
- The cache stores the complexity score.

Aposlop does not cache eligibility thresholds or report records.
Reporting configuration changes do not invalidate unchanged analysis.

## Corruption recovery

Aposlop treats an incompatible or corrupt cache as an empty cache.
Aposlop emits one cache diagnostic and analyzes supported files normally.
Aposlop replaces the cache after analysis and report output succeed.

## Atomic persistence

Aposlop sorts cache entries by their paths relative to the target directory.
Aposlop writes a temporary file in the target directory and atomically replaces the cache path.

## Disable caching

Set this field to disable caching:

```toml
[core]
use_cache = false
```

Use this option to disable caching for one run:

```bash
aposlop . --use-cache false
```

When users disable caching, Aposlop performs no cache read or write.
