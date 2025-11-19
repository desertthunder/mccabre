mod commands;
mod highlight;

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

        /// Disable syntax highlighting for clone code blocks
        #[arg(long)]
        no_highlight: bool,
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

        /// Disable syntax highlighting for clone code blocks
        #[arg(long)]
        no_highlight: bool,
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

    /// Analyze lines of code with ranking
    Loc {
        /// Path to file or directory to analyze
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

        /// Rank by criteria: logical, physical, comments, blank
        #[arg(long, default_value = "logical")]
        rank_by: String,

        /// Rank directories (with files ranked within each)
        #[arg(long)]
        rank_dirs: bool,

        /// Path to config file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Disable gitignore awareness
        #[arg(long)]
        no_gitignore: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path, json, threshold, min_tokens, config, no_gitignore, no_highlight } => {
            commands::analyze::run(
                path,
                json,
                threshold,
                Some(min_tokens),
                config,
                !no_gitignore,
                !no_highlight,
            )
        }
        Commands::Complexity { path, json, threshold, config, no_gitignore } => {
            commands::complexity::run(path, json, threshold, config, !no_gitignore)
        }
        Commands::Clones { path, json, min_tokens, config, no_gitignore, no_highlight } => {
            commands::clones::run(path, json, Some(min_tokens), config, !no_gitignore, !no_highlight)
        }
        Commands::DumpConfig { config, output } => commands::dump_config::run(config, output),
        Commands::Loc { path, json, rank_by, rank_dirs, config, no_gitignore } => {
            use mccabre_core::complexity::loc::RankBy;

            let rank_by = match rank_by.to_lowercase().as_str() {
                "logical" => RankBy::Logical,
                "physical" => RankBy::Physical,
                "comments" => RankBy::Comments,
                "blank" => RankBy::Blank,
                _ => {
                    eprintln!("Invalid rank_by value. Use: logical, physical, comments, or blank");
                    std::process::exit(1);
                }
            };

            commands::loc::run(path, json, rank_by, rank_dirs, config, !no_gitignore)
        }
    }
}
