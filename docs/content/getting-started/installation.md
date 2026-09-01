# Installation

Aposlop is a Rust binary that targets stable Rust and edition 2024.

## Install from a checkout

Run this command from the Aposlop repository root:

```bash
cargo install --path .
```

Confirm the installation:

```bash
aposlop --version
aposlop --help
```

## Build without installing

Use Cargo when testing a local checkout:

```bash
cargo build --release
cargo run --release -- /path/to/project
```

Cargo writes the release binary under `target/release/`.

