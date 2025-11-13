use anyhow::Result;
use mccabre_core::config::Config;
use owo_colors::OwoColorize;
use std::path::PathBuf;

pub fn run(config_path: Option<PathBuf>) -> Result<()> {
    let config = if let Some(path) = config_path {
        println!("{} {}", "Loading config from:".blue(), path.display());
        Config::from_file(&path)?
    } else {
        println!("{}", "Using default configuration".blue());
        Config::load_default()?
    };

    println!();
    println!("{}", "CONFIGURATION".green().bold());
    println!("{}", "=".repeat(80).cyan());
    println!();

    println!("{}", "Complexity Settings:".yellow().bold());
    println!("  Warning threshold:     {}", config.complexity.warning_threshold);
    println!("  Error threshold:       {}", config.complexity.error_threshold);
    println!();

    println!("{}", "Clone Detection Settings:".yellow().bold());
    println!("  Enabled:               {}", config.clones.enabled);
    println!("  Minimum tokens:        {}", config.clones.min_tokens);
    println!();

    println!("{}", "File Settings:".yellow().bold());
    println!("  Respect .gitignore:    {}", config.files.respect_gitignore);
    println!();

    println!("{}", "=".repeat(80).cyan());
    println!();
    println!("{}", "To save this configuration, create a mccabre.toml file.".dimmed());

    Ok(())
}
