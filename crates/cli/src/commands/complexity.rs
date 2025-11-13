use anyhow::Result;
use mccabre_core::{
    complexity::{CyclomaticMetrics, LocMetrics},
    config::Config,
    loader::FileLoader,
    reporter::{FileReport, Report},
};
use owo_colors::OwoColorize;
use std::path::PathBuf;

pub fn run(
    path: PathBuf, json: bool, threshold: Option<usize>, config_path: Option<PathBuf>, respect_gitignore: bool,
) -> Result<()> {
    let config = if let Some(config_path) = config_path {
        Config::from_file(config_path)?
    } else {
        Config::load_default()?
    };

    let config = config.merge_with_cli(threshold, None, Some(respect_gitignore));
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

    let report = Report::new(file_reports, Vec::new());

    if json {
        println!("{}", report.to_json()?);
    } else {
        print_complexity_report(&report, &config);
    }

    Ok(())
}

fn print_complexity_report(report: &Report, config: &Config) {
    println!("{}", "=".repeat(80).cyan());
    println!("{}", "COMPLEXITY ANALYSIS".cyan().bold());
    println!("{}\n", "=".repeat(80).cyan());

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
        "High complexity files:       {}\n",
        report.summary.high_complexity_files.bold()
    );

    println!("{}", "FILE METRICS".green().bold());
    println!("{}", "-".repeat(80).cyan());

    for file in &report.files {
        println!("{} {}", "FILE:".blue().bold(), file.path.display().bold());

        let complexity_value = file.cyclomatic.file_complexity;
        let complexity_text = format!("Cyclomatic Complexity:   {complexity_value}");

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
        println!("    Blank lines:             {}\n", file.loc.blank);

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
                    println!("{func_text}");
                }
            }
            println!();
        }
    }

    println!("{}", "=".repeat(80).cyan());
}
