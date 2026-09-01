# Contributing

Thank you for contributing to Aposlop.

## Before you start

Search existing issues before you create an issue.
Open an issue before you make a large behavior or design change.
Keep each pull request focused on one change.

Read [AI_POLICY.md](AI_POLICY.md) before you use an AI tool for a contribution.

## Development setup

A stable Rust toolchain must support edition 2024.

```bash
git clone https://github.com/EzyGang/aposlop.git
cd aposlop
cargo build
```

## Make a change

Follow the existing code and test patterns.
Add tests for new behavior and fixed defects.
Update user documentation when a public contract changes.
Do not add compatibility aliases, placeholders, or deferred implementations.

## Validate the change

Run all completion checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -- --help
```

Run a focused CLI smoke test when analysis or output behavior changes.

## Open a pull request

Write the pull request description yourself.
Do not use AI to draft or rewrite the description.
Explain the problem, the solution, and the validation results.
Link the related issue when one exists.

A pull request requires the configured checks and one approval.
A code owner must approve the pull request.
The author must resolve all review conversations.
