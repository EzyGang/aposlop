# Go support

Aposlop analyzes `.go` files.

## Blocks

Aposlop reports:

- Aposlop reports function declarations.
- Aposlop reports method declarations.
- Aposlop reports function literals.

Nested function literals have independent duplicate and complexity results.

## Type-2 normalization

Aposlop anonymizes value, field, label, package, and type identifiers.

It also anonymizes:

- Aposlop anonymizes integer literals.
- Aposlop anonymizes floating-point literals.
- Aposlop anonymizes imaginary literals.
- Aposlop anonymizes rune literals.
- Aposlop anonymizes raw string literals.
- Aposlop anonymizes interpreted string literals.
- Aposlop anonymizes `true`.
- Aposlop anonymizes `false`.
- Aposlop anonymizes `nil`.

Line comments and block comments do not enter the canonical stream.

## Complexity decisions

Aposlop counts these decisions:

- Aposlop counts `if` statements, including each `else if` statement.
- Aposlop counts every `for` statement form.
- Aposlop counts non-default expression switch cases.
- Aposlop counts non-default type switch cases.
- Aposlop counts non-default `select` communication cases.
- Aposlop counts `&&` operations.
- Aposlop counts `||` operations.

Every valid function, method, or function literal starts with complexity `1`.
Nested function literals have independent complexity scores.

## Configuration

```toml
[languages.go]
min_lines = 6
complexity_threshold = 20

[extensions.go]
min_nodes = 25
type_3_threshold = 0.90
```
