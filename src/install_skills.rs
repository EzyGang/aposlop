use std::io;
use std::process::{Command, ExitStatus};

use anyhow::{Context, bail};

#[cfg(not(windows))]
const NPX: &str = "npx";
#[cfg(windows)]
const NPX: &str = "npx.cmd";
#[cfg(not(windows))]
const PNPM: &str = "pnpm";
#[cfg(windows)]
const PNPM: &str = "pnpm.cmd";

pub(crate) fn run() -> anyhow::Result<()> {
    match Command::new(NPX)
        .args(["skills@latest", "add", "EzyGang/aposlop"])
        .status()
    {
        Ok(status) => return require_success("npx", status),
        Err(error) if error.kind() == io::ErrorKind::NotFound => (),
        Err(error) => return Err(error).context("failed to start npx"),
    }

    match Command::new(PNPM)
        .args(["dlx", "skills@latest", "add", "EzyGang/aposlop"])
        .status()
    {
        Ok(status) => require_success("pnpm", status),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            bail!(
                "npx and pnpm are unavailable; install one, then run `aposlop install-skills` again"
            )
        }
        Err(error) => Err(error).context("failed to start pnpm"),
    }
}

fn require_success(runner: &str, status: ExitStatus) -> anyhow::Result<()> {
    if status.success() {
        return Ok(());
    }
    bail!("{runner} failed with {status}")
}
