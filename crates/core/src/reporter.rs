use crate::cloner::Clone;
use crate::complexity::{CyclomaticMetrics, LocMetrics, Severity};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete analysis report for a codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// Per-file analysis results
    pub files: Vec<FileReport>,
    /// Detected clones across all files
    pub clones: Vec<Clone>,
    /// Summary statistics
    pub summary: Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReport {
    /// File path
    pub path: PathBuf,
    /// Lines of code metrics
    pub loc: LocMetrics,
    /// Cyclomatic complexity metrics
    pub cyclomatic: CyclomaticMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    /// Total number of files analyzed
    pub total_files: usize,
    /// Total physical lines of code
    pub total_physical_loc: usize,
    /// Total logical lines of code
    pub total_logical_loc: usize,
    /// Average cyclomatic complexity
    pub avg_complexity: f64,
    /// Maximum cyclomatic complexity
    pub max_complexity: usize,
    /// Number of files with high complexity
    pub high_complexity_files: usize,
    /// Total number of clone groups
    pub total_clones: usize,
}

impl Report {
    pub fn new(files: Vec<FileReport>, clones: Vec<Clone>) -> Self {
        let summary = Summary::from_files(&files, &clones);
        Self { files, clones, summary }
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Generate plaintext report
    pub fn to_plaintext(&self) -> String {
        let mut output = String::new();

        output.push_str(&"=".repeat(80));
        output.push('\n');
        output.push_str("MCCABRE CODE ANALYSIS REPORT\n");
        output.push_str(&"=".repeat(80));
        output.push_str("\n\n");

        output.push_str("SUMMARY\n");
        output.push_str(&"-".repeat(80));
        output.push('\n');
        output.push_str(&format!("Total files analyzed:        {}\n", self.summary.total_files));
        output.push_str(&format!(
            "Total physical LOC:          {}\n",
            self.summary.total_physical_loc
        ));
        output.push_str(&format!(
            "Total logical LOC:           {}\n",
            self.summary.total_logical_loc
        ));
        output.push_str(&format!(
            "Average complexity:          {:.2}\n",
            self.summary.avg_complexity
        ));
        output.push_str(&format!(
            "Maximum complexity:          {}\n",
            self.summary.max_complexity
        ));
        output.push_str(&format!(
            "High complexity files:       {}\n",
            self.summary.high_complexity_files
        ));
        output.push_str(&format!(
            "Clone groups detected:       {}\n\n",
            self.summary.total_clones
        ));

        if !self.files.is_empty() {
            output.push_str("FILE METRICS\n");
            output.push_str(&"-".repeat(80));
            output.push('\n');

            for file in &self.files {
                output.push_str(&format!("FILE: {}\n", file.path.display()));
                output.push_str(&format!(
                    "    Cyclomatic Complexity:   {} ({})\n",
                    file.cyclomatic.file_complexity,
                    match file.cyclomatic.severity() {
                        Severity::Low => "low",
                        Severity::Moderate => "moderate",
                        Severity::High => "high",
                        Severity::VeryHigh => "very high",
                    }
                ));
                output.push_str(&format!("    Physical LOC:            {}\n", file.loc.physical));
                output.push_str(&format!("    Logical LOC:             {}\n", file.loc.logical));
                output.push_str(&format!("    Comment lines:           {}\n", file.loc.comments));
                output.push_str(&format!("    Blank lines:             {}\n\n", file.loc.blank));

                if !file.cyclomatic.functions.is_empty() {
                    output.push_str("    Functions:\n");
                    for func in &file.cyclomatic.functions {
                        output.push_str(&format!(
                            "      - {} (line {}): complexity {}\n",
                            func.name, func.line, func.complexity
                        ));
                    }
                    output.push('\n');
                }
            }
        }

        if !self.clones.is_empty() {
            output.push_str("DETECTED CLONES\n");
            output.push_str(&"-".repeat(80));
            output.push('\n');

            for clone in &self.clones {
                output.push_str(&format!(
                    "Clone Group #{} (length: {} tokens, {} occurrences)\n",
                    clone.id,
                    clone.length,
                    clone.locations.len()
                ));

                for loc in &clone.locations {
                    output.push_str(&format!(
                        "  - {}:{}-{}\n",
                        loc.file.display(),
                        loc.start_line,
                        loc.end_line
                    ));
                }
                output.push('\n');
            }
        }

        output.push_str(&"=".repeat(80));
        output.push('\n');

        output
    }
}

impl Summary {
    fn from_files(files: &[FileReport], clones: &[Clone]) -> Self {
        let total_files = files.len();
        let total_physical_loc = files.iter().map(|f| f.loc.physical).sum();
        let total_logical_loc = files.iter().map(|f| f.loc.logical).sum();

        let complexities: Vec<usize> = files.iter().map(|f| f.cyclomatic.file_complexity).collect();
        let avg_complexity = if !complexities.is_empty() {
            complexities.iter().sum::<usize>() as f64 / complexities.len() as f64
        } else {
            0.0
        };

        let max_complexity = complexities.iter().max().copied().unwrap_or(0);

        let high_complexity_files = files
            .iter()
            .filter(|f| matches!(f.cyclomatic.severity(), Severity::High | Severity::VeryHigh))
            .count();

        let total_clones = clones.len();

        Self {
            total_files,
            total_physical_loc,
            total_logical_loc,
            avg_complexity,
            max_complexity,
            high_complexity_files,
            total_clones,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complexity::FunctionComplexity;

    #[test]
    fn test_empty_report() {
        let report = Report::new(vec![], vec![]);
        assert_eq!(report.summary.total_files, 0);
        assert_eq!(report.summary.total_clones, 0);
    }

    #[test]
    fn test_report_summary() {
        let files = vec![
            FileReport {
                path: PathBuf::from("test1.rs"),
                loc: LocMetrics { physical: 100, logical: 80, comments: 10, blank: 10 },
                cyclomatic: CyclomaticMetrics { file_complexity: 5, functions: vec![] },
            },
            FileReport {
                path: PathBuf::from("test2.rs"),
                loc: LocMetrics { physical: 50, logical: 40, comments: 5, blank: 5 },
                cyclomatic: CyclomaticMetrics { file_complexity: 15, functions: vec![] },
            },
        ];

        let report = Report::new(files, vec![]);

        assert_eq!(report.summary.total_files, 2);
        assert_eq!(report.summary.total_physical_loc, 150);
        assert_eq!(report.summary.total_logical_loc, 120);
        assert_eq!(report.summary.avg_complexity, 10.0);
        assert_eq!(report.summary.max_complexity, 15);
    }

    #[test]
    fn test_to_json() {
        let report = Report::new(vec![], vec![]);
        let json = report.to_json().unwrap();
        assert!(json.contains("files"));
        assert!(json.contains("clones"));
        assert!(json.contains("summary"));
    }

    #[test]
    fn test_to_plaintext() {
        let files = vec![FileReport {
            path: PathBuf::from("test.rs"),
            loc: LocMetrics { physical: 10, logical: 8, comments: 1, blank: 1 },
            cyclomatic: CyclomaticMetrics {
                file_complexity: 3,
                functions: vec![FunctionComplexity { name: "test".to_string(), complexity: 3, line: 1 }],
            },
        }];

        let report = Report::new(files, vec![]);
        let plaintext = report.to_plaintext();

        assert!(plaintext.contains("MCCABRE CODE ANALYSIS REPORT"));
        assert!(plaintext.contains("test.rs"));
        assert!(plaintext.contains("Cyclomatic Complexity"));
    }
}
