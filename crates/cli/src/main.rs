mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mccabre")]
#[command(about = "Code complexity & clone detection tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full analysis (complexity + clones + LOC)
    Analyze {
        /// Path to file or directory to analyze
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

        /// Complexity threshold for warnings
        #[arg(long)]
        threshold: Option<usize>,

        /// Minimum tokens for clone detection
        #[arg(long, default_value = "30")]
        min_tokens: usize,

        /// Path to config file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Disable gitignore awareness
        #[arg(long)]
        no_gitignore: bool,
    },

    /// Analyze cyclomatic complexity and LOC only
    Complexity {
        /// Path to file or directory to analyze
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

        /// Complexity threshold for warnings
        #[arg(long)]
        threshold: Option<usize>,

        /// Path to config file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Disable gitignore awareness
        #[arg(long)]
        no_gitignore: bool,
    },

    /// Detect code clones only
    Clones {
        /// Path to file or directory to analyze
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

        /// Minimum tokens for clone detection
        #[arg(long, default_value = "30")]
        min_tokens: usize,

        /// Path to config file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Disable gitignore awareness
        #[arg(long)]
        no_gitignore: bool,
    },

    /// Display current configuration
    DumpConfig {
        /// Path to config file (if not specified, shows defaults)
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Save configuration to file (file path or directory)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path, json, threshold, min_tokens, config, no_gitignore } => {
            commands::analyze::run(path, json, threshold, Some(min_tokens), config, !no_gitignore)
        }

        Commands::Complexity { path, json, threshold, config, no_gitignore } => {
            commands::complexity::run(path, json, threshold, config, !no_gitignore)
        }

        Commands::Clones { path, json, min_tokens, config, no_gitignore } => {
            commands::clones::run(path, json, Some(min_tokens), config, !no_gitignore)
        }

        Commands::DumpConfig { config, output } => commands::dump_config::run(config, output),
    }
}
