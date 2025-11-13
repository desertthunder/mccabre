use anyhow::Result;
use mccabre_core::{
    cloner::CloneDetector,
    complexity::{CyclomaticMetrics, LocMetrics},
    config::Config,
    loader::{FileLoader, SourceFile},
    reporter::{FileReport, Report},
};
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::highlight::Highlighter;

pub fn run(
    path: PathBuf, json: bool, threshold: Option<usize>, min_tokens: Option<usize>, config_path: Option<PathBuf>,
    respect_gitignore: bool, highlight: bool,
) -> Result<()> {
    let config = if let Some(config_path) = config_path {
        Config::from_file(config_path)?
    } else {
        Config::load_default()?
    };

    let config = config.merge_with_cli(threshold, min_tokens, Some(respect_gitignore));
    let loader = FileLoader::new().with_gitignore(config.files.respect_gitignore);
    let files = loader.load(&path)?;

    if files.is_empty() {
        eprintln!("{}", "No supported files found".yellow());
        return Ok(());
    }

    let mut file_reports = Vec::new();

    for file in &files {
        let loc = LocMetrics::calculate(&file.content, file.language)?;
        let cyclomatic = CyclomaticMetrics::calculate(&file.content, file.language)?;

        file_reports.push(FileReport { path: file.path.clone(), loc, cyclomatic });
    }

    let clones = if config.clones.enabled {
        let detector = CloneDetector::new(config.clones.min_tokens);
        let files_for_clone_detection: Vec<_> = files
            .iter()
            .map(|f| (f.path.clone(), f.content.clone(), f.language))
            .collect();
        detector.detect_across_files(&files_for_clone_detection)?
    } else {
        Vec::new()
    };

    let report = Report::new(file_reports, clones);

    if json {
        println!("{}", report.to_json()?);
    } else {
        print_pretty_report(&report, &config, &files, highlight);
    }

    Ok(())
}

fn print_pretty_report(report: &Report, config: &Config, files: &[SourceFile], highlight: bool) {
    println!("{}", "=".repeat(80).cyan());
    println!("{}", "MCCABRE CODE ANALYSIS REPORT".cyan().bold());
    println!("{}", "=".repeat(80).cyan());
    println!();

    println!("{}", "SUMMARY".green().bold());
    println!("{}", "-".repeat(80).cyan());
    println!("Total files analyzed:        {}", report.summary.total_files.bold());
    println!(
        "Total physical LOC:          {}",
        report.summary.total_physical_loc.bold()
    );
    println!(
        "Total logical LOC:           {}",
        report.summary.total_logical_loc.bold()
    );
    println!(
        "Average complexity:          {}",
        format!("{:.2}", report.summary.avg_complexity).bold()
    );
    println!("Maximum complexity:          {}", report.summary.max_complexity.bold());
    println!(
        "High complexity files:       {}",
        report.summary.high_complexity_files.bold()
    );
    println!("Clone groups detected:       {}", report.summary.total_clones.bold());
    println!();

    if !report.files.is_empty() {
        println!("{}", "FILE METRICS".green().bold());
        println!("{}", "-".repeat(80).cyan());

        for file in &report.files {
            println!("{} {}", "FILE:".blue().bold(), file.path.display().bold());

            let complexity_value = file.cyclomatic.file_complexity;
            let complexity_text = format!("Cyclomatic Complexity:   {}", complexity_value);

            if complexity_value > config.complexity.error_threshold {
                println!("    {}", complexity_text.red().bold());
            } else if complexity_value > config.complexity.warning_threshold {
                println!("    {}", complexity_text.yellow());
            } else {
                println!("    {}", complexity_text.green());
            }
            println!("    Physical LOC:            {}", file.loc.physical);
            println!("    Logical LOC:             {}", file.loc.logical);
            println!("    Comment lines:           {}", file.loc.comments);
            println!("    Blank lines:             {}", file.loc.blank);
            println!();

            if !file.cyclomatic.functions.is_empty() {
                println!("    {}:", "Functions".magenta());
                for func in &file.cyclomatic.functions {
                    let func_text = format!(
                        "      - {} (line {}): complexity {}",
                        func.name, func.line, func.complexity
                    );

                    if func.complexity > config.complexity.error_threshold {
                        println!("{}", func_text.red());
                    } else if func.complexity > config.complexity.warning_threshold {
                        println!("{}", func_text.yellow());
                    } else {
                        println!("{}", func_text);
                    }
                }
                println!();
            }
        }
    }

    if !report.clones.is_empty() {
        println!("{}", "DETECTED CLONES".green().bold());
        println!("{}", "-".repeat(80).cyan());

        let file_map: HashMap<_, _> = files.iter().map(|f| (&f.path, f)).collect();
        let highlighter = if highlight { Some(Highlighter::new()) } else { None };

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

                if highlight && let Some(source_file) = file_map.get(&loc.file) {
                    let code_block = extract_lines(&source_file.content, loc.start_line, loc.end_line);

                    if let Some(ref hl) = highlighter {
                        let ext = source_file.path.extension().and_then(|e| e.to_str()).unwrap_or("txt");
                        let highlighted = hl.highlight(&code_block, ext);

                        println!("{}", "    ┌─────".dimmed());
                        for line in highlighted.lines() {
                            println!("    │ {}", line);
                        }
                        println!("{}", "    └─────".dimmed());
                    }
                }
            }
            println!();
        }
    }

    println!("{}", "=".repeat(80).cyan());
}

/// Extract lines from source code by line numbers (1-indexed)
fn extract_lines(source: &str, start_line: usize, end_line: usize) -> String {
    source
        .lines()
        .enumerate()
        .filter(|(idx, _)| {
            let line_num = idx + 1;
            line_num >= start_line && line_num <= end_line
        })
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join("\n")
}
