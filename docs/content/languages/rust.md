# Rust support

Aposlop analyzes `.rs` files.

## Blocks

Aposlop reports free functions and methods as independent blocks.

## Type-2 normalization

The provider anonymizes:

- identifiers
- integer literals
- floating-point literals
- string literals
- raw string literals
- character literals
- Boolean literals

Line comments and block comments do not enter canonical token streams.

## Complexity decisions

Aposlop counts these decisions:

- `if` expressions
- `while` expressions
- `for` expressions
- `loop` expressions
- match arms
- `&&` operations
- `||` operations

Every valid function starts with complexity `1`.

## Configuration

Use `languages.rust` for every Rust file:

```toml
[languages.rust]
min_lines = 8
complexity_threshold = 20
```

Use `extensions.rs` for an extension-specific layer:

```toml
[extensions.rs]
type_3_threshold = 0.90
```
