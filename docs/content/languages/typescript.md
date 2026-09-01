# TypeScript and TSX support

Aposlop analyzes `.ts` and `.tsx` files.
TSX parsing supports embedded JSX syntax.

Both extensions use the `typescript` configuration key.
Language overrides apply to both extensions.
Extension overrides can distinguish them.

## Blocks

Aposlop reports:

- function declarations
- generator function declarations
- method definitions
- function expressions
- generator functions
- arrow functions

## Type-2 normalization

Aposlop anonymizes identifiers, property identifiers, shorthand identifiers, and shorthand binding identifiers.

It also anonymizes:

- numbers
- strings
- template strings
- regular expressions
- `true`
- `false`
- `null`

Comments do not enter canonical token streams.

## Complexity decisions

Aposlop counts these decisions:

- `if` statements
- `for` statements
- `for in` and `for of` statements
- `while` statements
- `do` statements
- switch cases
- catch clauses
- ternary expressions
- `&&` operations
- `||` operations
- `??` operations

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
