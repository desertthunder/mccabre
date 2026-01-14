use crate::coverage::{CoverageReport, FileCoverage};
use owo_colors::OwoColorize;

#[cfg(test)]
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn wrap_line_ranges(ranges: &[(u32, u32)], max_width: usize) -> String {
    let ranges_plain: Vec<String> = ranges
        .iter()
        .map(
            |(start, end)| {
                if start == end { start.to_string() } else { format!("{}-{}", start, end) }
            },
        )
        .collect();

    let full_str = ranges_plain.join(", ");

    if full_str.len() <= max_width {
        return full_str.red().to_string();
    }

    let mut result = String::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for (i, range_str) in ranges_plain.iter().enumerate() {
        let range_width = range_str.len();
        let comma_sep = ", ";
        let sep_width = if current_width > 0 { comma_sep.len() } else { 0 };

        if current_width + sep_width + range_width <= max_width {
            if current_width > 0 {
                current_line.push_str(comma_sep);
                current_width += comma_sep.len();
            }
            current_line.push_str(range_str);
            current_width += range_width;
        } else {
            if !current_line.is_empty() {
                result.push_str(&current_line.red().to_string());
                result.push('\n');
            }
            current_line = range_str.to_string();
            current_width = range_width;
        }

        if i == ranges_plain.len() - 1 && !current_line.is_empty() {
            result.push_str(&current_line.red().to_string());
        }
    }

    result
}

pub fn report_coverage(report: &CoverageReport) -> String {
    let mut output = String::new();

    output.push_str(&"=".repeat(80).cyan().to_string());
    output.push('\n');
    output.push_str(&"COVERAGE REPORT".cyan().bold().to_string());
    output.push('\n');
    output.push_str(&"=".repeat(80).cyan().to_string());
    output.push_str("\n\n");

    output.push_str(&"SUMMARY".green().bold().to_string());
    output.push('\n');
    output.push_str(&"-".repeat(80).cyan().to_string());
    output.push('\n');
    output.push_str(&format!("Total files:                {}\n", report.files.len().bold()));
    output.push_str(&format!("Total lines:                {}\n", report.totals.total.bold()));
    output.push_str(&format!(
        "Covered lines:              {}\n",
        report.totals.hit.green().bold()
    ));
    output.push_str(&format!(
        "Uncovered lines:            {}\n",
        report.totals.miss.red().bold()
    ));

    let rate_text = if report.totals.rate >= 80.0 {
        format!("{:.2}%", report.totals.rate).green().bold().to_string()
    } else if report.totals.rate >= 50.0 {
        format!("{:.2}%", report.totals.rate).yellow().bold().to_string()
    } else {
        format!("{:.2}%", report.totals.rate).red().bold().to_string()
    };
    output.push_str(&format!("Coverage rate:              {}\n\n", rate_text));

    if !report.files.is_empty() {
        output.push_str(&"FILE COVERAGE".green().bold().to_string());
        output.push('\n');
        output.push_str(&"-".repeat(80).cyan().to_string());
        output.push('\n');

        for file in &report.files {
            output.push_str(&format!("{}\n", format_file_coverage(file, 2)));
        }
    }

    output.push_str(&"=".repeat(80).cyan().to_string());
    output.push('\n');

    output
}

pub fn format_file_coverage(file: &FileCoverage, indent: usize) -> String {
    let spaces = " ".repeat(indent);
    let uncovered_prefix = format!("{}    Uncovered:  ", spaces);
    let mut output = String::new();

    output.push_str(&spaces);
    output.push_str("FILE: ");
    let file_path = file.path.bold().to_string();
    let current_width = spaces.len() + "FILE: ".len();
    let remaining_width = 80 - current_width;

    let path_only = file.path.to_string();
    if path_only.len() <= remaining_width {
        output.push_str(&file_path);
    } else {
        for (i, chunk) in path_only.as_bytes().chunks(remaining_width).enumerate() {
            if i > 0 {
                output.push_str(&format!("\n{}    ", spaces));
            }
            output.push_str(&String::from_utf8_lossy(chunk).bold().to_string());
        }
    }
    output.push('\n');

    let rate_text = if file.summary.rate >= 80.0 {
        format!("{:.2}%", file.summary.rate).green().bold().to_string()
    } else if file.summary.rate >= 50.0 {
        format!("{:.2}%", file.summary.rate).yellow().bold().to_string()
    } else {
        format!("{:.2}%", file.summary.rate).red().bold().to_string()
    };

    output.push_str(&spaces);
    output.push_str(&format!(
        "    Lines:      {} / {} ({})\n",
        file.summary.hit.green().bold(),
        file.summary.total,
        rate_text
    ));

    if !file.miss_ranges.is_empty() {
        output.push_str(&uncovered_prefix);
        let max_width = 80 - uncovered_prefix.len();

        let wrapped_ranges = wrap_line_ranges(&file.miss_ranges, max_width);
        for (i, line) in wrapped_ranges.lines().enumerate() {
            if i > 0 {
                output.push_str(&" ".repeat(uncovered_prefix.len()));
            }
            output.push_str(line);
            output.push('\n');
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coverage::FileCoverage;
    use std::collections::BTreeMap;

    #[test]
    fn test_report_coverage_empty() {
        let report = CoverageReport::new(vec![]);
        let output = report_coverage(&report);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("COVERAGE REPORT"));
        assert!(output.contains("Total files:                0"));
    }

    #[test]
    fn test_report_coverage_with_files() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 5);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let report = CoverageReport::new(vec![file]);
        let output = report_coverage(&report);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("COVERAGE REPORT"));
        assert!(output.contains("test.rs"));
        assert!(output.contains("2 / 3"));
        assert!(output.contains("Uncovered:  2"));
    }

    #[test]
    fn test_format_file_coverage() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 5);
        lines.insert(4, 0);
        lines.insert(5, 0);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let output = format_file_coverage(&file, 2);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("FILE: test.rs"));
        assert!(output.contains("2 / 5"));
        assert!(output.contains("Uncovered:  2, 4-5"));
    }

    #[test]
    fn test_format_file_coverage_full() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 5);
        lines.insert(3, 1);

        let file = FileCoverage::new("full.rs".to_string(), lines);
        let output = format_file_coverage(&file, 2);
        let output = strip_ansi_codes(&output);

        assert!(output.contains("FILE: full.rs"));
        assert!(output.contains("3 / 3 (100.00%)"));
        assert!(!output.contains("Uncovered"));
    }
}
