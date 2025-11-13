use crate::Result;
use crate::tokenizer::{Language, TokenType, Tokenizer};
use serde::{Deserialize, Serialize};

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
}
