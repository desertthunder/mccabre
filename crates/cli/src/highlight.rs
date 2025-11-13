use owo_colors::OwoColorize;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Highlighter {
    pub fn new() -> Self {
        Self { syntax_set: SyntaxSet::load_defaults_newlines(), theme_set: ThemeSet::load_defaults() }
    }

    /// Highlight code with syntax highlighting
    pub fn highlight(&self, code: &str, file_extension: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_extension(file_extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];

        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut output = String::new();

        for line in LinesWithEndings::from(code) {
            let ranges = highlighter.highlight_line(line, &self.syntax_set).unwrap_or_default();

            for (style, text) in ranges {
                output.push_str(&style_to_owo(&style, text));
            }
        }

        output
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert syntect Style to owo-colors styled text
fn style_to_owo(style: &Style, text: &str) -> String {
    let fg = style.foreground;

    let colored = if is_grayscale(fg) {
        if fg.r < 100 {
            text.bright_black().to_string()
        } else if fg.r < 180 {
            text.white().to_string()
        } else {
            text.bright_white().to_string()
        }
    } else {
        match dominant_color(fg) {
            ColorChannel::Red => {
                if fg.r > 200 {
                    text.bright_red().to_string()
                } else {
                    text.red().to_string()
                }
            }
            ColorChannel::Green => {
                if fg.g > 200 {
                    text.bright_green().to_string()
                } else {
                    text.green().to_string()
                }
            }
            ColorChannel::Blue => {
                if fg.b > 200 {
                    text.bright_cyan().to_string()
                } else {
                    text.cyan().to_string()
                }
            }
            ColorChannel::Yellow => {
                if fg.r > 200 && fg.g > 200 {
                    text.bright_yellow().to_string()
                } else {
                    text.yellow().to_string()
                }
            }
            ColorChannel::Magenta => {
                if fg.r > 200 && fg.b > 200 {
                    text.bright_magenta().to_string()
                } else {
                    text.magenta().to_string()
                }
            }
        }
    };

    if style.font_style.contains(syntect::highlighting::FontStyle::BOLD) {
        colored.bold().to_string()
    } else {
        colored
    }
}

enum ColorChannel {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
}

fn is_grayscale(color: Color) -> bool {
    let max_diff = color.r.abs_diff(color.g).max(color.g.abs_diff(color.b));
    max_diff < 30
}

fn dominant_color(color: Color) -> ColorChannel {
    let r = color.r as u16;
    let g = color.g as u16;
    let b = color.b as u16;

    if r > 150 && g > 150 && b < 100 {
        return ColorChannel::Yellow;
    }
    if r > 150 && b > 150 && g < 100 {
        return ColorChannel::Magenta;
    }

    if r >= g && r >= b {
        ColorChannel::Red
    } else if g >= r && g >= b {
        ColorChannel::Green
    } else {
        ColorChannel::Blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlighter_creation() {
        let highlighter = Highlighter::new();
        assert!(!highlighter.syntax_set.syntaxes().is_empty());
        assert!(!highlighter.theme_set.themes.is_empty());
    }

    #[test]
    fn test_default_highlighter() {
        let highlighter = Highlighter::default();
        assert!(!highlighter.syntax_set.syntaxes().is_empty());
    }

    #[test]
    fn test_highlight_rust_code() {
        let highlighter = Highlighter::new();
        let code = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let highlighted = highlighter.highlight(code, "rs");

        assert!(!highlighted.is_empty());
        assert!(highlighted.len() >= code.len());
    }

    #[test]
    fn test_highlight_python_code() {
        let highlighter = Highlighter::new();
        let code = "def hello():\n    print('Hello, world!')";
        let highlighted = highlighter.highlight(code, "py");

        assert!(!highlighted.is_empty());
        assert!(highlighted.len() >= code.len());
    }

    #[test]
    fn test_highlight_unknown_extension() {
        let highlighter = Highlighter::new();
        let code = "some random text";
        let highlighted = highlighter.highlight(code, "unknown_ext");

        assert!(!highlighted.is_empty());
    }

    #[test]
    fn test_is_grayscale() {
        assert!(is_grayscale(Color { r: 128, g: 128, b: 128, a: 255 }));
        assert!(is_grayscale(Color { r: 100, g: 105, b: 100, a: 255 }));
        assert!(!is_grayscale(Color { r: 255, g: 0, b: 0, a: 255 }));
        assert!(!is_grayscale(Color { r: 200, g: 100, b: 50, a: 255 }));
    }

    #[test]
    fn test_dominant_color_red() {
        let color = Color { r: 255, g: 50, b: 50, a: 255 };
        matches!(dominant_color(color), ColorChannel::Red);
    }

    #[test]
    fn test_dominant_color_green() {
        let color = Color { r: 50, g: 255, b: 50, a: 255 };
        matches!(dominant_color(color), ColorChannel::Green);
    }

    #[test]
    fn test_dominant_color_blue() {
        let color = Color { r: 50, g: 50, b: 255, a: 255 };
        matches!(dominant_color(color), ColorChannel::Blue);
    }

    #[test]
    fn test_dominant_color_yellow() {
        let color = Color { r: 200, g: 200, b: 50, a: 255 };
        matches!(dominant_color(color), ColorChannel::Yellow);
    }

    #[test]
    fn test_dominant_color_magenta() {
        let color = Color { r: 200, g: 50, b: 200, a: 255 };
        matches!(dominant_color(color), ColorChannel::Magenta);
    }

    #[test]
    fn test_style_to_owo_preserves_text() {
        let style = Style {
            foreground: Color { r: 255, g: 255, b: 255, a: 255 },
            background: Color { r: 0, g: 0, b: 0, a: 255 },
            font_style: syntect::highlighting::FontStyle::empty(),
        };
        let text = "test text";
        let styled = style_to_owo(&style, text);
        assert!(styled.contains(text));
    }
}
