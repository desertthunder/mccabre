use anyhow::Result;
use mccabre_core::{
    complexity::loc::{FileLocReport, LocMetrics, LocReport, RankBy},
    config::Config,
    loader::FileLoader,
};
use owo_colors::OwoColorize;
use std::path::PathBuf;

pub fn run(
    path: PathBuf, json: bool, rank_by: RankBy, rank_dirs: bool, config_path: Option<PathBuf>, respect_gitignore: bool,
) -> Result<()> {
    let config = if let Some(config_path) = config_path {
        Config::from_file(config_path)?
    } else {
        Config::load_default()?
    };

    let config = config.merge_with_cli(None, None, Some(respect_gitignore));
    let loader = FileLoader::new().with_gitignore(config.files.respect_gitignore);
    let files = loader.load(&path)?;

    if files.is_empty() {
        eprintln!("{}", "No supported files found".yellow());
        return Ok(());
    }

    let mut file_reports = Vec::new();

    for file in &files {
        let metrics = LocMetrics::calculate(&file.content, file.language)?;
        file_reports.push(FileLocReport { path: file.path.clone(), metrics });
    }

    let report = LocReport::new(file_reports, rank_by, rank_dirs);

    if json {
        println!("{}", report.to_json()?);
    } else {
        print_loc_report(&report, rank_by, rank_dirs);
    }

    Ok(())
}

fn print_loc_report(report: &LocReport, rank_by: RankBy, rank_dirs: bool) {
    println!("{}", "=".repeat(80).cyan());
    println!("{}", "LINES OF CODE ANALYSIS".cyan().bold());
    println!("{}\n", "=".repeat(80).cyan());

    println!("{}", "SUMMARY".green().bold());
    println!("{}", "-".repeat(80).cyan());
    println!("Total files analyzed:        {}", report.summary.total_files.bold());
    println!("Total physical LOC:          {}", report.summary.total_physical.bold());
    println!("Total logical LOC:           {}", report.summary.total_logical.bold());
    println!("Total comment lines:         {}", report.summary.total_comments.bold());
    println!("Total blank lines:           {}\n", report.summary.total_blank.bold());

    let rank_label = match rank_by {
        RankBy::Logical => "Logical LOC",
        RankBy::Physical => "Physical LOC",
        RankBy::Comments => "Comment Lines",
        RankBy::Blank => "Blank Lines",
    };

    if rank_dirs {
        if let Some(directories) = &report.directories {
            println!(
                "{} {}",
                "DIRECTORIES RANKED BY".green().bold(),
                rank_label.green().bold()
            );
            println!("{}\n", "-".repeat(80).cyan());

            for dir in directories {
                println!("{} {}", "DIRECTORY:".blue().bold(), dir.path.display().bold());
                println!(
                    "  Total Physical:  {} | Logical:  {} | Comments: {} | Blank: {}",
                    dir.total.physical.bold(),
                    dir.total.logical.bold(),
                    dir.total.comments.bold(),
                    dir.total.blank.bold()
                );
                println!();

                if !dir.files.is_empty() {
                    println!("  {}:", "Files".magenta());
                    for file in &dir.files {
                        let filename = file.path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");

                        let rank_value = rank_by.value_from(&file.metrics);
                        println!(
                            "    {} ({}: {}) - P: {} | L: {} | C: {} | B: {}",
                            filename,
                            rank_label.dimmed(),
                            rank_value.to_string().yellow(),
                            file.metrics.physical,
                            file.metrics.logical,
                            file.metrics.comments,
                            file.metrics.blank
                        );
                    }
                    println!();
                }
            }
        }
    } else {
        println!("{} {}", "FILES RANKED BY".green().bold(), rank_label.green().bold());
        println!("{}\n", "-".repeat(80).cyan());

        for (idx, file) in report.files.iter().enumerate() {
            let rank_value = rank_by.value_from(&file.metrics);
            println!(
                "{}. {} ({}: {})",
                (idx + 1).to_string().dimmed(),
                file.path.display().bold(),
                rank_label.dimmed(),
                rank_value.to_string().yellow()
            );
            println!(
                "   Physical: {} | Logical: {} | Comments: {} | Blank: {}",
                file.metrics.physical, file.metrics.logical, file.metrics.comments, file.metrics.blank
            );
            println!();
        }
    }

    println!("{}", "=".repeat(80).cyan());
}
