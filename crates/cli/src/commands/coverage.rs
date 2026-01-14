use anyhow::Result;
use mccabre_core::coverage::{FileCoverage, parse_coverage_from_file};
use mccabre_core::reporter::{coverage_jsonl::JsonlReporter, coverage_term::report_coverage};
use owo_colors::OwoColorize;
use std::fs;
use std::path::{Path, PathBuf};

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

pub fn run_show(
    from: PathBuf, repo_root: Option<PathBuf>, path: Option<PathBuf>, truncate_threshold: Option<usize>,
) -> Result<()> {
    if !from.exists() {
        eprintln!("{}", format!("LCOV file not found: {}", from.display()).red());
        std::process::exit(1);
    }

    let report = parse_coverage_from_file(&from, repo_root.as_deref())?;

    if report.files.is_empty() {
        eprintln!("{}", "No coverage data found".yellow());
        return Ok(());
    }

    match path {
        Some(path) => {
            if path.is_file() {
                show_file_coverage(&report, &path, truncate_threshold)?;
            } else if path.is_dir() {
                show_directory_coverage(&report, &path)?;
            } else {
                eprintln!("{}", format!("Path not found: {}", path.display()).red());
                std::process::exit(1);
            }
        }
        None => {
            show_all_files(&report)?;
        }
    }

    Ok(())
}

fn show_file_coverage(
    report: &mccabre_core::coverage::CoverageReport, path: &Path, truncate_threshold: Option<usize>,
) -> Result<()> {
    let path_str = path.to_string_lossy();

    let file_coverage = report
        .files
        .iter()
        .find(|f| f.path == path_str || f.path.ends_with(path_str.as_ref()));

    match file_coverage {
        Some(file) => {
            if !path.exists() {
                eprintln!("{}", format!("Source file not found: {}", path.display()).yellow());
                return Ok(());
            }

            let source_code = fs::read_to_string(path)?;
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt");

            let output = mccabre_core::reporter::report_detailed_file_view(
                file,
                &source_code,
                extension,
                truncate_threshold.unwrap_or(5),
            );
            println!("{}", output);

            Ok(())
        }
        None => {
            eprintln!("{}", format!("File not found in coverage data: {}", path_str).red());
            std::process::exit(1);
        }
    }
}

fn show_directory_coverage(report: &mccabre_core::coverage::CoverageReport, dir_path: &Path) -> Result<()> {
    let dir_str = dir_path.to_string_lossy().to_string();

    let files: Vec<FileCoverage> = report
        .files
        .iter()
        .filter(|f| f.path.starts_with(&dir_str))
        .cloned()
        .collect();

    if files.is_empty() {
        eprintln!(
            "{}",
            format!("No coverage data found for directory: {}", dir_str).yellow()
        );
        return Ok(());
    }

    let output = mccabre_core::reporter::report_directory_view(&files, &dir_str);
    println!("{}", output);

    Ok(())
}

fn show_all_files(report: &mccabre_core::coverage::CoverageReport) -> Result<()> {
    let output = mccabre_core::reporter::report_directory_view(&report.files, "");
    println!("{}", output);

    Ok(())
}
