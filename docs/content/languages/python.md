# Python support

Aposlop analyzes `.py` files.
Aposlop accepts Python 3.14 unparenthesized multi-exception handlers from [PEP 758](https://peps.python.org/pep-0758/).

## Blocks

Aposlop reports:

- Aposlop reports regular and asynchronous function definitions.
- Aposlop reports decorated functions.
- Aposlop reports class definitions.
- Aposlop reports decorated classes.
- Aposlop reports lambda expressions.

Aposlop reports classes and their methods as independent blocks.
Aposlop can also report nested valid blocks independently.

## Type-2 normalization

Aposlop anonymizes:

- Aposlop anonymizes identifiers.
- Aposlop anonymizes integers.
- Aposlop anonymizes floating-point values.
- Aposlop anonymizes strings.
- Aposlop anonymizes concatenated strings.
- Aposlop anonymizes `True`.
- Aposlop anonymizes `False`.
- Aposlop anonymizes `None`.
- Aposlop anonymizes `...`.

Comments do not enter the canonical stream.

## Complexity decisions

Aposlop counts these decisions:

- Aposlop counts `if` statements.
- Aposlop counts `elif` clauses.
- Aposlop counts `for` statements.
- Aposlop counts `while` statements.
- Aposlop counts assertions.
- Aposlop counts comprehension generators and filters.
- Aposlop counts `case` clauses.
- Aposlop counts guarded `case` clauses.
- Aposlop counts `except` clauses.
- Aposlop counts conditional expressions.
- Aposlop counts Boolean operators.

Every valid function, class, or lambda starts with complexity `1`.

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
