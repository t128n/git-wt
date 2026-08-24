mod config;
mod git;
mod worktree;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

use config::Config;

#[derive(Parser)]
#[command(
    name = "git-wt",
    about = "A simpler interface to git worktrees",
    version,
    after_help = "Run 'git-wt <command> --help' for more information on a specific command."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Generate shell completions for the specified shell
    #[arg(long = "completions", value_enum)]
    completions: Option<Shell>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a named worktree
    Add {
        /// Name for the worktree
        name: String,

        /// Branch to checkout (optional)
        branch: Option<String>,
    },

    /// List all worktrees with friendly names
    List,

    /// Remove a worktree by name or path
    Remove {
        /// Worktree name or path
        target: String,

        /// Force removal even if dirty
        #[arg(long)]
        force: bool,
    },

    /// Clean up stale worktree data
    Prune,

    /// Print worktree path (use with cd)
    Goto {
        /// Worktree name
        name: String,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Initialize config file with all available settings
    Init {
        /// Overwrite existing config file if present
        #[arg(long, short)]
        force: bool,
    },

    /// Reset config file to factory defaults
    Reset,

    /// Open config file in editor
    Edit,
}

#[derive(clap::ValueEnum, Clone)]
#[allow(clippy::enum_variant_names)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        let shell_name = match shell {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::PowerShell => clap_complete::Shell::PowerShell,
            Shell::Elvish => clap_complete::Shell::Elvish,
        };
        clap_complete::generate(shell_name, &mut cmd, "git-wt", &mut std::io::stdout());
        return Ok(());
    }

    let config = Config::load();

    match cli.command {
        Some(Commands::Add { name, branch }) => {
            worktree::add_worktree(&config, &name, branch.as_deref())
        }
        Some(Commands::List) => worktree::list_worktrees(&config),
        Some(Commands::Remove { target, force }) => {
            worktree::remove_worktree(&config, &target, force)
        }
        Some(Commands::Prune) => worktree::prune_worktrees(),
        Some(Commands::Goto { name }) => {
            let path = worktree::goto_worktree(&config, &name)?;
            println!("{}", path.display());
            Ok(())
        }
        Some(Commands::Config { action }) => cmd_config(action),
        None => {
            print_usage();
            Ok(())
        }
    }
}

fn cmd_config(action: Option<ConfigAction>) -> Result<()> {
    match action {
        Some(ConfigAction::Init { force }) => {
            let path = Config::init(force)?;
            println!("Initialized config at {}", path.display());
            Ok(())
        }
        Some(ConfigAction::Reset) => {
            let path = Config::reset()?;
            println!("Reset config to factory defaults at {}", path.display());
            Ok(())
        }
        Some(ConfigAction::Edit) => cmd_config_edit(),
        None => {
            if let Some(path) = Config::config_path() {
                let status = if path.exists() { "exists" } else { "not found" };
                println!("Config file: {} ({status})", path.display());
            } else {
                println!("Config file: unknown (could not determine home directory)");
            }
            println!();
            println!("Available subcommands:");
            println!("    init     Initialize config file with all available settings (use --force to overwrite)");
            println!("    reset    Reset config file to factory defaults");
            println!("    edit     Open config file in editor");
            println!();
            println!("Usage: git-wt config <COMMAND>");
            Ok(())
        }
    }
}

fn cmd_config_edit() -> Result<()> {
    let path = Config::config_path().context("Could not determine user home directory")?;

    if !path.exists() {
        Config::init(false)?;
    }

    let editor = std::env::var("EDITOR").ok();

    let editors: Vec<String> = if let Some(ed) = editor {
        vec![ed]
    } else if cfg!(windows) {
        vec!["notepad".to_string()]
    } else {
        vec!["vim".to_string(), "nano".to_string(), "vi".to_string()]
    };

    for editor in &editors {
        let status = Command::new(editor).arg(&path).status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            Ok(_) => continue,
            Err(_) => continue,
        }
    }

    anyhow::bail!(
        "No editor found. Set EDITOR environment variable or install vim/nano/vi"
    )
}

fn print_usage() {
    println!("git-wt - A simpler interface to git worktrees");
    println!();
    println!("USAGE:");
    println!("    git wt add <name> [branch]    Create a named worktree");
    println!("    git wt list                   List all worktrees");
    println!("    git wt remove <name> [--force] Remove a worktree");
    println!("    git wt prune                  Clean up stale worktree data");
    println!("    git wt goto <name>            Print worktree path (for cd)");
    println!("    git wt config [init|reset|edit] Manage configuration");
    println!("    git wt help                   Print this help message");
    println!();
    println!("OPTIONS:");
    println!("    --force        Force removal even if dirty");
    println!("    --completions <SHELL>  Generate shell completions (bash, zsh, fish, powershell, elvish)");
    if let Some(path) = Config::config_path() {
        println!();
        println!("CONFIG: {}", path.display());
    }
}
