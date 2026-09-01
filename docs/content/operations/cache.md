# Cache operations

Aposlop stores analyzed file data in `<PATH>/.aposlop_cache` when caching is enabled.
The file uses a versioned binary format.

## Cache identity

A cache hit requires equal values for:

- root-relative path
- file size
- modification time seconds
- modification time nanoseconds
- language identity
- cache format version
- analysis schema version

A missing or stale entry becomes a cache miss.
Unmatched old entries are dropped from the next cache file.

## Cached data

Each cached analyzed block contains:

- source location and line span
- named-node count
- canonical and normalized streams
- exact and normalized hashes
- normalized token hashes
- sorted unique shingles
- MinHash signature
- complexity score

Eligibility thresholds and report records are not cached.
Changing reporting configuration does not invalidate unchanged analysis.

## Corruption recovery

An incompatible or corrupt cache becomes a cold cache.
Aposlop emits one cache diagnostic and analyzes supported files normally.
It replaces the cache after analysis and report output succeed.

## Atomic persistence

Aposlop serializes entries in root-relative path order.
It writes a temporary file in the target directory and atomically replaces the cache path.

## Disable caching

Configure:

```toml
[core]
use_cache = false
```

Or override one run:

```bash
aposlop . --use-cache false
```

Disabled caching performs no cache read or write.
