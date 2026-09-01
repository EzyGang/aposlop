# Rust support

Aposlop analyzes `.rs` files.

## Blocks

Aposlop reports free functions, methods, and closures as independent blocks.

## Type-2 normalization

Aposlop anonymizes these Rust values:

- Aposlop anonymizes value, field, shorthand-field, and type identifiers.
- Aposlop anonymizes integer literals.
- Aposlop anonymizes floating-point literals.
- Aposlop anonymizes string literals.
- Aposlop anonymizes raw string literals.
- Aposlop anonymizes character literals.
- Aposlop anonymizes Boolean literals.

Line comments and block comments do not enter the canonical stream.

## Complexity decisions

Aposlop counts these decisions:

- Aposlop counts `if` expressions.
- Aposlop counts `while` expressions.
- Aposlop counts `for` expressions.
- Aposlop counts `loop` expressions.
- Aposlop counts match arms.
- Aposlop counts `?` expressions.
- Aposlop counts `&&` operations.
- Aposlop counts `&&` operations in let-chains.
- Aposlop counts `||` operations.

Every valid function or closure starts with complexity `1`.

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
