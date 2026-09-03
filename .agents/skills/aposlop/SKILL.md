---
name: aposlop
description: Configure and use Aposlop to find duplicated code, excessive file length, and cyclomatic complexity in Go, Rust, Python, TypeScript, and TSX repositories. Use this skill whenever a user asks to detect code duplication, code clones, repeated agent-generated logic, code slop, oversized source files, or excessive complexity. Also use it when adding Aposlop to a repository, agent validation workflow, or CI process, and when investigating or resolving Aposlop findings.
---

# Aposlop

Use Aposlop as a source-quality check for duplicate code, excessive file length, and excessive cyclomatic complexity.
Aposlop supports Go, Rust, Python, TypeScript, and TSX.

## Select the command

Prefer an installed `aposlop` binary.
Run `aposlop --version` before the first analysis.

If the binary is unavailable and `uvx` exists, run Aposlop as `uvx aposlop` without a persistent installation.
Ask before installing a global tool unless the user already requested installation.

Persistent installation options are:

```bash
cargo install aposlop --locked
uv tool install aposlop
brew install EzyGang/tap/aposlop
```

Use the signed platform installers when the user prefers a native release.
Read [the installation guide](https://aposlop.ezygang.digital/getting-started/installation/) for current commands.

The examples below use `aposlop`.
Replace it with `uvx aposlop` when using the temporary installation.

## Analyze a repository

1. Read the repository instructions and existing quality configuration.
2. Locate an existing `.aposlop.toml`, `.aposlopignore`, and `.gitignore` before changing files.
3. Run a complete baseline report from the target root.
4. Inspect diagnostics and unused ignores before interpreting findings.
5. Refactor unintended duplication, oversized files, and excessive control flow at their source.
6. Remove obsolete ignore IDs only after confirming their findings no longer exist.
7. Run the complete report again.
8. Run `aposlop ci .` only after the complete report is clean or accepted.

Use a terminal code report for human investigation:

```bash
aposlop . --terminal-output code
```

Use JSON when an agent must inspect every finding reliably:

```bash
aposlop . --format json
```

Read `references/automation.md` before parsing JSON or depending on exit codes.

## Set up a repository

Create `.aposlop.toml` only when the project needs explicit, reviewable policy.
Aposlop already uses the values below as built-in defaults.

```toml
[core]
min_lines = 5
min_nodes = 30
exclude = ["tests/", "vendor/", "node_modules/", "target/"]
use_cache = true

[duplicates_detection]
type_1 = true
type_2 = true
type_3 = true
type_3_threshold = 0.85

[metrics]
calculate_complexity = true
complexity_threshold = 15

[file_length]
max_lines = 300
exclude = []
```

Add the cache file to the target repository's `.gitignore`:

```gitignore
.aposlop_cache
```

Keep `.aposlop.toml` and intentional `.aposlopignore` entries under version control.
Do not commit `.aposlop_cache`.

Read `references/configuration.md` before changing thresholds, exclusions, language settings, or extension settings.
An exclusion list replaces lower-precedence lists instead of extending them.
Exclusion values use the same pattern syntax as one `.gitignore` line.

## Set up agent validation

Add Aposlop to the repository's existing agent instruction file when the user requests agent setup.
Do not create a second instruction convention beside an existing one.

Use guidance equivalent to this text:

```markdown
After supported source changes, run `aposlop ci .` from the repository root.
If it reports findings, inspect them with `aposlop . --terminal-output code` or `aposlop . --format json`.
Refactor unintended duplication, oversized files, and excessive complexity before completion.
Use `aposlop allow <FINDING> .` only for reviewed duplicate or complexity findings.
```

Use `aposlop ci .` as the completion check because normal terminal and JSON reports do not fail for findings.
A CI exit code of `1` can also represent an operational failure.
Inspect the complete report when the concise CI output does not explain the failure.

## Resolve findings

Treat each finding as evidence to inspect, not an automatic refactoring order.

For duplicate groups:

- Type-1 means every connecting relation has identical canonical syntax.
- Type-2 means at least one connecting relation requires identifier or literal normalization.
- Type-3 means at least one connecting relation passed verified Jaccard similarity.
- The minimum similarity is the lowest accepted relation similarity in the group.
- A Type-3 group can contain instances that do not match each other directly.
- Compare every instance's behavior, ownership, and change cadence before extracting shared code.
- Prefer deleting accidental copies or reusing an existing owner.
- Do not create a shared abstraction when grouped blocks only look similar.

For complexity findings:

- A block violates the policy only when its score is greater than its effective threshold.
- Use guard clauses, separate responsibilities, or simpler state transitions when behavior permits.
- Preserve behavior and rerun the affected scenario before accepting the result.

Show source for every instance in each duplicate group:

```bash
aposlop . --terminal-output code
```

Use a one-run override to test a policy change before editing configuration:

```bash
aposlop . --min-lines 8 --type-3-threshold 0.90 --complexity-threshold 20
```

Boolean command-line options require explicit values such as `--type-2 false`.

## Accept intentional findings

Use manual acceptance only after the user or repository policy establishes that a finding is intentional.
Do not hide a finding merely to make validation pass.

```bash
aposlop allow <FINDING_ID> .
```

This command writes the deterministic finding ID to `.aposlopignore` without running analysis.
Rerun the complete report and `aposlop ci .` after adding the entry.

A valid ignore ID that matches no current finding appears in the final `Unused ignores` section.
Remove an unused ID after review instead of deleting ignore entries automatically.
Unused ignores are informational and do not change the CI exit code.

## Completion checks

For configuration or agent-setup changes, verify all of these outcomes:

1. `aposlop . --format json` completes without operational diagnostics that the change introduced.
2. The report covers the intended supported files.
3. `.aposlop_cache` is ignored.
4. No unused ignore remains without an explicit policy reason.
5. `aposlop ci .` returns the result required by repository policy.
6. Agent instructions name the same command and target path that were verified.

Do not claim the setup is clean when findings remain.
Report accepted findings separately from unresolved findings.
