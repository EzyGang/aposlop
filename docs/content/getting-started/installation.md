# Installation

Aposlop is available as a native binary for Linux, macOS, and Windows.

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
