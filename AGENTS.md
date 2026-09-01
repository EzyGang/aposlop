# AGENTS.md — Aposlop

## Scope

- Applies repository-wide
- A closer `AGENTS.md` may add or override rules for its subtree
- Before editing, inspect the owning implementation, tests, configuration, and callers
- Reuse established patterns; do not introduce a second convention

## Project

Aposlop is a high-performance Rust CLI that detects duplicated code and reports cyclomatic complexity for Rust, Python, TypeScript, and TSX.

Pipeline order:

1. ingestion and filtering
2. optional cache resolution
3. Tree-sitter parsing
4. language-specific normalization and metric extraction
5. hashing, candidate generation, verification, and reporting

The core stays language-agnostic. Built-in languages belong behind `LanguageSupport`. Adding one must not change ingestion, caching, duplicate detection, metrics aggregation, or reporting algorithms.

## Workflow

1. Inspect the owning code, tests, configuration, and every affected caller.
2. Reproduce the defect or establish current observable behavior.
3. Make the smallest complete change.
4. Remove obsolete code, comments, aliases, imports, and re-exports.
5. Update tests only for changed behavior or an uncovered contract.
6. Run focused checks, then all applicable completion checks.
7. Report only completed work and observed results.

Done means behavior, callers, tests, configuration, help text, and user documentation agree. Do not leave compatibility aliases, deprecated paths, placeholders, `todo!()`, `unimplemented!()`, or deferred implementations unless explicitly requested.

## Ownership

- Application boundary: CLI parsing, diagnostics, process exit
- Configuration: defaults, TOML deserialization, validation, `core -> language -> extension` resolution
- Ingestion: traversal, ignore handling, extension filtering, file metadata
- Cache: identity, serialization, compatibility, reads, atomic writes
- Language providers: extensions, grammars, block extraction, normalization queries, complexity queries
- Exact-clone detection: normalized equality, fast-hash candidate groups
- Near-miss detection: shingles, MinHash signatures, LSH buckets, Jaccard verification
- Reporting: stable result models, terminal rendering, JSON rendering

Analysis code must not print. Filesystem, Tree-sitter, hashing, serialization, and terminal-formatting details stop at their owning boundaries. Domain APIs use typed contracts, not untyped maps or JSON values.

Each mutable value has one owner. Mutations go through that owner's methods. Avoid catch-all `manager`, `service`, `utils`, `helpers`, and `types` modules when a precise domain name exists.

## Performance

Performance changes require correctness checks and evidence from representative inputs.

- Keep traversal and analysis streaming or bounded
- Keep parallel work deterministic, bounded, and free of shared mutable hot spots
- Borrow, move, intern, or share source text, normalized nodes, shingles, signatures, and paths
- Avoid temporary `String`, `Vec`, map, and JSON allocations in hot loops
- Calculate normalization, hashes, signatures, and metrics once per unchanged block
- Never replace candidate generation with all-pairs comparison
- Generate benchmark fixtures deterministically
- Keep fixture generation, cache warm-up, and cleanup outside timed sections
- Compare benchmarks only with the same fixture, profile, machine, and cache state
- Preserve raw benchmark results

## Rust

Target stable Rust, edition 2024. `rustfmt` and Clippy define mechanical style.

### Correctness

- No `unsafe`
- No production `unwrap()`, `expect()`, `panic!()`, `todo!()`, or `unimplemented!()`
- Return narrow structured errors for expected domain failures
- Use `thiserror` for reusable domain errors
- Use `anyhow` only at the CLI boundary
- Return errors or add actionable boundary context; never swallow them
- Use full-path `tracing` macros for runtime diagnostics
- Do not suppress Clippy warnings without a targeted, documented external limitation

### Types and ownership

- Make invalid states unrepresentable with enums, newtypes, and validated constructors
- Return typed structs and enums instead of untyped collections
- Accept `&str` and `&[T]`, not `&String` and `&Vec<T>`
- Avoid `clone()` unless a distinct owner is required
- Convert `&str` with `.to_owned()`
- Put behavior on its owning type
- Put trait bounds in `where` clauses
- Use conventional generic names such as `S`, `Fut`, and `F`
- Add generic conversions only when callers need multiple input types
- Derive standard value traits when semantically valid
- Derive `Default` only for a genuine default
- Mark side-effect-free result functions `#[must_use]` when ignoring them is usually a bug
- Prefer `Self` inside implementations
- Do not add one-consumer abstractions unless they name an invariant or isolate an external boundary

### Control flow and imports

- Prefer guard clauses and early returns
- Use `match` when multiple variants have behavior
- Use `=> (),` for intentionally empty match arms
- Do not use glob imports or add `pub use` re-exports
- Declare modules first
- Group imports as `std`, external crates, `crate`, then local modules
- Use one `use` declaration per crate and blank lines between groups
- Prefer `crate::...` when it identifies the defining path clearly

### Files and modules

- Order files as public types, public functions, private types, implementations, private helpers
- Keep production code beside its owning domain
- Keep domain roots focused on declarations, primary public types, and the smallest cohesive boundary
- Group three or more files with one responsibility into a semantic submodule
- Remove redundant prefixes inside named submodules: `cache/entry.rs`, not `cache/cache_entry.rs`
- Do not create one-file directories, empty modules, or speculative nesting
- Keep production Rust files at or below 300 lines
- Generated files and declarative Tree-sitter queries are exempt
- Make module moves as clean cutovers; update all declarations and imports


## Testing

Test observable behavior, boundaries, invariants, transitions, precedence, determinism, and real failures. Do not test Rust, Serde, Tree-sitter, or hashing-library behavior in isolation.

Required coverage:

- configuration defaults and override precedence
- `.gitignore`, excludes, and extension filtering
- cache hits, misses, invalidation, corruption, disabling, and format versions
- Rust, Python, TypeScript, and TSX provider selection
- normalization and block extraction
- Type-1, Type-2, and verified Type-3 detection
- threshold boundaries and duplicate suppression
- complexity counting for every provider
- deterministic terminal and JSON output
- invalid input and partial parses without panics

Tests must be deterministic, isolated, and full-suite-safe. Use fixed MinHash seeds and controlled metadata in cache tests. Do not depend on checkout contents, network services, wall-clock timing, or thread order.

Keep test bodies out of production modules. Use a sibling `*_tests.rs` for a focused suite or one `<domain>/tests/` module for several files; never both.

## Validation

After Rust changes, run from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -- --help
```

For analysis or output changes, also run a focused CLI smoke scenario against a temporary fixture outside the repository. Verify terminal and JSON output when either contract changes.

# Output

Use ASD-STE100 Issue 9 as the writing guide.

- Use one term for one meaning.
- Use active voice unless the actor is unknown.
- Limit descriptive sentences to 25 words.
- Limit procedural sentences to 20 words and one instruction.
- Write each complete sentence on one Markdown line. Do not wrap a sentence manually.
- Use a vertical list when one sentence contains many items or actions.
- Give information in a general-to-specific order.
- Do not use contractions or semicolons.
- Use no more than six sentences in one paragraph.
- Write for readers who do not know this conversation.
- Do not include intermediate steps or breadcrumbs.
- Present final code, comments, and output without development history.