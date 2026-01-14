use serde::{Deserialize, Serialize};

/// Coverage report for the entire codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    pub files: Vec<FileCoverage>,
    pub totals: CoverageSummary,
}

impl CoverageReport {
    pub fn new(files: Vec<FileCoverage>) -> Self {
        let totals = CoverageSummary::from_files(&files);
        Self { files, totals }
    }
}

/// Coverage data for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    pub path: String,
    pub lines: std::collections::BTreeMap<u32, u64>,
    pub miss_ranges: Vec<(u32, u32)>,
    pub summary: CoverageSummary,
}

impl FileCoverage {
    pub fn new(path: String, lines: std::collections::BTreeMap<u32, u64>) -> Self {
        let summary = CoverageSummary::from_lines(&lines);
        let miss_ranges = super::misses::compute_miss_ranges(&lines);
        Self { path, lines, miss_ranges, summary }
    }
}

/// Coverage summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageSummary {
    pub total: usize,
    pub hit: usize,
    pub miss: usize,
    pub rate: f64,
}

impl CoverageSummary {
    pub fn from_lines(lines: &std::collections::BTreeMap<u32, u64>) -> Self {
        let total = lines.len();
        let hit = lines.values().filter(|&&c| c > 0).count();
        let miss = lines.values().filter(|&&c| c == 0).count();
        let rate = if total > 0 { (hit as f64 / total as f64) * 100.0 } else { 0.0 };

        Self { total, hit, miss, rate }
    }

    pub fn from_files(files: &[FileCoverage]) -> Self {
        let total: usize = files.iter().map(|f| f.summary.total).sum();
        let hit: usize = files.iter().map(|f| f.summary.hit).sum();
        let miss: usize = files.iter().map(|f| f.summary.miss).sum();
        let rate = if total > 0 { (hit as f64 / total as f64) * 100.0 } else { 0.0 };

        Self { total, hit, miss, rate }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage_summary_full() {
        let mut lines = std::collections::BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 5);
        lines.insert(3, 1);

        let summary = CoverageSummary::from_lines(&lines);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.hit, 3);
        assert_eq!(summary.miss, 0);
        assert_eq!(summary.rate, 100.0);
    }

    #[test]
    fn test_coverage_summary_partial() {
        let mut lines = std::collections::BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 5);

        let summary = CoverageSummary::from_lines(&lines);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.hit, 2);
        assert_eq!(summary.miss, 1);
        assert!((summary.rate - 66.66666666666666).abs() < 0.0001);
    }

    #[test]
    fn test_coverage_summary_empty() {
        let lines = std::collections::BTreeMap::new();
        let summary = CoverageSummary::from_lines(&lines);
        assert_eq!(summary.total, 0);
        assert_eq!(summary.hit, 0);
        assert_eq!(summary.miss, 0);
        assert_eq!(summary.rate, 0.0);
    }

    #[test]
    fn test_file_coverage() {
        let mut lines = std::collections::BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 5);

        let file = FileCoverage::new("test.rs".to_string(), lines.clone());
        assert_eq!(file.path, "test.rs");
        assert_eq!(file.summary.total, 3);
        assert_eq!(file.summary.hit, 2);
        assert_eq!(file.summary.miss, 1);
        assert_eq!(file.miss_ranges, vec![(2, 2)]);
    }
}
