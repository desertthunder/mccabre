use crate::Result;
use crate::tokenizer::{Language, TokenType, Tokenizer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineKind {
    Code,
    Comment,
    Blank,
}

/// Lines of Code metrics for a single file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocMetrics {
    /// Total number of lines in the file
    pub physical: usize,
    /// Number of non-blank, non-comment lines
    pub logical: usize,
    /// Number of comment lines
    pub comments: usize,
    /// Number of blank lines
    pub blank: usize,
}

impl LocMetrics {
    pub fn calculate(source: &str, language: Language) -> Result<Self> {
        let tokens = Tokenizer::new(source, language).tokenize()?;
        let physical = if source.is_empty() { 0 } else { source.split('\n').count() };
        let mut line_types = vec![LineKind::Blank; physical];

        for token in &tokens {
            let line_idx = token.line.saturating_sub(1);
            if line_idx >= line_types.len() {
                continue;
            }

            match token.token_type {
                _ if token.token_type.is_significant() => {
                    line_types[line_idx] = LineKind::Code;
                }
                TokenType::Comment => {
                    if line_types[line_idx] != LineKind::Code {
                        line_types[line_idx] = LineKind::Comment;
                    }
                }
                _ => {}
            }
        }

        for (idx, line) in source.lines().enumerate() {
            if line.trim().is_empty() && idx < line_types.len() {
                line_types[idx] = LineKind::Blank;
            }
        }

        let comments = line_types.iter().filter(|&&t| t == LineKind::Comment).count();
        let blank = line_types.iter().filter(|&&t| t == LineKind::Blank).count();
        let logical = physical - comments - blank;

        Ok(LocMetrics { physical, logical, comments, blank })
    }

    /// Add two LocMetrics together
    fn add(&self, other: &Self) -> Self {
        Self {
            physical: self.physical + other.physical,
            logical: self.logical + other.logical,
            comments: self.comments + other.comments,
            blank: self.blank + other.blank,
        }
    }
}

/// Ranking criteria for LOC analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RankBy {
    /// Rank by logical lines of code
    Logical,
    /// Rank by physical lines of code
    Physical,
    /// Rank by comment lines
    Comments,
    /// Rank by blank lines
    Blank,
}

impl RankBy {
    /// Get the value from LocMetrics based on ranking criteria
    pub fn value_from(&self, metrics: &LocMetrics) -> usize {
        match self {
            Self::Logical => metrics.logical,
            Self::Physical => metrics.physical,
            Self::Comments => metrics.comments,
            Self::Blank => metrics.blank,
        }
    }
}

/// LOC metrics for a single file with path information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLocReport {
    /// File path
    pub path: PathBuf,
    /// LOC metrics
    pub metrics: LocMetrics,
}

/// Aggregated LOC metrics for a directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryLocMetrics {
    /// Directory path
    pub path: PathBuf,
    /// Total LOC metrics for all files in this directory
    pub total: LocMetrics,
    /// Files in this directory
    pub files: Vec<FileLocReport>,
}

/// Complete LOC analysis report with ranking capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocReport {
    /// Per-file reports
    pub files: Vec<FileLocReport>,
    /// Per-directory aggregation (if enabled)
    pub directories: Option<Vec<DirectoryLocMetrics>>,
    /// Summary statistics
    pub summary: LocSummary,
}

/// Summary statistics for LOC report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocSummary {
    /// Total number of files analyzed
    pub total_files: usize,
    /// Total physical lines of code
    pub total_physical: usize,
    /// Total logical lines of code
    pub total_logical: usize,
    /// Total comment lines
    pub total_comments: usize,
    /// Total blank lines
    pub total_blank: usize,
}

impl LocReport {
    /// Create a new LOC report from file reports
    pub fn new(mut files: Vec<FileLocReport>, rank_by: RankBy, rank_dirs: bool) -> Self {
        files.sort_by(|a, b| rank_by.value_from(&b.metrics).cmp(&rank_by.value_from(&a.metrics)));

        let directories = if rank_dirs { Some(Self::aggregate_by_directory(&files, rank_by)) } else { None };
        let summary = LocSummary::from_files(&files);

        Self { files, directories, summary }
    }

    /// Aggregate files by directory
    fn aggregate_by_directory(files: &[FileLocReport], rank_by: RankBy) -> Vec<DirectoryLocMetrics> {
        let mut dir_map: HashMap<PathBuf, Vec<FileLocReport>> = HashMap::new();

        for file in files {
            let dir = file.path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
            dir_map.entry(dir).or_default().push(file.clone());
        }

        let mut directories: Vec<DirectoryLocMetrics> = dir_map
            .into_iter()
            .map(|(path, files)| {
                let total = files.iter().fold(
                    LocMetrics { physical: 0, logical: 0, comments: 0, blank: 0 },
                    |acc, f| acc.add(&f.metrics),
                );

                let mut sorted_files = files;
                sorted_files.sort_by(|a, b| rank_by.value_from(&b.metrics).cmp(&rank_by.value_from(&a.metrics)));

                DirectoryLocMetrics { path, total, files: sorted_files }
            })
            .collect();

        directories.sort_by(|a, b| rank_by.value_from(&b.total).cmp(&rank_by.value_from(&a.total)));

        directories
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl LocSummary {
    fn from_files(files: &[FileLocReport]) -> Self {
        let total_files = files.len();
        let total_physical = files.iter().map(|f| f.metrics.physical).sum();
        let total_logical = files.iter().map(|f| f.metrics.logical).sum();
        let total_comments = files.iter().map(|f| f.metrics.comments).sum();
        let total_blank = files.iter().map(|f| f.metrics.blank).sum();

        Self { total_files, total_physical, total_logical, total_comments, total_blank }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_file() {
        let source = "";
        let metrics = LocMetrics::calculate(source, Language::Rust).unwrap();
        assert_eq!(metrics.physical, 0);
        assert_eq!(metrics.logical, 0);
    }

    #[test]
    fn test_simple_code() {
        let source = r#"
fn main() {
    println!("Hello");
}
"#;
        let metrics = LocMetrics::calculate(source, Language::Rust).unwrap();
        assert_eq!(metrics.physical, 5);
        assert!(metrics.logical >= 2);
        assert!(metrics.blank >= 1);
    }

    #[test]
    fn test_comments() {
        let source = r#"
// This is a comment
/* Multi-line
   comment */
let x = 5; // inline comment
"#;
        let metrics = LocMetrics::calculate(source, Language::Rust).unwrap();
        assert!(metrics.comments >= 2);
        assert!(metrics.logical >= 1);
    }

    #[test]
    fn test_blank_lines() {
        let source = r#"


fn test() {}


"#;
        let metrics = LocMetrics::calculate(source, Language::Rust).unwrap();
        assert!(metrics.blank >= 4);
        assert!(metrics.logical >= 1);
    }

    #[test]
    fn test_javascript() {
        let source = r#"
function hello() {
    console.log("Hello");
}
"#;
        let metrics = LocMetrics::calculate(source, Language::JavaScript).unwrap();
        assert_eq!(metrics.physical, 5);
        assert!(metrics.logical >= 2);
    }

    #[test]
    fn test_all_comments() {
        let source = r#"
// Comment 1
// Comment 2
/* Comment 3 */
"#;
        let metrics = LocMetrics::calculate(source, Language::Rust).unwrap();
        assert!(metrics.comments >= 3);
        assert_eq!(metrics.logical, 0);
    }

    #[test]
    fn test_loc_metrics_add() {
        let m1 = LocMetrics { physical: 10, logical: 8, comments: 1, blank: 1 };
        let m2 = LocMetrics { physical: 20, logical: 15, comments: 3, blank: 2 };
        let result = m1.add(&m2);

        assert_eq!(result.physical, 30);
        assert_eq!(result.logical, 23);
        assert_eq!(result.comments, 4);
        assert_eq!(result.blank, 3);
    }

    #[test]
    fn test_rank_by_value_from() {
        let metrics = LocMetrics { physical: 100, logical: 80, comments: 10, blank: 10 };

        assert_eq!(RankBy::Physical.value_from(&metrics), 100);
        assert_eq!(RankBy::Logical.value_from(&metrics), 80);
        assert_eq!(RankBy::Comments.value_from(&metrics), 10);
        assert_eq!(RankBy::Blank.value_from(&metrics), 10);
    }

    #[test]
    fn test_loc_report_new() {
        let files = vec![
            FileLocReport {
                path: PathBuf::from("test1.rs"),
                metrics: LocMetrics { physical: 100, logical: 80, comments: 10, blank: 10 },
            },
            FileLocReport {
                path: PathBuf::from("test2.rs"),
                metrics: LocMetrics { physical: 50, logical: 40, comments: 5, blank: 5 },
            },
        ];

        let report = LocReport::new(files, RankBy::Logical, false);

        assert_eq!(report.summary.total_files, 2);
        assert_eq!(report.summary.total_physical, 150);
        assert_eq!(report.summary.total_logical, 120);
        assert_eq!(report.summary.total_comments, 15);
        assert_eq!(report.summary.total_blank, 15);

        assert_eq!(report.files[0].metrics.logical, 80);
        assert_eq!(report.files[1].metrics.logical, 40);
    }

    #[test]
    fn test_loc_report_with_directories() {
        let files = vec![
            FileLocReport {
                path: PathBuf::from("src/main.rs"),
                metrics: LocMetrics { physical: 100, logical: 80, comments: 10, blank: 10 },
            },
            FileLocReport {
                path: PathBuf::from("src/lib.rs"),
                metrics: LocMetrics { physical: 50, logical: 40, comments: 5, blank: 5 },
            },
            FileLocReport {
                path: PathBuf::from("tests/test.rs"),
                metrics: LocMetrics { physical: 30, logical: 25, comments: 3, blank: 2 },
            },
        ];

        let report = LocReport::new(files, RankBy::Logical, true);

        assert!(report.directories.is_some());
        let dirs = report.directories.unwrap();
        assert_eq!(dirs.len(), 2);

        assert_eq!(dirs[0].path, PathBuf::from("src"));
        assert_eq!(dirs[0].total.logical, 120);
        assert_eq!(dirs[0].files.len(), 2);

        assert_eq!(dirs[1].path, PathBuf::from("tests"));
        assert_eq!(dirs[1].total.logical, 25);
        assert_eq!(dirs[1].files.len(), 1);
    }

    #[test]
    fn test_loc_report_to_json() {
        let files = vec![FileLocReport {
            path: PathBuf::from("test.rs"),
            metrics: LocMetrics { physical: 10, logical: 8, comments: 1, blank: 1 },
        }];

        let report = LocReport::new(files, RankBy::Logical, false);
        let json = report.to_json().unwrap();

        assert!(json.contains("files"));
        assert!(json.contains("summary"));
        assert!(json.contains("test.rs"));
    }
}
