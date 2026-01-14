use anyhow::Result;
use mccabre_core::coverage::{FileCoverage, parse_coverage_from_file};
use mccabre_core::reporter::{coverage_jsonl::JsonlReporter, coverage_term::report_coverage};
use owo_colors::OwoColorize;
use std::path::PathBuf;

pub fn run(from: PathBuf, jsonl: Option<PathBuf>, repo_root: Option<PathBuf>) -> Result<()> {
    if !from.exists() {
        eprintln!("{}", format!("LCOV file not found: {}", from.display()).red());
        std::process::exit(1);
    }

    let report = parse_coverage_from_file(&from, repo_root.as_deref())?;

    if report.files.is_empty() {
        eprintln!("{}", "No coverage data found".yellow());
        return Ok(());
    }

    if let Some(jsonl_path) = jsonl {
        let mut reporter = JsonlReporter::new();
        reporter.add_report(&report);
        reporter.write_to_file(&jsonl_path)?;

        println!(
            "{}",
            format!("JSONL report written to: {}", jsonl_path.display())
                .green()
                .bold()
        );
    }

    println!("{}", report_coverage(&report));

    Ok(())
}

pub fn run_file_view(path: PathBuf, from: PathBuf) -> Result<()> {
    if !from.exists() {
        eprintln!("{}", format!("LCOV file not found: {}", from.display()).red());
        std::process::exit(1);
    }

    let report = parse_coverage_from_file(&from, None)?;

    let file_coverage = report.files.iter().find(|f| f.path == path.to_string_lossy());

    match file_coverage {
        Some(file) => {
            println!("{}", report_coverage_for_file(file));
        }
        None => {
            eprintln!(
                "{}",
                format!("File not found in coverage data: {}", path.display()).red()
            );
            std::process::exit(1);
        }
    }

    Ok(())
}

fn report_coverage_for_file(file: &FileCoverage) -> String {
    mccabre_core::reporter::coverage_term::format_file_coverage(file, 0)
}
