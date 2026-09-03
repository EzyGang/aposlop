# Aposlop

<p align="center">
<a href="https://aposlop.ezygang.digital/">
    <img src="docs/content/images/icon.png" width="320" alt="Aposlop">
</a>
</p>

<p align="center">
<a href="https://github.com/EzyGang/aposlop/actions/workflows/ci.yml">
    <img src="https://github.com/EzyGang/aposlop/actions/workflows/ci.yml/badge.svg" alt="CI status">
</a>
<a href="https://crates.io/crates/aposlop">
    <img src="https://img.shields.io/crates/v/aposlop" alt="crates.io version">
</a>
<a href="https://pypi.org/project/aposlop/">
    <img src="https://img.shields.io/pypi/v/aposlop" alt="PyPI version">
</a>
<a href="https://github.com/EzyGang/aposlop/blob/main/LICENSE-MIT">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License">
</a>
</p>

---

<h3 align="center">Apos, the Slopbreaker</h3>

<p align="center">
  Slop always rolls downhill. Aposlop helps to push back.
</p>

**Aposlop** is a fast command-line tool.
It finds duplicate code, excessive file length, and cyclomatic complexity.

Aposlop supports Go, Rust, Python, TypeScript, and TSX.
Aposlop uses Tree-sitter to parse each supported language.

**Documentation**: https://aposlop.ezygang.digital/

**Source Code**: https://github.com/EzyGang/aposlop

**Issues**: https://github.com/EzyGang/aposlop/issues

---

## Table of Contents

- [Why Aposlop?](#why-aposlop)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Features](#core-features)
  - [Duplicate Detection](#duplicate-detection)
  - [Cyclomatic Complexity](#cyclomatic-complexity)
  - [File Length](#file-length)
  - [Language Support](#language-support)
  - [Output Formats](#output-formats)
  - [Manual Exclusions](#manual-exclusions)
- [Configuration](#configuration)
- [CLI Quick Reference](#cli-quick-reference)
- [Contributing](#contributing)
- [License](#license)

---

## Why Aposlop?

Aposlop exists to find code slop from coding agents.

Coding agents can generate duplicate code, complex control flow, and oversized source files.
These changes can enter a project faster than a reviewer can find them.

Text comparison does not find a duplicate after an agent renames identifiers or changes literals.
A linter does not find repeated logic in different files or languages.
A complexity limit does not identify repeated logic or growing file responsibilities.

Aposlop uses exact, normalized, and verified near-miss duplicate detection.
It calculates cyclomatic complexity for each code block.
It also reports supported source files that exceed their configured line limit.

Aposlop is fast enough for an agent validation loop.
You can configure thresholds for each language and file extension.
You can exclude paths or accept duplicate and complexity findings.
These controls let a project accept known findings and continue development.

| Feature             | Result                                                  |
| ------------------- | ------------------------------------------------------- |
| Type-1 detection    | Finds exact duplicates                                  |
| Type-2 detection    | Finds duplicates after identifier or literal changes    |
| Type-3 detection    | Finds verified near-miss duplicates                     |
| Complexity analysis | Calculates complexity for each code block               |
| File-length check   | Reports supported files above their effective limit      |
| Terminal code view  | Shows every group instance with line numbers            |
| JSON output         | Provides stable data for other tools                    |
| CI command          | Returns failure when findings remain                    |
| Local cache         | Reuses analysis for unchanged files                     |
| Update check        | Warns interactive users when a new release is available |

<details>
<summary><strong>How Aposlop works</strong></summary>

- **Parse once.** Tree-sitter providers extract blocks, identifiers, literals, comments, and complexity decisions.
- **Type-1.** XXH3 groups exact token streams before complete equality verification.
- **Type-2.** The same process compares streams with normalized identifiers and literals.
- **Type-3.** Five-token shingles feed a prefix-filtered similarity join.
  Length and position filters prune candidates before Jaccard verification.
- **Grouping.** Connected duplicate relations become one deterministic group with every source instance.
- **Complexity.** Each block starts at `1` and adds unique language-specific decision captures.
  Nested blocks have independent scores.
- **File length.** Each supported file keeps one physical line count for threshold reporting.
- **Cache.** Versioned bincode data reuses unchanged analysis.
  File metadata and schema versions invalidate stale entries before atomic replacement.

</details>

---

## Installation

### Install script on Linux or macOS

Install [cosign](https://docs.sigstore.dev/cosign/system_config/installation/).
Then run the installer:

```bash
curl -fsSLo install.sh https://github.com/EzyGang/aposlop/releases/latest/download/install.sh
sh install.sh
```

### Install script on Windows

Install [cosign](https://docs.sigstore.dev/cosign/system_config/installation/).
Then run the installer:

```powershell
Invoke-WebRequest https://github.com/EzyGang/aposlop/releases/latest/download/install.ps1 -OutFile install.ps1
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

Both scripts verify the archive checksum and Sigstore signatures.

### Cargo

Install Aposlop from crates.io:

```bash
cargo install aposlop --locked
```

### PyPI with uv

Install Aposlop as a global tool:

```bash
uv tool install aposlop
```

Run Aposlop without installing it:

```bash
uvx aposlop --help
```

### Homebrew

Install Aposlop from the EzyGang Homebrew tap:

```bash
brew install EzyGang/tap/aposlop
```

### From source

Install Aposlop from source.
A stable Rust toolchain must support edition 2024.

```bash
git clone https://github.com/EzyGang/aposlop.git
cd aposlop
cargo install --path . --locked
```

Verify the installation:

```bash
aposlop --version
aposlop --help
```

### Agent skills

Install all Aposlop skills for supported coding agents:

```bash
npx skills@latest add EzyGang/aposlop \
  --skill aposlop \
  --skill aposlop-code-changes \
  --skill aposlop-deslop-tests
```

The `aposlop` skill teaches agents to configure Aposlop, inspect findings, and add Aposlop to validation workflows.
The `aposlop-code-changes` skill helps agents desplop code through small, reuse-first changes that fix shared root causes.
The `aposlop-deslop-tests` skill removes low-value tests and then simplifies production seams that only those tests required.

The first two skills can activate automatically from the request context.
The `aposlop-deslop-tests` skill declares manual-only activation.
Invoke `/aposlop-deslop-tests` manually when the agent supports slash commands.
For other agents, select `aposlop-deslop-tests` through their skill interface.
The skills do not install the Aposlop binary.

### Update checks

Aposlop checks for a new GitHub release during interactive runs.
It performs a network request at most once every 24 hours.
It stores the latest result in the user cache directory.
Non-interactive commands do not perform this check.

Set this environment variable to disable the check:

```bash
APOSLOP_NO_UPDATE_CHECK=1 aposlop .
```

---

## Quick Start

Analyze the current directory:

```bash
aposlop .
```

Show the source for each duplicate:

```bash
aposlop . --terminal-output code
```

Run CI validation:

```bash
aposlop ci .
```

The `ci` command returns exit code `1` when a finding remains.

Write the complete report as JSON:

```bash
aposlop . --format json > aposlop-report.json
```

[Read the full quick-start guide](https://aposlop.ezygang.digital/getting-started/quickstart/).

---

## Core Features

### Duplicate Detection

A block enters analysis when it meets the line and named-node limits.

Aposlop classifies block relations in this order:

1. Type-1 requires identical canonical syntax.
2. Type-2 allows different identifiers and literals.
3. Type-3 requires a Jaccard similarity at or above the configured threshold.

Aposlop reports each connected set of duplicate relations as one group.
Two blocks do not match when either contains the other in the same file.
A TypeScript block can match a TSX block.

[Read the duplicate model](https://aposlop.ezygang.digital/concepts/duplicate-types/).

### Cyclomatic Complexity

Each valid block has an initial complexity score of `1`.
Aposlop adds one for each language-specific decision.
Decisions include branches, loops, alternatives, exception paths, conditional expressions, and short-circuit operations.

A nested block has an independent score.
Nested blocks include functions, closures, lambdas, field initializers, and static blocks.

A violation requires:

```text
score > complexity_threshold
```

[Read the complexity model](https://aposlop.ezygang.digital/concepts/complexity/).

### File Length

Aposlop reports a supported source file when its line count exceeds its effective maximum.
The default maximum is `300` lines.

```text
lines > max_file_lines
```

Use `[file_length].exclude` for check-specific gitignore-style exclusions.
File-length violations cannot be suppressed with `aposlop allow`.

[Read the file-length guide](https://aposlop.ezygang.digital/concepts/file-length/).

### Language Support

| Language   | Extensions |
| ---------- | ---------- |
| Go         | `.go`      |
| Rust       | `.rs`      |
| Python     | `.py`      |
| TypeScript | `.ts`      |
| TSX        | `.tsx`     |

Aposlop ignores unsupported extensions.
Aposlop follows standard ignore files such as `.gitignore`.

[Read the language guides](https://aposlop.ezygang.digital/languages/).

### Output Formats

The terminal report is the default.
It contains duplicate groups, complexity findings, file-length violations, diagnostics, and a summary.

```bash
aposlop . --format terminal
```

The JSON report contains the complete report and its schema version.

```bash
aposlop . --format json
```

The `ci` command shows only the status and finding counts.

```bash
aposlop ci .
```

[Read the output guide](https://aposlop.ezygang.digital/operations/output/).

### Manual Exclusions

Aposlop assigns a deterministic five-character ID to each duplicate group or complexity finding.

Add a finding to the manual exclusions:

```bash
aposlop allow aB7_x
```

The command writes the ID to `.aposlopignore`.
Delete the ID from that file to restore the finding.
Aposlop reports valid IDs that match no current finding as unused ignores at the end of each report.
Unused ignores do not change the process exit code.
File-length violations have no finding ID and cannot be added to `.aposlopignore`.

---

## Configuration

Aposlop reads `<PATH>/.aposlop.toml`.
Aposlop uses built-in values when this file does not exist.

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
`core.exclude` and `file_length.exclude` use the same syntax as one `.gitignore` line.
`core.exclude` removes matching paths from all analysis.
`file_length.exclude` suppresses only file-length violations.
Directory patterns match at any depth, while `/` anchors and `**` recurse.

Language and extension tables can override `max_file_lines` and the existing analysis rules.
Command-line values override all configuration-file layers.

[Read the configuration guide](https://aposlop.ezygang.digital/configuration/).

---

## CLI Quick Reference

| Command or option                     | Purpose                                                      |
| ------------------------------------- | ------------------------------------------------------------ |
| `aposlop ci [PATH]`                   | Print a concise finding summary and fail when findings exist |
| `aposlop allow <FINDING> [PATH]`      | Add a finding to the target's manual exclusions              |
| `--format <terminal\|json>`           | Select the report format                                     |
| `--terminal-output <locations\|code>` | Select terminal duplicate detail                             |
| `--min-lines <N>`                     | Override the minimum block line count                        |
| `--min-nodes <N>`                     | Override the minimum named-node count                        |
| `--exclude <GLOB>`                    | Replace configured gitignore-style exclusion patterns        |
| `--use-cache <BOOL>`                  | Enable or disable the analysis cache                         |
| `--type-1 <BOOL>`                     | Enable or disable Type-1 findings                            |
| `--type-2 <BOOL>`                     | Enable or disable Type-2 findings                            |
| `--type-3 <BOOL>`                     | Enable or disable Type-3 findings                            |
| `--type-3-threshold <RATIO>`          | Override the Type-3 threshold                                |
| `--calculate-complexity <BOOL>`       | Enable or disable complexity findings                        |
| `--complexity-threshold <N>`          | Override the complexity threshold                            |
| `--max-file-lines <N>`                | Override the maximum source-file line count                  |

[Read the complete CLI reference](https://aposlop.ezygang.digital/reference/cli/).

---

## Contributing

Open a [GitHub issue](https://github.com/EzyGang/aposlop/issues) to discuss a large change.
Then open a pull request.

Run these checks before you submit the pull request:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -- --help
```

---

## License

You can use Aposlop under either license:

- [MIT](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
