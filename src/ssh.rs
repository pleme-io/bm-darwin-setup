use std::process::Command;

use anyhow::{Context, Result};

const SSHD_SERVICE: &str = "system/com.openssh.sshd";
const SSHD_PLIST: &str = "/System/Library/LaunchDaemons/ssh.plist";

/// Enable macOS sshd via launchctl if not already loaded.
pub fn enable() -> Result<()> {
    if is_loaded()? {
        eprintln!("sshd: already enabled");
        return Ok(());
    }

    let status = Command::new("/bin/launchctl")
        .args(["load", "-w", SSHD_PLIST])
        .status()
        .context("failed to run launchctl")?;

    if status.success() {
        eprintln!("sshd: enabled");
    } else {
        eprintln!(
            "sshd: launchctl load exited {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}

fn is_loaded() -> Result<bool> {
    let output = Command::new("/bin/launchctl")
        .args(["print", SSHD_SERVICE])
        .output()
        .context("failed to run launchctl print")?;

    Ok(output.status.success())
}
