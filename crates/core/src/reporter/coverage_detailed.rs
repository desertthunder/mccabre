use crate::coverage::FileCoverage;
use crate::highlight::Highlighter;

use super::coverage_term::strip_ansi_codes;

use owo_colors::OwoColorize;

pub fn report_detailed_file_view(
    file: &FileCoverage, source_code: &str, file_extension: &str, truncate_threshold: usize,
) -> String {
    let mut output = String::new();

    output.push_str(&render_header(file));
    output.push('\n');

    let highlighter = Highlighter::new();
    let highlighted_code = highlighter.highlight(source_code, file_extension);

    let lines: Vec<&str> = highlighted_code.lines().collect();

    let max_line_num = lines.len();
    let line_num_width = max_line_num.to_string().len();

    let mut current_range_start: Option<usize> = None;
    let mut range_lines: Vec<(usize, &str)> = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let line_num = line_idx + 1;
        let hit_count = file.lines.get(&(line_num as u32));

        if is_ignored_line(hit_count) {
            if current_range_start.is_none() {
                current_range_start = Some(line_num);
            }
            range_lines.push((line_num, line));
        } else {
            if let Some(_start) = current_range_start.take() {
                output.push_str(&handle_range(&range_lines, line_num_width, truncate_threshold));
                range_lines.clear();
            }
            let line_output = render_line_with_coverage(line, line_num, hit_count, line_num_width);
            output.push_str(&line_output);
            output.push('\n');
        }
    }

    if !range_lines.is_empty() {
        output.push_str(&handle_range(&range_lines, line_num_width, truncate_threshold));
    }

    output
}

fn render_header(file: &FileCoverage) -> String {
    let mut output = String::new();

    let separator = "═".repeat(80);

    output.push_str(&separator.bright_cyan().to_string());
    output.push('\n');

    let rate_text = if file.summary.rate >= 80.0 {
        format!("{:.2}%", file.summary.rate).green().bold().to_string()
    } else if file.summary.rate >= 50.0 {
        format!("{:.2}%", file.summary.rate).yellow().bold().to_string()
    } else {
        format!("{:.2}%", file.summary.rate).red().bold().to_string()
    };

    output.push_str(&format!("FILE: {}", file.path.bold()));
    output.push('\n');

    output.push_str(&format!(
        "Lines: {} / {} | {}",
        file.summary.hit.green().bold(),
        file.summary.total,
        rate_text
    ));
    output.push('\n');

    let bar_width = 40;
    let filled = (file.summary.rate / 100.0 * bar_width as f64) as usize;
    let empty = bar_width - filled;

    let bar = format!("{}{}", "█".repeat(filled).green(), "░".repeat(empty).red());
    output.push_str(&format!("[{}]", bar));

    output.push('\n');
    output.push_str(&separator.bright_cyan().to_string());

    output
}

fn render_line_with_coverage(line: &str, line_num: usize, hit_count: Option<&u64>, line_num_width: usize) -> String {
    let (hit_str, marker, styled_line) = match hit_count {
        Some(0) => {
            let clean_line = strip_ansi_codes(line);
            (
                " -".red().to_string(),
                " !".red().bold().to_string(),
                clean_line.bright_red().bold().to_string(),
            )
        }
        Some(_count) => (
            " ✓".green().to_string(),
            " |".green().to_string(),
            line.dimmed().to_string(),
        ),
        None => (
            " -".dimmed().to_string(),
            " .".dimmed().to_string(),
            line.dimmed().to_string(),
        ),
    };

    format!(
        "{:>width$}  {} {} {}",
        line_num,
        hit_str,
        marker,
        styled_line,
        width = line_num_width
    )
}

fn is_ignored_line(hit_count: Option<&u64>) -> bool {
    hit_count.is_none()
}

fn render_truncation_marker(line_num_width: usize) -> String {
    format!("{:>width$}  {} {}", "", " -", " . ...".dimmed(), width = line_num_width)
}

fn handle_range(lines: &[(usize, &str)], line_num_width: usize, threshold: usize) -> String {
    let mut output = String::new();

    if lines.len() >= threshold {
        let (first_num, first_line) = lines.first().unwrap();
        let (last_num, last_line) = lines.last().unwrap();

        output.push_str(&render_line_with_coverage(first_line, *first_num, None, line_num_width));
        output.push('\n');
        output.push_str(&render_truncation_marker(line_num_width));
        output.push('\n');
        output.push_str(&render_line_with_coverage(last_line, *last_num, None, line_num_width));
        output.push('\n');
    } else {
        for (line_num, line) in lines {
            output.push_str(&render_line_with_coverage(line, *line_num, None, line_num_width));
            output.push('\n');
        }
    }

    output
}

pub fn report_directory_view(files: &[FileCoverage], base_path: &str) -> String {
    let mut output = String::new();

    output.push_str(&"═".repeat(80).bright_cyan().to_string());
    output.push('\n');
    output.push_str(&format!("DIRECTORY: {}", base_path.bold()));
    output.push('\n');
    output.push_str(&"═".repeat(80).bright_cyan().to_string());
    output.push_str("\n\n");

    let mut sorted_files: Vec<&FileCoverage> = files.iter().collect();
    sorted_files.sort_by(|a, b| a.summary.rate.partial_cmp(&b.summary.rate).unwrap());

    for file in sorted_files {
        output.push_str(&format_file_entry(file, base_path));
        output.push('\n');
    }

    output
}

fn format_file_entry(file: &FileCoverage, base_path: &str) -> String {
    let relative_path = file
        .path
        .strip_prefix(base_path)
        .unwrap_or(&file.path)
        .trim_start_matches('/');

    let rate_text = if file.summary.rate >= 80.0 {
        format!("{:>6.2}%", file.summary.rate).green().bold().to_string()
    } else if file.summary.rate >= 50.0 {
        format!("{:>6.2}%", file.summary.rate).yellow().bold().to_string()
    } else {
        format!("{:>6.2}%", file.summary.rate).red().bold().to_string()
    };

    format!(
        "{} {} / {} ({})",
        rate_text, file.summary.hit, file.summary.total, relative_path
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn create_test_file_coverage(path: &str) -> FileCoverage {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 5);
        FileCoverage::new(path.to_string(), lines)
    }

    #[test]
    fn test_render_header() {
        let file = create_test_file_coverage("test.rs");
        let header = render_header(&file);
        let header = strip_ansi_codes(&header);

        assert!(header.contains("FILE: test.rs"));
        assert!(header.contains("Lines: 2 / 3"));
        assert!(header.contains("66.67%"));
    }

    #[test]
    fn test_render_line_with_coverage_hit() {
        let line = "fn main() {}";
        let output = render_line_with_coverage(line, 1, Some(&10), 3);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("1"));
        assert!(output.contains("✓"));
        assert!(output.contains("|"));
        assert!(output.contains("fn main() {}"));
    }

    #[test]
    fn test_render_line_with_coverage_miss() {
        let line = "println!(\"hello\");";
        let output = render_line_with_coverage(line, 2, Some(&0), 3);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("2"));
        assert!(output.contains("-"));
        assert!(output.contains("!"));
        assert!(output.contains("println!(\"hello\");"));
    }

    #[test]
    fn test_render_line_with_coverage_none() {
        let line = "// comment";
        let output = render_line_with_coverage(line, 1, None, 3);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("1"));
        assert!(output.contains("-"));
        assert!(output.contains("."));
        assert!(output.contains("// comment"));
    }

    #[test]
    fn test_report_directory_view() {
        let file1 = create_test_file_coverage("src/lib.rs");
        let file2 = create_test_file_coverage("src/main.rs");

        let output = report_directory_view(&[file1, file2], "src");
        let output = strip_ansi_codes(&output);

        assert!(output.contains("DIRECTORY: src"));
        assert!(output.contains("lib.rs"));
        assert!(output.contains("main.rs"));
    }

    #[test]
    fn test_format_file_entry() {
        let file = create_test_file_coverage("src/test.rs");
        let entry = format_file_entry(&file, "src/");
        let entry = strip_ansi_codes(&entry);

        assert!(entry.contains("66.67%"));
        assert!(entry.contains("2 / 3"));
        assert!(entry.contains("test.rs"));
    }

    #[test]
    fn test_report_detailed_file_view() {
        let file = create_test_file_coverage("test.rs");
        let source_code = "fn main() {\n    println!(\"Hello\");\n    return;\n}";

        let output = report_detailed_file_view(&file, source_code, "rs", 5);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("FILE: test.rs"));
        assert!(output.contains("fn main() {"));
        assert!(output.contains("Hello"));
    }

    #[test]
    fn test_truncation_default_threshold() {
        let file = create_test_file_coverage("test.rs");
        let source_code = (1..100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");

        let output = report_detailed_file_view(&file, &source_code, "rs", 5);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("..."));
        assert!(output.lines().count() < 100);
    }

    #[test]
    fn test_no_truncation_below_threshold() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(6, 5);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let source_code = (1..10).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");

        let output = report_detailed_file_view(&file, &source_code, "rs", 5);
        let output = strip_ansi_codes(&output);

        assert!(!output.contains("..."));
    }

    #[test]
    fn test_custom_truncation_threshold() {
        let file = create_test_file_coverage("test.rs");
        let source_code = (1..100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");

        let output = report_detailed_file_view(&file, &source_code, "rs", 10);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("..."));
    }

    #[test]
    fn test_truncation_at_start() {
        let mut lines = BTreeMap::new();
        lines.insert(10, 5);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let source_code = (1..100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");

        let output = report_detailed_file_view(&file, &source_code, "rs", 5);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("line 1"));
        assert!(output.contains("..."));
        assert!(output.contains("line 9"));
        assert!(output.contains("✓"));
        assert!(output.contains("line 10"));
    }

    #[test]
    fn test_truncation_at_end() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 5);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let source_code = (1..=100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");

        let output = report_detailed_file_view(&file, &source_code, "rs", 5);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("✓"));
        assert!(output.contains("line 1"));
        assert!(output.contains("..."));
        assert!(output.contains("line 100"));
    }
}
