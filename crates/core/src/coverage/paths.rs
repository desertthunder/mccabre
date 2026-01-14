use std::path::{Path, PathBuf};

pub fn normalize_path(path: &str, repo_root: Option<&Path>) -> String {
    let path = PathBuf::from(path);

    let normalized = if let Some(root) = repo_root {
        if let Ok(stripped) = path.strip_prefix(root) {
            if stripped.as_os_str().is_empty() { PathBuf::from(".") } else { stripped.to_path_buf() }
        } else {
            path
        }
    } else {
        path
    };

    normalized.display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_no_root() {
        let path = "/absolute/path/to/file.rs";
        let normalized = normalize_path(path, None);
        assert_eq!(normalized, path);
    }

    #[test]
    fn test_normalize_path_with_root() {
        let path = "/repo/src/lib.rs";
        let root = Path::new("/repo");
        let normalized = normalize_path(path, Some(root));
        assert_eq!(normalized, "src/lib.rs");
    }

    #[test]
    fn test_normalize_path_no_match() {
        let path = "/other/path/file.rs";
        let root = Path::new("/repo");
        let normalized = normalize_path(path, Some(root));
        assert_eq!(normalized, path);
    }

    #[test]
    fn test_normalize_path_relative() {
        let path = "src/lib.rs";
        let normalized = normalize_path(path, None);
        assert_eq!(normalized, path);
    }

    #[test]
    fn test_normalize_path_exact_match() {
        let path = "/repo/src/lib.rs";
        let root = Path::new("/repo/src/lib.rs");
        let normalized = normalize_path(path, Some(root));
        assert_eq!(normalized, ".");
    }
}
