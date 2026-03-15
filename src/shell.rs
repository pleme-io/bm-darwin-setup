use std::process::Command;

use anyhow::{Context, Result};

/// Set a user's login shell via `dscl`. Skips silently if the user doesn't
/// exist in the macOS directory or if the shell is already correct.
pub fn set(user: &str, shell: &str) -> Result<()> {
    let Some(current) = read_shell(user)? else {
        eprintln!("shell: user {user} not found in directory — skipping");
        return Ok(());
    };

    if current == shell {
        eprintln!("shell: {user} already has {shell}");
        return Ok(());
    }

    let status = Command::new("dscl")
        .args([".", "-change", &format!("/Users/{user}"), "UserShell", &current, shell])
        .status()
        .context("failed to run dscl change")?;

    if status.success() {
        eprintln!("shell: {user} → {shell}");
    } else {
        eprintln!(
            "shell: dscl change failed for {user} (exit {})",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}

/// Print the user's current login shell. Exits cleanly if user doesn't exist.
pub fn check(user: &str) -> Result<()> {
    match read_shell(user)? {
        Some(shell) => println!("{user}: {shell}"),
        None => eprintln!("shell: user {user} not found in directory"),
    }
    Ok(())
}

/// List all local users and their login shells.
pub fn list() -> Result<()> {
    let output = Command::new("dscl")
        .args([".", "-list", "/Users", "UserShell"])
        .output()
        .context("failed to run dscl list")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            // Skip system/hidden users (those starting with _)
            if !line.starts_with('_') {
                println!("{line}");
            }
        }
    }

    Ok(())
}

/// Read a user's `UserShell` from the macOS directory. Returns `None` if the
/// user record doesn't exist.
fn read_shell(user: &str) -> Result<Option<String>> {
    let output = Command::new("dscl")
        .args([".", "-read", &format!("/Users/{user}"), "UserShell"])
        .output()
        .context("failed to run dscl read")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Output format: "UserShell: /bin/zsh"
    let shell = stdout
        .lines()
        .find_map(|line| line.strip_prefix("UserShell:").map(|s| s.trim().to_owned()));

    Ok(shell)
}
