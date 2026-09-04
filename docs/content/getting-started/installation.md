# Installation

Aposlop is available as a native binary for Linux, macOS, and Windows.

Aposlop has two components:

1. The CLI scans source code and reports findings.
2. Agent skills guide coding agents during setup, coding, testing, and cleanup.

Install either component or both.

## Install script on Linux or macOS

Install [cosign](https://docs.sigstore.dev/cosign/system_config/installation/).
Then run the installer:

```bash
curl -fsSLo install.sh https://github.com/EzyGang/aposlop/releases/latest/download/install.sh
sh install.sh
```

The script installs Aposlop in `$HOME/.local/bin` by default.
Set `APOSLOP_INSTALL_DIR` to select a different directory.

## Install script on Windows

Install [cosign](https://docs.sigstore.dev/cosign/system_config/installation/).
Then run the installer:

```powershell
Invoke-WebRequest https://github.com/EzyGang/aposlop/releases/latest/download/install.ps1 -OutFile install.ps1
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

The script installs Aposlop under `%LOCALAPPDATA%\Programs\aposlop\bin` by default.
Use `-InstallDir` to select a different directory.

Both scripts verify the archive checksum and Sigstore signatures.

## Install with Cargo

```bash
cargo install aposlop --locked
```

## Install from PyPI with uv

```bash
uv tool install aposlop
```

Use `uvx` for a temporary installation:

```bash
uvx aposlop --help
```

## Install with Homebrew

```bash
brew install EzyGang/tap/aposlop
```

## Install from source

A stable Rust toolchain must support edition 2024.

```bash
git clone https://github.com/EzyGang/aposlop.git
cd aposlop
cargo install --path . --locked
```

## Install the agent skills

Use the installed CLI:

```bash
aposlop install-skills
```

The command uses `npx` when available.
It uses `pnpm dlx` when `npx` is unavailable.

You can also run either installer directly:

```bash
npx skills@latest add EzyGang/aposlop
pnpm dlx skills@latest add EzyGang/aposlop
```

The installer lets you choose the skills and target agents.
The skills do not install the Aposlop CLI.
Read the [agent skills guide](../skills/index.md) for each skill.

## Update checks

Aposlop checks for a new GitHub release during interactive runs.
It performs a network request at most once every 24 hours.
It stores the result in the user cache directory.
Non-interactive commands do not perform this check.

Set this environment variable to disable the check:

```bash
APOSLOP_NO_UPDATE_CHECK=1 aposlop .
```

## Verify the installation

```bash
aposlop --version
aposlop --help
```
