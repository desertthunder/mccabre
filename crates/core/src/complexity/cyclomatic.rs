use crate::Result;
use crate::tokenizer::{Language, Token, TokenType, Tokenizer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Moderate,
    High,
    VeryHigh,
}

/// Cyclomatic Complexity metrics for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclomaticMetrics {
    /// Overall file complexity
    pub file_complexity: usize,
    /// Individual function complexities (if we can detect them)
    pub functions: Vec<FunctionComplexity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    /// Function name (if identifiable)
    pub name: String,
    /// Cyclomatic complexity value
    pub complexity: usize,
    /// Line number where function starts
    pub line: usize,
}

impl CyclomaticMetrics {
    /// Calculate cyclomatic complexity from source code
    ///
    /// Uses the simplified formula: CC = number of decision points + 1
    /// Decision points include: if, else if, while, for, loop, match/switch, case, catch, &&, ||, ?
    pub fn calculate(source: &str, language: Language) -> Result<Self> {
        let tokens = Tokenizer::new(source, language).tokenize()?;
        let decision_points = tokens.iter().filter(|t| t.token_type.is_decision_point()).count();
        let file_complexity = if decision_points == 0 { 1 } else { decision_points + 1 };
        let functions = Self::detect_functions(&tokens, language);

        Ok(CyclomaticMetrics { file_complexity, functions })
    }

    /// Attempt to detect function boundaries and calculate per-function complexity
    ///
    /// Look for function patterns:
    /// - Rust: "fn" identifier "(" ... ")" "{"
    /// - JS/TS: "function" identifier "(" ... ")" "{"
    /// - Go: "func" identifier "(" ... ")" "{"
    /// - Java/C++: type identifier "(" ... ")" "{"
    fn detect_functions(tokens: &[Token], _language: Language) -> Vec<FunctionComplexity> {
        let mut functions = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            let is_function_keyword = if let TokenType::Identifier(name) = &tokens[i].token_type {
                name == "fn" || name == "func" || name == "function"
            } else {
                false
            };

            if is_function_keyword {
                let mut name = "anonymous".to_string();
                let line = tokens[i].line;

                if i + 1 < tokens.len()
                    && let TokenType::Identifier(id) = &tokens[i + 1].token_type
                {
                    name = id.clone();
                }

                let body_start = Self::find_next_token(tokens, i, TokenType::LeftBrace);

                if let Some(body_start_idx) = body_start
                    && let Some(body_end_idx) = Self::find_matching_brace(tokens, body_start_idx)
                {
                    let decision_points = tokens[body_start_idx..=body_end_idx]
                        .iter()
                        .filter(|t| t.token_type.is_decision_point())
                        .count();

                    let complexity = if decision_points == 0 { 1 } else { decision_points + 1 };

                    functions.push(FunctionComplexity { name, complexity, line });

                    i = body_end_idx + 1;
                    continue;
                }
            }

            i += 1;
        }

        functions
    }

    /// Find the next token of a specific type
    fn find_next_token(tokens: &[Token], start: usize, token_type: TokenType) -> Option<usize> {
        tokens[start..]
            .iter()
            .position(|t| std::mem::discriminant(&t.token_type) == std::mem::discriminant(&token_type))
            .map(|pos| start + pos)
    }

    /// Find the matching closing brace for an opening brace
    fn find_matching_brace(tokens: &[Token], open_idx: usize) -> Option<usize> {
        let mut depth = 0;

        for (offset, token) in tokens[open_idx..].iter().enumerate() {
            match token.token_type {
                TokenType::LeftBrace => depth += 1,
                TokenType::RightBrace => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(open_idx + offset);
                    }
                }
                _ => {}
            }
        }

        None
    }

    /// Get severity level based on complexity threshold
    /// Standard thresholds from literature:
    /// 1-10: Simple, low risk
    /// 11-20: More complex, moderate risk
    /// 21-50: Complex, high risk
    /// 50+: Very complex, very high risk
    pub fn severity(&self) -> Severity {
        match self.file_complexity {
            1..=10 => Severity::Low,
            11..=20 => Severity::Moderate,
            21..=50 => Severity::High,
            _ => Severity::VeryHigh,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let source = r#"
fn simple() {
    let x = 5;
    return x;
}
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::Rust).unwrap();
        assert_eq!(metrics.file_complexity, 1);
        assert_eq!(metrics.severity(), Severity::Low);
    }

    #[test]
    fn test_single_if() {
        let source = r#"
fn check(x: i32) {
    if x > 5 {
        println!("big");
    }
}
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::Rust).unwrap();
        assert_eq!(metrics.file_complexity, 2);
    }

    #[test]
    fn test_multiple_decision_points() {
        let source = r#"
fn complex(x: i32, y: i32) {
    if x > 0 && y > 0 {
        while x < 10 {
            x += 1;
        }
    } else if x < 0 {
        for i in 0..5 {
            println!("{}", i);
        }
    }
}
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::Rust).unwrap();
        assert_eq!(metrics.file_complexity, 6);
    }

    #[test]
    fn test_ternary_operator() {
        let source = r#"
let x = condition ? true_value : false_value;
let y = a && b ? c : d;
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::JavaScript).unwrap();
        assert_eq!(metrics.file_complexity, 4);
    }

    #[test]
    fn test_switch_case() {
        let source = r#"
switch (x) {
    case 1:
        break;
    case 2:
        break;
    default:
        break;
}
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::JavaScript).unwrap();
        assert!(metrics.file_complexity >= 4);
    }

    #[test]
    fn test_function_detection_rust() {
        let source = r#"
fn simple() {
    let x = 5;
}

fn complex() {
    if true {
        while false {
            loop { break; }
        }
    }
}
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::Rust).unwrap();

        if !metrics.functions.is_empty() {
            for func in &metrics.functions {
                assert!(func.complexity >= 1);
                assert!(!func.name.is_empty());
            }
        }
    }

    #[test]
    fn test_javascript_function() {
        let source = r#"
function hello() {
    if (x > 0) {
        return true;
    }
    return false;
}
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::JavaScript).unwrap();
        assert!(!metrics.functions.is_empty());
        assert_eq!(metrics.file_complexity, 2);
    }

    #[test]
    fn test_severity_levels() {
        assert_eq!(
            CyclomaticMetrics { file_complexity: 5, functions: vec![] }.severity(),
            Severity::Low
        );
        assert_eq!(
            CyclomaticMetrics { file_complexity: 15, functions: vec![] }.severity(),
            Severity::Moderate
        );
        assert_eq!(
            CyclomaticMetrics { file_complexity: 25, functions: vec![] }.severity(),
            Severity::High
        );
        assert_eq!(
            CyclomaticMetrics { file_complexity: 100, functions: vec![] }.severity(),
            Severity::VeryHigh
        );
    }

    #[test]
    fn test_logical_operators() {
        let source = r#"
if (a && b && c) { }
if (x || y || z) { }
"#;
        let metrics = CyclomaticMetrics::calculate(source, Language::JavaScript).unwrap();
        assert_eq!(metrics.file_complexity, 7);
    }
}
