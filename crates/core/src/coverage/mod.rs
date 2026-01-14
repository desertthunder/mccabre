pub mod lcov;
pub mod misses;
pub mod model;
pub mod paths;

pub use lcov::parse_lcov_content;
pub use lcov::parse_lcov_file;
pub use model::{CoverageReport, CoverageSummary, FileCoverage};

use crate::Result;

pub fn parse_coverage_from_file(path: &std::path::Path, repo_root: Option<&std::path::Path>) -> Result<CoverageReport> {
    let files = lcov::parse_lcov_file(path, repo_root)?;
    Ok(CoverageReport::new(files))
}

pub fn parse_coverage_from_content(content: &str, repo_root: Option<&std::path::Path>) -> Result<CoverageReport> {
    let files = lcov::parse_lcov_content(content, repo_root)?;
    Ok(CoverageReport::new(files))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coverage_from_content() {
        let lcov = r#"SF:test.rs
DA:1,10
DA:2,0
DA:3,5
end_of_record
"#;

        let report = parse_coverage_from_content(lcov, None).unwrap();
        assert_eq!(report.files.len(), 1);
        assert_eq!(report.totals.total, 3);
        assert_eq!(report.totals.hit, 2);
        assert_eq!(report.totals.miss, 1);
    }

    #[test]
    fn test_parse_coverage_multiple_files() {
        let lcov = r#"SF:test1.rs
DA:1,10
DA:2,0
end_of_record
SF:test2.rs
DA:1,5
DA:2,5
end_of_record
"#;

        let report = parse_coverage_from_content(lcov, None).unwrap();
        assert_eq!(report.files.len(), 2);
        assert_eq!(report.totals.total, 4);
        assert_eq!(report.totals.hit, 3);
        assert_eq!(report.totals.miss, 1);
    }
}
