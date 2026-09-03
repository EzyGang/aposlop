# Supported languages

Aposlop analyzes five source-file extensions.

| Language | Extensions |
| --- | --- |
| Go | `.go` |
| Rust | `.rs` |
| Python | `.py` |
| TypeScript | `.ts` |
| TSX | `.tsx` |

Aposlop skips unsupported extensions before parsing.

## Language-aware analysis

Aposlop uses a separate Tree-sitter grammar and analysis query set for each language.
This design lets Aposlop recognize function forms, identifiers, literals, comments, and complexity decisions.

TypeScript and TSX share one language identity.
One duplicate group can contain TypeScript and TSX instances.

## Shared behavior

Every supported language follows the same report rules:

- Whitespace and comments do not change the canonical stream.
- Identifier and literal changes can produce Type-2 duplicate relations.
- Verified normalized similarity can produce Type-3 duplicate relations.
- Aposlop gives each function-like block one complexity score.
- Aposlop checks each source file against its effective file-length limit.
- Aposlop skips invalid blocks without stopping valid files.

Review the language pages for supported block forms and complexity decisions.
