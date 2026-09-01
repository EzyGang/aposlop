# Supported languages

Aposlop analyzes four source-file extensions.

| Language | Extensions |
| --- | --- |
| Rust | `.rs` |
| Python | `.py` |
| TypeScript | `.ts` |
| TSX | `.tsx` |

Unsupported extensions are skipped before parsing.

## Language-aware analysis

Each language uses its own Tree-sitter grammar and analysis queries.
This lets Aposlop recognize function forms, identifiers, literals, comments, and complexity decisions correctly.

TypeScript and TSX share one analysis identity.
They can match each other during duplicate detection while keeping separate extension configuration.

## Shared behavior

Every supported language follows the same report rules:

- whitespace and comments do not change Type-1 tokens
- identifier and literal changes can produce Type-2 matches
- verified normalized similarity can produce Type-3 matches
- function-like blocks receive one complexity score
- invalid blocks are skipped without stopping valid files

Review the language pages for supported block forms and complexity decisions.
