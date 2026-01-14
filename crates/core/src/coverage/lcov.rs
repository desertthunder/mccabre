use crate::{MccabreError, Result};
use std::collections::BTreeMap;
use std::path::Path;

pub fn parse_lcov_file(path: &Path, repo_root: Option<&Path>) -> Result<Vec<FileCoverage>> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| MccabreError::Io(std::io::Error::other(format!("Failed to read LCOV file: {e}"))))?;

    parse_lcov_content(&content, repo_root)
}

pub fn parse_lcov_content(content: &str, repo_root: Option<&Path>) -> Result<Vec<FileCoverage>> {
    let mut files: std::collections::HashMap<String, BTreeMap<u32, u64>> = std::collections::HashMap::new();
    let mut current_file: Option<String> = None;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("SF:") {
            let path = super::paths::normalize_path(rest, repo_root);
            current_file = Some(path);
            files.entry(current_file.clone().unwrap()).or_default();
        } else if let Some(rest) = line.strip_prefix("DA:") {
            if let Some(ref file) = current_file
                && let Some((line_num, count)) = rest.split_once(',')
                && let (Ok(line_num), Ok(count)) = (line_num.parse::<u32>(), count.parse::<u64>())
            {
                files.entry(file.clone()).or_default().insert(line_num, count);
            }
        } else if line == "end_of_record" {
            current_file = None;
        }
    }

    let mut file_coverages = Vec::new();
    for (path, lines) in files {
        file_coverages.push(FileCoverage::new(path, lines));
    }

    file_coverages.sort_by(|a, b| {
        a.summary
            .rate
            .partial_cmp(&b.summary.rate)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(file_coverages)
}

use super::model::FileCoverage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_lcov() {
        let lcov = r#"SF:test.rs
DA:1,10
DA:2,5
DA:3,0
end_of_record
"#;

        let files = parse_lcov_content(lcov, None).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "test.rs");
        assert_eq!(files[0].lines.len(), 3);
        assert_eq!(files[0].lines.get(&1), Some(&10));
        assert_eq!(files[0].lines.get(&2), Some(&5));
        assert_eq!(files[0].lines.get(&3), Some(&0));
    }

    #[test]
    fn test_parse_multiple_files() {
        let lcov = r#"SF:test1.rs
DA:1,10
DA:2,5
end_of_record
SF:test2.rs
DA:1,1
DA:2,0
DA:3,0
end_of_record
"#;

        let files = parse_lcov_content(lcov, None).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.path == "test1.rs"));
        assert!(files.iter().any(|f| f.path == "test2.rs"));
    }

    #[test]
    fn test_parse_with_repo_root() {
        let lcov = r#"SF:/repo/src/lib.rs
DA:1,10
DA:2,0
end_of_record
"#;

        let root = Path::new("/repo");
        let files = parse_lcov_content(lcov, Some(root)).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "src/lib.rs");
    }

    #[test]
    fn test_parse_empty_lcov() {
        let lcov = "";
        let files = parse_lcov_content(lcov, None).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_parse_invalid_lines() {
        let lcov = r#"SF:test.rs
DA:invalid,10
DA:1,5
end_of_record
"#;

        let files = parse_lcov_content(lcov, None).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].lines.get(&1), Some(&5));
    }

    #[test]
    fn test_parse_sorted_by_coverage_rate() {
        let lcov = r#"SF:full.rs
DA:1,10
DA:2,10
end_of_record
SF:partial.rs
DA:1,10
DA:2,0
end_of_record
SF:none.rs
DA:1,0
DA:2,0
end_of_record
"#;

        let files = parse_lcov_content(lcov, None).unwrap();
        assert_eq!(files.len(), 3);
        assert_eq!(files[0].path, "none.rs");
        assert_eq!(files[1].path, "partial.rs");
        assert_eq!(files[2].path, "full.rs");
    }
}
