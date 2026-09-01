# Python support

Aposlop analyzes `.py` files.

## Blocks

Aposlop reports:

- function definitions
- lambda expressions

Nested valid blocks can be reported independently.

## Type-2 normalization

Aposlop anonymizes:

- identifiers
- integers
- floating-point values
- strings
- concatenated strings
- `True`
- `False`
- `None`

Comments do not enter canonical token streams.

## Complexity decisions

Aposlop counts these decisions:

- `if` statements
- `elif` clauses
- `for` statements
- `while` statements
- `case` clauses
- `except` clauses
- conditional expressions
- Boolean operators

Every valid function or lambda starts with complexity `1`.

## Partial files

A syntax error in one area does not fail the complete run.
Aposlop keeps valid function-like blocks that contain no error or missing node.
It skips invalid blocks and emits one deterministic file diagnostic.

## Configuration

```toml
[languages.python]
min_lines = 6
min_nodes = 25

[extensions.py]
complexity_threshold = 18
```
