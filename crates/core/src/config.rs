use crate::error::{MccabreError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Configuration for mccabre analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Cyclomatic complexity thresholds
    #[serde(default)]
    pub complexity: ComplexityConfig,

    /// Clone detection settings
    #[serde(default)]
    pub clones: CloneConfig,

    /// File filtering settings
    #[serde(default)]
    pub files: FileConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityConfig {
    /// Threshold for warning level (default: 10)
    #[serde(default = "default_warning_threshold")]
    pub warning_threshold: usize,

    /// Threshold for error level (default: 20)
    #[serde(default = "default_error_threshold")]
    pub error_threshold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneConfig {
    /// Minimum number of tokens for clone detection (default: 30)
    #[serde(default = "default_min_tokens")]
    pub min_tokens: usize,

    /// Whether to enable clone detection (default: true)
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    /// Whether to respect .gitignore (default: true)
    #[serde(default = "default_true")]
    pub respect_gitignore: bool,
}

impl Default for ComplexityConfig {
    fn default() -> Self {
        Self { warning_threshold: default_warning_threshold(), error_threshold: default_error_threshold() }
    }
}

impl Default for CloneConfig {
    fn default() -> Self {
        Self { min_tokens: default_min_tokens(), enabled: default_true() }
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self { respect_gitignore: default_true() }
    }
}

fn default_warning_threshold() -> usize {
    10
}

fn default_error_threshold() -> usize {
    20
}

fn default_min_tokens() -> usize {
    30
}

fn default_true() -> bool {
    true
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| MccabreError::FileRead { path: path.as_ref().to_path_buf(), source: e })?;

        toml::from_str(&content).map_err(|e| MccabreError::InvalidConfig(e.to_string()))
    }

    /// Try to load configuration from default locations
    /// Looks for: mccabre.toml, .mccabre.toml, .mccabre/config.toml
    pub fn load_default() -> Result<Self> {
        let candidates = vec!["mccabre.toml", ".mccabre.toml", ".mccabre/config.toml"];

        for path in candidates {
            if Path::new(path).exists() {
                return Self::from_file(path);
            }
        }

        Ok(Self::default())
    }

    /// Save configuration to a TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self).map_err(|e| MccabreError::InvalidConfig(e.to_string()))?;

        fs::write(path.as_ref(), content)
            .map_err(|e| MccabreError::FileRead { path: path.as_ref().to_path_buf(), source: e })?;

        Ok(())
    }

    /// Merge with CLI overrides
    pub fn merge_with_cli(
        mut self, complexity_threshold: Option<usize>, min_tokens: Option<usize>, respect_gitignore: Option<bool>,
    ) -> Self {
        if let Some(threshold) = complexity_threshold {
            self.complexity.warning_threshold = threshold;
        }

        if let Some(min) = min_tokens {
            self.clones.min_tokens = min;
        }

        if let Some(respect) = respect_gitignore {
            self.files.respect_gitignore = respect;
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.complexity.warning_threshold, 10);
        assert_eq!(config.complexity.error_threshold, 20);
        assert_eq!(config.clones.min_tokens, 30);
        assert!(config.clones.enabled);
        assert!(config.files.respect_gitignore);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let config = Config::default();
        config.save(&config_path).unwrap();

        let loaded = Config::from_file(&config_path).unwrap();
        assert_eq!(loaded.complexity.warning_threshold, config.complexity.warning_threshold);
        assert_eq!(loaded.clones.min_tokens, config.clones.min_tokens);
    }

    #[test]
    fn test_merge_with_cli() {
        let mut config = Config::default();
        config = config.merge_with_cli(Some(15), Some(40), Some(false));

        assert_eq!(config.complexity.warning_threshold, 15);
        assert_eq!(config.clones.min_tokens, 40);
        assert!(!config.files.respect_gitignore);
    }

    #[test]
    fn test_partial_cli_override() {
        let mut config = Config::default();
        config = config.merge_with_cli(Some(25), None, None);

        assert_eq!(config.complexity.warning_threshold, 25);
        assert_eq!(config.clones.min_tokens, 30);
        assert!(config.files.respect_gitignore);
    }
}
