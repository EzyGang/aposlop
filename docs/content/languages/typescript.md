# TypeScript and TSX support

Aposlop analyzes `.ts` and `.tsx` files.
TSX parsing supports embedded JSX syntax.

Both extensions use the `typescript` configuration key.
Language overrides apply to both extensions.
Extension overrides can distinguish them.

## Blocks

Aposlop reports:

- Aposlop reports function declarations.
- Aposlop reports generator function declarations.
- Aposlop reports method definitions.
- Aposlop reports function expressions.
- Aposlop reports generator functions.
- Aposlop reports arrow functions.

## Type-2 normalization

Aposlop anonymizes identifiers, property identifiers, shorthand identifiers, and shorthand binding identifiers.

It also anonymizes:

- Aposlop anonymizes numbers.
- Aposlop anonymizes strings.
- Aposlop anonymizes template strings.
- Aposlop anonymizes regular expressions.
- Aposlop anonymizes `true`.
- Aposlop anonymizes `false`.
- Aposlop anonymizes `null`.

Comments do not enter the canonical stream.

## Complexity decisions

Aposlop counts these decisions:

- Aposlop counts `if` statements.
- Aposlop counts `for` statements.
- Aposlop counts `for in` and `for of` statements.
- Aposlop counts `while` statements.
- Aposlop counts `do` statements.
- Aposlop counts switch cases.
- Aposlop counts catch clauses.
- Aposlop counts ternary expressions.
- Aposlop counts `&&` operations.
- Aposlop counts `||` operations.
- Aposlop counts `??` operations.

## Configuration

```toml
[languages.typescript]
min_lines = 6
complexity_threshold = 20

[extensions.tsx]
min_lines = 15
min_nodes = 50
type_3_threshold = 0.90
```

The extension table wins over the language table for TSX files.
