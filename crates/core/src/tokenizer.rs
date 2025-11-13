use crate::error::{MccabreError, Result};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Go,
    Java,
    Cpp,
}

impl Language {
    /// Detect language from file extension
    pub fn from_path(path: &Path) -> Result<Self> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| MccabreError::UnsupportedFileType(path.to_string_lossy().to_string()))?;

        match extension {
            "rs" => Ok(Language::Rust),
            "js" | "jsx" | "mjs" | "cjs" => Ok(Language::JavaScript),
            "ts" | "tsx" => Ok(Language::TypeScript),
            "go" => Ok(Language::Go),
            "java" => Ok(Language::Java),
            "cpp" | "cc" | "cxx" | "c++" | "hpp" | "h" | "hh" | "hxx" => Ok(Language::Cpp),
            _ => Err(MccabreError::UnsupportedFileType(extension.to_string())),
        }
    }

    /// Get single-line comment prefix
    pub fn single_line_comment(&self) -> &'static str {
        match self {
            Language::Rust
            | Language::JavaScript
            | Language::TypeScript
            | Language::Go
            | Language::Java
            | Language::Cpp => "//",
        }
    }

    /// Get multi-line comment delimiters (start, end)
    pub fn multi_line_comment(&self) -> (&'static str, &'static str) {
        match self {
            Language::Rust
            | Language::JavaScript
            | Language::TypeScript
            | Language::Go
            | Language::Java
            | Language::Cpp => ("/*", "*/"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    If,
    Else,
    ElseIf,
    While,
    For,
    Loop,
    Match,
    Switch,
    Case,
    Default,
    Catch,

    LogicalAnd,
    LogicalOr,
    Ternary,

    Operator(String),

    Identifier(String),
    Literal(String),

    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,

    Comment,
    Whitespace,
    Newline,
    Unknown(char),
}

impl TokenType {
    /// Returns true if this token contributes to cyclomatic complexity
    pub fn is_decision_point(&self) -> bool {
        matches!(
            self,
            TokenType::If
                | TokenType::ElseIf
                | TokenType::While
                | TokenType::For
                | TokenType::Loop
                | TokenType::Match
                | TokenType::Switch
                | TokenType::Case
                | TokenType::Catch
                | TokenType::LogicalAnd
                | TokenType::LogicalOr
                | TokenType::Ternary
        )
    }

    /// Returns true if this token should be included in clone detection
    pub fn is_significant(&self) -> bool {
        !matches!(self, TokenType::Comment | TokenType::Whitespace | TokenType::Newline)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub text: String,
}

pub struct Tokenizer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    _language: Language,
}

impl Tokenizer {
    pub fn new(source: &str, language: Language) -> Self {
        Self { source: source.chars().collect(), position: 0, line: 1, column: 1, _language: language }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        let start_line = self.line;
        let start_column = self.column;
        let start_pos = self.position;
        let ch = self.current()?;

        if ch.is_whitespace() {
            if ch == '\n' {
                self.advance();
                return Ok(Some(Token {
                    token_type: TokenType::Newline,
                    line: start_line,
                    column: start_column,
                    text: "\n".to_string(),
                }));
            } else {
                while !self.is_at_end() && self.current()?.is_whitespace() && self.current()? != '\n' {
                    self.advance();
                }
                return Ok(Some(Token {
                    token_type: TokenType::Whitespace,
                    line: start_line,
                    column: start_column,
                    text: " ".to_string(),
                }));
            }
        }

        if ch == '/' {
            if self.peek() == Some('/') {
                while !self.is_at_end() && self.current()? != '\n' {
                    self.advance();
                }
                return Ok(Some(Token {
                    token_type: TokenType::Comment,
                    line: start_line,
                    column: start_column,
                    text: "//".to_string(),
                }));
            } else if self.peek() == Some('*') {
                self.advance();
                self.advance();
                while !self.is_at_end() {
                    if self.current()? == '*' && self.peek() == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
                return Ok(Some(Token {
                    token_type: TokenType::Comment,
                    line: start_line,
                    column: start_column,
                    text: "/**/".to_string(),
                }));
            }
        }

        if ch == '"' || ch == '\'' {
            let quote = ch;
            self.advance();
            while !self.is_at_end() && self.current()? != quote {
                if self.current()? == '\\' {
                    self.advance();
                    if !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.advance();
                }
            }
            if !self.is_at_end() {
                self.advance();
            }
            let text: String = self.source[start_pos..self.position].iter().collect();
            return Ok(Some(Token {
                token_type: TokenType::Literal(text.clone()),
                line: start_line,
                column: start_column,
                text,
            }));
        }

        if ch.is_ascii_digit() {
            while !self.is_at_end()
                && (self.current()?.is_ascii_alphanumeric() || self.current()? == '.' || self.current()? == '_')
            {
                self.advance();
            }
            let text: String = self.source[start_pos..self.position].iter().collect();
            return Ok(Some(Token {
                token_type: TokenType::Literal(text.clone()),
                line: start_line,
                column: start_column,
                text,
            }));
        }

        if ch.is_alphabetic() || ch == '_' {
            while !self.is_at_end() && (self.current()?.is_alphanumeric() || self.current()? == '_') {
                self.advance();
            }
            let text: String = self.source[start_pos..self.position].iter().collect();
            let token_type = self.classify_keyword(&text);
            return Ok(Some(Token { token_type, line: start_line, column: start_column, text }));
        }

        let token_type = match ch {
            '{' => {
                self.advance();
                TokenType::LeftBrace
            }
            '}' => {
                self.advance();
                TokenType::RightBrace
            }
            '(' => {
                self.advance();
                TokenType::LeftParen
            }
            ')' => {
                self.advance();
                TokenType::RightParen
            }
            '[' => {
                self.advance();
                TokenType::LeftBracket
            }
            ']' => {
                self.advance();
                TokenType::RightBracket
            }
            ';' => {
                self.advance();
                TokenType::Semicolon
            }
            ',' => {
                self.advance();
                TokenType::Comma
            }
            '?' => {
                self.advance();
                TokenType::Ternary
            }
            '&' if self.peek() == Some('&') => {
                self.advance();
                self.advance();
                TokenType::LogicalAnd
            }
            '|' if self.peek() == Some('|') => {
                self.advance();
                self.advance();
                TokenType::LogicalOr
            }
            _ => {
                let op_chars = "+-*/%=<>!&|^~";
                if op_chars.contains(ch) {
                    while !self.is_at_end() && op_chars.contains(self.current()?) {
                        self.advance();
                    }
                    let text: String = self.source[start_pos..self.position].iter().collect();
                    TokenType::Operator(text)
                } else {
                    self.advance();
                    TokenType::Unknown(ch)
                }
            }
        };

        let text: String = self.source[start_pos..self.position].iter().collect();
        Ok(Some(Token { token_type, line: start_line, column: start_column, text }))
    }

    fn classify_keyword(&self, word: &str) -> TokenType {
        match word {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "elif" => TokenType::ElseIf,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "loop" => TokenType::Loop,
            "match" => TokenType::Match,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "default" => TokenType::Default,
            "catch" => TokenType::Catch,
            _ => TokenType::Identifier(word.to_string()),
        }
    }

    fn current(&self) -> Result<char> {
        self.source
            .get(self.position)
            .copied()
            .ok_or_else(|| MccabreError::TokenizationError("Unexpected end of input".to_string()))
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.source.get(self.position) {
            if *ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_path(Path::new("test.rs")).unwrap(), Language::Rust);
        assert_eq!(Language::from_path(Path::new("test.js")).unwrap(), Language::JavaScript);
        assert_eq!(Language::from_path(Path::new("test.ts")).unwrap(), Language::TypeScript);
        assert_eq!(Language::from_path(Path::new("test.go")).unwrap(), Language::Go);
        assert_eq!(Language::from_path(Path::new("test.java")).unwrap(), Language::Java);
        assert_eq!(Language::from_path(Path::new("test.cpp")).unwrap(), Language::Cpp);
    }

    #[test]
    fn test_tokenize_simple() {
        let source = "if (x > 5) { return true; }";
        let tokenizer = Tokenizer::new(source, Language::Rust);
        let tokens = tokenizer.tokenize().unwrap();

        let significant: Vec<_> = tokens.iter().filter(|t| t.token_type.is_significant()).collect();

        assert!(!significant.is_empty());
        assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::If)));
    }

    #[test]
    fn test_decision_points() {
        let source = "if (x && y || z) { while (true) { } }";
        let tokenizer = Tokenizer::new(source, Language::Rust);
        let tokens = tokenizer.tokenize().unwrap();
        let decision_count = tokens.iter().filter(|t| t.token_type.is_decision_point()).count();
        assert_eq!(decision_count, 4);
    }

    #[test]
    fn test_comments() {
        let source = r#"
// Single line comment
/* Multi-line
   comment */
let x = 5;
"#;
        let tokenizer = Tokenizer::new(source, Language::Rust);
        let tokens = tokenizer.tokenize().unwrap();

        let comments: Vec<_> = tokens
            .iter()
            .filter(|t| matches!(t.token_type, TokenType::Comment))
            .collect();

        assert_eq!(comments.len(), 2);
    }

    #[test]
    fn test_strings() {
        let source = r#"let s = "hello \"world\""; let c = 'x';"#;
        let tokenizer = Tokenizer::new(source, Language::Rust);
        let tokens = tokenizer.tokenize().unwrap();

        let literals: Vec<_> = tokens
            .iter()
            .filter(|t| matches!(t.token_type, TokenType::Literal(_)))
            .collect();

        assert!(literals.len() >= 2);
    }
}
