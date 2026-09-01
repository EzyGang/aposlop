# Supported languages

Aposlop analyzes four source-file extensions.

| Language | Extensions |
| --- | --- |
| Rust | `.rs` |
| Python | `.py` |
| TypeScript | `.ts` |
| TSX | `.tsx` |

Aposlop skips unsupported extensions before parsing.

## Language-aware analysis

Aposlop uses a separate Tree-sitter grammar and analysis query set for each language.
This design lets Aposlop recognize function forms, identifiers, literals, comments, and complexity decisions.

TypeScript and TSX share one language identity.
One duplicate match can contain a TypeScript block and a TSX block.

## Shared behavior

Every supported language follows the same report rules:

- Whitespace and comments do not change the canonical stream.
- Identifier and literal changes can produce Type-2 duplicate matches.
- Verified normalized similarity can produce Type-3 duplicate matches.
- Aposlop gives each function-like block one complexity score.
- Aposlop skips invalid blocks without stopping valid files.

Review the language pages for supported block forms and complexity decisions.
