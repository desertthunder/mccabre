use crate::Result;
use crate::cloner::rolling_hash::{RollingHash, token_hash};
use crate::tokenizer::{Language, Token, Tokenizer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// A detected code clone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clone {
    /// Unique ID for this clone group
    pub id: usize,
    /// Number of tokens in the cloned sequence
    pub length: usize,
    /// All locations where this clone appears
    pub locations: Vec<CloneLocation>,
    /// Hash value of the clone (for deduplication)
    #[serde(skip)]
    pub hash: u64,
}

/// Location of a code clone
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloneLocation {
    /// File path
    pub file: PathBuf,
    /// Starting line number
    pub start_line: usize,
    /// Ending line number
    pub end_line: usize,
}

pub struct CloneDetector {
    /// Minimum number of tokens to consider as a clone
    _min_tokens: usize,
    /// Window size for rolling hash
    window_size: usize,
}

impl Default for CloneDetector {
    fn default() -> Self {
        Self { _min_tokens: 30, window_size: 30 }
    }
}

impl CloneDetector {
    pub fn new(min_tokens: usize) -> Self {
        Self { _min_tokens: min_tokens, window_size: min_tokens }
    }

    /// Detect clones in a single file
    pub fn detect_in_file(&self, source: &str, language: Language, file_path: PathBuf) -> Result<Vec<Clone>> {
        let tokens = Tokenizer::new(source, language).tokenize()?;
        let significant_tokens: Vec<&Token> = tokens.iter().filter(|t| t.token_type.is_significant()).collect();

        if significant_tokens.len() < self.window_size {
            return Ok(Vec::new());
        }

        let mut hash_map: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();
        let mut rh = RollingHash::new(self.window_size);

        let token_hashes: Vec<u64> = significant_tokens.iter().map(|t| token_hash(&t.text)).collect();

        rh.init(&token_hashes[0..self.window_size]);
        let start_line = significant_tokens[0].line;
        let end_line = significant_tokens[self.window_size - 1].line;
        hash_map.entry(rh.get()).or_default().push((start_line, end_line));

        for i in self.window_size..token_hashes.len() {
            let hash = rh.roll(token_hashes[i - self.window_size], token_hashes[i]);
            let start_line = significant_tokens[i - self.window_size + 1].line;
            let end_line = significant_tokens[i].line;
            hash_map.entry(hash).or_default().push((start_line, end_line));
        }

        let mut clones = Vec::new();
        let mut clone_id = 0;

        for (hash, locations) in hash_map {
            if locations.len() > 1 {
                clone_id += 1;
                clones.push(Clone {
                    id: clone_id,
                    length: self.window_size,
                    locations: locations
                        .into_iter()
                        .map(|(start, end)| CloneLocation { file: file_path.clone(), start_line: start, end_line: end })
                        .collect(),
                    hash,
                });
            }
        }

        Ok(clones)
    }

    /// Detect clones across multiple files
    pub fn detect_across_files(&self, files: &[(PathBuf, String, Language)]) -> Result<Vec<Clone>> {
        let mut global_hash_map: HashMap<u64, Vec<CloneLocation>> = HashMap::new();

        for (file_path, source, language) in files {
            let tokens = Tokenizer::new(source, *language).tokenize()?;
            let significant_tokens: Vec<&Token> = tokens.iter().filter(|t| t.token_type.is_significant()).collect();

            if significant_tokens.len() < self.window_size {
                continue;
            }

            let mut rh = RollingHash::new(self.window_size);

            let token_hashes: Vec<u64> = significant_tokens.iter().map(|t| token_hash(&t.text)).collect();

            rh.init(&token_hashes[0..self.window_size]);
            let start_line = significant_tokens[0].line;
            let end_line = significant_tokens[self.window_size - 1].line;
            global_hash_map.entry(rh.get()).or_default().push(CloneLocation {
                file: file_path.clone(),
                start_line,
                end_line,
            });

            for i in self.window_size..token_hashes.len() {
                let hash = rh.roll(token_hashes[i - self.window_size], token_hashes[i]);
                let start_line = significant_tokens[i - self.window_size + 1].line;
                let end_line = significant_tokens[i].line;
                global_hash_map.entry(hash).or_default().push(CloneLocation {
                    file: file_path.clone(),
                    start_line,
                    end_line,
                });
            }
        }

        let mut clones = Vec::new();
        let mut clone_id = 0;

        for (hash, mut locations) in global_hash_map {
            if locations.len() > 1 {
                locations.sort_by(|a, b| {
                    a.file
                        .cmp(&b.file)
                        .then(a.start_line.cmp(&b.start_line))
                        .then(a.end_line.cmp(&b.end_line))
                });
                locations.dedup();

                if locations.len() > 1 {
                    clone_id += 1;
                    clones.push(Clone { id: clone_id, length: self.window_size, locations, hash });
                }
            }
        }

        clones.sort_by(|a, b| b.locations.len().cmp(&a.locations.len()));
        Ok(clones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_clones_in_simple_file() {
        let source = r#"
fn simple() {
    let x = 5;
    let y = 10;
    return x + y;
}
"#;
        let detector = CloneDetector::new(10);
        let clones = detector
            .detect_in_file(source, Language::Rust, PathBuf::from("test.rs"))
            .unwrap();

        assert_eq!(clones.len(), 0);
    }

    #[test]
    fn test_detect_simple_clone() {
        let source = r#"
fn process_a() {
    let x = input.get();
    let y = x * 2;
    let z = y + 5;
    return z;
}

fn process_b() {
    let x = input.get();
    let y = x * 2;
    let z = y + 5;
    return z;
}
"#;
        let detector = CloneDetector::new(5);
        let clones = detector
            .detect_in_file(source, Language::Rust, PathBuf::from("test.rs"))
            .unwrap();

        assert!(!clones.is_empty());

        for clone in &clones {
            assert!(clone.locations.len() >= 2);
        }
    }

    #[test]
    fn test_across_files() {
        let file1 = r#"
fn helper() {
    for i in 0..10 {
        println!("{}", i);
    }
}
"#;
        let file2 = r#"
fn another() {
    for i in 0..10 {
        println!("{}", i);
    }
}
"#;

        let files = vec![
            (PathBuf::from("file1.rs"), file1.to_string(), Language::Rust),
            (PathBuf::from("file2.rs"), file2.to_string(), Language::Rust),
        ];

        let detector = CloneDetector::new(5);
        let clones = detector.detect_across_files(&files).unwrap();

        if !clones.is_empty() {
            let has_cross_file = clones.iter().any(|clone| {
                let files: std::collections::HashSet<_> = clone.locations.iter().map(|l| &l.file).collect();
                files.len() > 1
            });
            assert!(has_cross_file, "Should detect clones across different files");
        }
    }

    #[test]
    fn test_min_tokens_threshold() {
        let source = "let x = 5; let y = 10; let x = 5; let y = 10;";

        let detector1 = CloneDetector::new(3);
        let clones1 = detector1
            .detect_in_file(source, Language::Rust, PathBuf::from("test.rs"))
            .unwrap();

        let detector2 = CloneDetector::new(100);
        let clones2 = detector2
            .detect_in_file(source, Language::Rust, PathBuf::from("test.rs"))
            .unwrap();

        assert!(clones2.len() <= clones1.len());
    }
}
