# Introduction

**Mccabre** language-agnostic code complexity and clone detection tool designed to help developers identify problematic & repeated code patterns.

## Features

- **Cyclomatic Complexity Analysis**: Measure control-flow complexity using McCabe's algorithm
- **Lines of Code Metrics**: Count physical, logical, comment, and blank lines
- **Clone Detection**: Find duplicated code using Rabin-Karp rolling hash
- **Multi-Language Support**: Rust, JavaScript/TypeScript, Go, Java, and C++
- **Gitignore Aware**: Automatically respects .gitignore files
- **Multiple Output Formats**: Beautiful terminal output or JSON

## Design Philosophy

Mccabre prioritizes:

1. **Speed**: Linear or near-linear performance through tokenization instead of full parsing
2. **Simplicity**: Easy to use with sensible defaults
3. **Actionability**: Clear, color-coded output highlighting issues
4. **Extensibility**: Modular design allowing future enhancements

## Limitations

- **Token-based**: Function detection is heuristic-based and may miss some functions
- **Language Support**: Currently supports C-style languages; Python coming soon
- **Clone Detection**: Finds exact token matches, not semantic equivalence (yet)

See the [Quick Start](./quickstart.md) guide to begin using Mccabre.
