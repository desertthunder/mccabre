use crate::coverage::{CoverageReport, FileCoverage};
use std::io::Write;

pub struct JsonlReporter {
    output: Vec<String>,
}

impl JsonlReporter {
    pub fn new() -> Self {
        Self { output: Vec::new() }
    }

    pub fn add_file(&mut self, file: &FileCoverage) {
        let record = serde_json::to_string(file).expect("Failed to serialize file coverage");
        self.output.push(record);
    }

    pub fn add_report(&mut self, report: &CoverageReport) {
        for file in &report.files {
            self.add_file(file);
        }
    }

    pub fn write_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        for line in &self.output {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    pub fn as_string(&self) -> String {
        self.output.join("\n")
    }
}

impl Default for JsonlReporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coverage::FileCoverage;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    #[test]
    fn test_jsonl_reporter_empty() {
        let reporter = JsonlReporter::new();
        assert!(reporter.output.is_empty());
    }

    #[test]
    fn test_jsonl_reporter_add_file() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let mut reporter = JsonlReporter::new();
        reporter.add_file(&file);

        assert_eq!(reporter.output.len(), 1);
        assert!(reporter.output[0].contains("test.rs"));
    }

    #[test]
    fn test_jsonl_reporter_add_report() {
        let mut lines1 = BTreeMap::new();
        lines1.insert(1, 10);

        let mut lines2 = BTreeMap::new();
        lines2.insert(1, 5);
        lines2.insert(2, 0);

        let file1 = FileCoverage::new("test1.rs".to_string(), lines1);
        let file2 = FileCoverage::new("test2.rs".to_string(), lines2);
        let report = CoverageReport::new(vec![file1, file2]);

        let mut reporter = JsonlReporter::new();
        reporter.add_report(&report);

        assert_eq!(reporter.output.len(), 2);
    }

    #[test]
    fn test_jsonl_reporter_to_string() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let mut reporter = JsonlReporter::new();
        reporter.add_file(&file);

        let output = reporter.as_string();
        assert!(output.contains("test.rs"));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_jsonl_reporter_write_to_file() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let mut reporter = JsonlReporter::new();
        reporter.add_file(&file);

        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("coverage.jsonl");
        reporter.write_to_file(&file_path).unwrap();

        let content = std::fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("test.rs"));
    }

    #[test]
    fn test_jsonl_reporter_serialization() {
        let mut lines = BTreeMap::new();
        lines.insert(1, 10);
        lines.insert(2, 0);
        lines.insert(3, 5);

        let file = FileCoverage::new("test.rs".to_string(), lines);
        let mut reporter = JsonlReporter::new();
        reporter.add_file(&file);

        let output = reporter.as_string();
        let parsed: FileCoverage = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed.path, "test.rs");
        assert_eq!(parsed.lines.len(), 3);
    }
}
