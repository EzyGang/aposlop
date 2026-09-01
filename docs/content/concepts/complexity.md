# Cyclomatic complexity

Aposlop calculates complexity for every analyzed block during parsing.
Reporting configuration does not change cached analysis data.

## Score

The score is:

```text
1 + unique complexity captures inside the block
```

Aposlop deduplicates captures by syntax-node byte range.
This prevents overlapping query patterns from counting one decision twice.

Providers capture supported decisions such as:

- branches
- loop decisions
- match or case alternatives
- exception branches
- conditional expressions
- short-circuit Boolean operations

See each [language page](../languages/index.md) for provider-specific forms.

## Violations

A violation requires:

```text
score > complexity_threshold
```

A score equal to the threshold is not a violation.
Set `metrics.calculate_complexity = false` to hide violations.
This setting does not invalidate existing cache entries.

## Per-file rules

Language and extension tables can set independent complexity behavior.
Command-line values override all configuration-file layers.

For example:

```toml
[metrics]
complexity_threshold = 15

[languages.typescript]
complexity_threshold = 20

[extensions.tsx]
calculate_complexity = false
```

The example reports TypeScript violations above `20` and hides TSX violations.
