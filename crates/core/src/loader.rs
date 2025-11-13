use crate::error::{MccabreError, Result};
use crate::tokenizer::Language;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// File entry with source code and metadata
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    pub language: Language,
}

/// File loader that respects .gitignore and supports various input types
pub struct FileLoader {
    /// Whether to respect .gitignore files
    respect_gitignore: bool,
}

impl Default for FileLoader {
    fn default() -> Self {
        Self { respect_gitignore: true }
    }
}

impl FileLoader {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable gitignore awareness
    pub fn with_gitignore(mut self, respect: bool) -> Self {
        self.respect_gitignore = respect;
        self
    }

    /// Load files from a path (file, directory, or list)
    pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<Vec<SourceFile>> {
        let path = path.as_ref();

        if path.is_file() {
            let file = self.load_file(path)?;
            Ok(vec![file])
        } else if path.is_dir() {
            self.load_directory(path)
        } else {
            Err(MccabreError::FileRead {
                path: path.to_path_buf(),
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "Path is neither a file nor a directory"),
            })
        }
    }

    /// Load multiple paths
    pub fn load_multiple<P: AsRef<Path>>(&self, paths: &[P]) -> Result<Vec<SourceFile>> {
        let mut files = Vec::new();

        for path in paths {
            let mut loaded = self.load(path)?;
            files.append(&mut loaded);
        }

        files.sort_by(|a, b| a.path.cmp(&b.path));
        files.dedup_by(|a, b| a.path == b.path);

        Ok(files)
    }

    /// Load a single file
    fn load_file(&self, path: &Path) -> Result<SourceFile> {
        let language = Language::from_path(path)?;
        let content =
            fs::read_to_string(path).map_err(|e| MccabreError::FileRead { path: path.to_path_buf(), source: e })?;

        Ok(SourceFile { path: path.to_path_buf(), content, language })
    }

    /// Load all supported files from a directory
    fn load_directory(&self, dir: &Path) -> Result<Vec<SourceFile>> {
        let mut files = Vec::new();

        let walker = WalkBuilder::new(dir)
            .standard_filters(self.respect_gitignore)
            .hidden(false)
            .parents(true)
            .build();

        for entry in walker {
            let entry = entry.map_err(|e| MccabreError::Io(io::Error::other(e.to_string())))?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            match self.load_file(path) {
                Ok(file) => files.push(file),
                Err(MccabreError::UnsupportedFileType(_)) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_single_file() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {}").unwrap();

        let loader = FileLoader::new();
        let files = loader.load(&file_path)?;

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].content, "fn main() {}");
        assert_eq!(files[0].language, Language::Rust);

        Ok(())
    }

    #[test]
    fn test_load_directory() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file1.rs"), "fn test1() {}").unwrap();
        fs::write(temp_dir.path().join("file2.js"), "function test2() {}").unwrap();
        fs::write(temp_dir.path().join("readme.txt"), "Not code").unwrap();

        let loader = FileLoader::new();
        let files = loader.load(temp_dir.path())?;

        assert_eq!(files.len(), 2);

        let has_rust = files.iter().any(|f| f.path.ends_with("file1.rs"));
        let has_js = files.iter().any(|f| f.path.ends_with("file2.js"));
        assert!(has_rust);
        assert!(has_js);

        Ok(())
    }

    #[test]
    fn test_gitignore_respected() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("included.rs"), "fn included() {}").unwrap();

        let ignored_dir = temp_dir.path().join("build");
        fs::create_dir(&ignored_dir).unwrap();
        fs::write(ignored_dir.join("excluded.rs"), "fn excluded() {}").unwrap();

        fs::write(temp_dir.path().join(".gitignore"), "build/\n").unwrap();

        let loader_with_gitignore = FileLoader::new().with_gitignore(true);
        let files_with = loader_with_gitignore.load(temp_dir.path())?;

        let loader_without_gitignore = FileLoader::new().with_gitignore(false);
        let files_without = loader_without_gitignore.load(temp_dir.path())?;

        assert!(files_with.iter().any(|f| f.path.ends_with("included.rs")));

        assert!(files_without.iter().any(|f| f.path.ends_with("included.rs")));
        assert!(files_without.iter().any(|f| f.path.ends_with("excluded.rs")));

        Ok(())
    }

    #[test]
    fn test_unsupported_file_type() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.xyz");
        fs::write(&file_path, "random content").unwrap();

        let loader = FileLoader::new();
        let result = loader.load(&file_path);

        assert!(matches!(result, Err(MccabreError::UnsupportedFileType(_))));
    }

    #[test]
    fn test_load_multiple() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("test1.rs");
        let file2 = temp_dir.path().join("test2.js");

        fs::write(&file1, "fn test1() {}").unwrap();
        fs::write(&file2, "function test2() {}").unwrap();

        let loader = FileLoader::new();
        let files = loader.load_multiple(&[&file1, &file2])?;

        assert_eq!(files.len(), 2);

        Ok(())
    }
}
