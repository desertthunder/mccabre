use anyhow::Result;
use mccabre_core::{cloner::CloneDetector, config::Config, loader::FileLoader, reporter::Report};
use owo_colors::OwoColorize;
use std::path::PathBuf;

pub fn run(
    path: PathBuf, json: bool, min_tokens: Option<usize>, config_path: Option<PathBuf>, respect_gitignore: bool,
) -> Result<()> {
    let config = if let Some(config_path) = config_path {
        Config::from_file(config_path)?
    } else {
        Config::load_default()?
    };

    let config = config.merge_with_cli(None, min_tokens, Some(respect_gitignore));
    let loader = FileLoader::new().with_gitignore(config.files.respect_gitignore);
    let files = loader.load(&path)?;

    if files.is_empty() {
        eprintln!("{}", "No supported files found".yellow());
        return Ok(());
    }

    let detector = CloneDetector::new(config.clones.min_tokens);
    let files_for_clone_detection: Vec<_> = files
        .iter()
        .map(|f| (f.path.clone(), f.content.clone(), f.language))
        .collect();
    let clones = detector.detect_across_files(&files_for_clone_detection)?;

    let report = Report::new(Vec::new(), clones);

    if json {
        println!("{}", report.to_json()?);
    } else {
        print_clones_report(&report);
    }

    Ok(())
}

fn print_clones_report(report: &Report) {
    println!("{}", "=".repeat(80).cyan());
    println!("{}", "CLONE DETECTION REPORT".cyan().bold());
    println!("{}\n", "=".repeat(80).cyan());

    if report.clones.is_empty() {
        println!("{}", "No clones detected!".green().bold());
    } else {
        println!(
            "{} {} {}",
            "Found".green().bold(),
            report.clones.len().to_string().yellow().bold(),
            "clone groups".green().bold()
        );
        println!();

        for clone in &report.clones {
            println!(
                "{} {} {} {} {} {}",
                "Clone Group".yellow(),
                format!("#{}", clone.id).yellow().bold(),
                "(length:".dimmed(),
                format!("{} tokens", clone.length).bold(),
                format!("{} occurrences)", clone.locations.len()).bold(),
                "".dimmed()
            );

            for loc in &clone.locations {
                println!(
                    "  {} {}:{}",
                    "-".dimmed(),
                    loc.file.display(),
                    format!("{}-{}", loc.start_line, loc.end_line).dimmed()
                );
            }
            println!();
        }
    }

    println!("{}", "=".repeat(80).cyan());
}
