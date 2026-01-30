use anyhow::Result;
use path_absolutize::Absolutize;
use std::path::{Path, PathBuf};
use glob::Pattern;

use crate::policy::FilesystemPolicy;

pub struct PathValidator {
    policy: FilesystemPolicy,
}

impl PathValidator {
    pub fn new(policy: FilesystemPolicy) -> Self {
        Self { policy }
    }

    pub fn can_read(&self, path: &Path) -> Result<bool> {
        let canonical = self.canonicalize_path(path)?;
        
        // Check deny patterns first
        if self.matches_deny_pattern(&canonical) {
            return Ok(false);
        }
        
        // Check against allowed read paths
        Ok(self.matches_allowed_paths(&canonical, &self.policy.allowed_read))
    }

    /// Check if a path can be written to
    #[allow(dead_code)] // Will be used when WriteFile is implemented
    pub fn can_write(&self, path: &Path) -> Result<bool> {
        let canonical = self.canonicalize_path(path)?;
        
        // Check deny patterns first
        if self.matches_deny_pattern(&canonical) {
            return Ok(false);
        }
        
        // Check against allowed write paths
        Ok(self.matches_allowed_paths(&canonical, &self.policy.allowed_write))
    }

    fn canonicalize_path(&self, path: &Path) -> Result<PathBuf> {
        // Absolutize and normalize the path to prevent directory traversal
        let abs_path = path.absolutize()?;
        Ok(abs_path.to_path_buf())
    }

    fn matches_deny_pattern(&self, path: &PathBuf) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern_str in &self.policy.denied_patterns {
            if let Ok(pattern) = Pattern::new(pattern_str) {
                if pattern.matches(&path_str) {
                    return true;
                }
            }
        }
        
        false
    }

    fn matches_allowed_paths(&self, path: &PathBuf, allowed_paths: &[String]) -> bool {
        let path_str = path.to_string_lossy();
        
        for allowed in allowed_paths {
            // Check if path starts with an allowed prefix
            if path_str.starts_with(allowed) {
                return true;
            }
            
            // Also check glob patterns
            if let Ok(pattern) = Pattern::new(allowed) {
                if pattern.matches(&path_str) {
                    return true;
                }
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::FilesystemPolicy;

    fn create_test_policy() -> FilesystemPolicy {
        FilesystemPolicy {
            allowed_read: vec![
                "/tmp".to_string(),
                "/home/user/documents".to_string(),
            ],
            allowed_write: vec![
                "/tmp".to_string(),
            ],
            denied_patterns: vec![
                "*.key".to_string(),
                "*.pem".to_string(),
            ],
        }
    }

    #[test]
    fn test_can_read_allowed_path() {
        let validator = PathValidator::new(create_test_policy());
        let result = validator.can_read(Path::new("/tmp/test.txt"));
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_can_read_denied_pattern() {
        let validator = PathValidator::new(create_test_policy());
        let result = validator.can_read(Path::new("/tmp/secret.key"));
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_can_write_allowed_path() {
        let validator = PathValidator::new(create_test_policy());
        let result = validator.can_write(Path::new("/tmp/output.txt"));
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_can_write_denied_path() {
        let validator = PathValidator::new(create_test_policy());
       let result = validator.can_write(Path::new("/home/user/documents/file.txt"));
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
