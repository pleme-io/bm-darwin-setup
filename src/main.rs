mod ssh;
mod shell;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "bm-darwin-setup",
    about = "Darwin system activation tool for blackmatter"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage macOS SSH daemon (Remote Login)
    Ssh {
        #[command(subcommand)]
        action: SshAction,
    },
    /// Manage user login shells via Directory Services
    Shell {
        #[command(subcommand)]
        action: ShellAction,
    },
}

#[derive(Subcommand)]
enum SshAction {
    /// Enable sshd via launchctl if not already running
    Enable,
}

#[derive(Subcommand)]
enum ShellAction {
    /// Set a user's login shell (skips if user doesn't exist)
    Set {
        /// macOS username
        user: String,
        /// Path to the shell binary
        shell: String,
    },
    /// Check a user's current login shell
    Check {
        /// macOS username
        user: String,
    },
    /// List all local users and their login shells
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ssh { action } => match action {
            SshAction::Enable => ssh::enable()?,
        },
        Commands::Shell { action } => match action {
            ShellAction::Set { user, shell } => shell::set(&user, &shell)?,
            ShellAction::Check { user } => shell::check(&user)?,
            ShellAction::List => shell::list()?,
        },
    }

    Ok(())
}
