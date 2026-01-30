use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use super::Policy;

pub fn load_policy(path: &PathBuf) -> Result<Policy> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read policy file: {:?}", path))?;
    
    let policy: Policy = toml::from_str(&content)
        .with_context(|| format!("Failed to parse policy file: {:?}", path))?;
    
    validate_policy(&policy)?;
    
    Ok(policy)
}

fn validate_policy(policy: &Policy) -> Result<()> {
    // Validate filesystem paths
    if policy.filesystem.allowed_read.is_empty() 
        && policy.filesystem.allowed_write.is_empty() {
        anyhow::bail!("Policy must specify at least one allowed read or write path");
    }
    
    // Validate network settings
    if policy.network.max_connections == 0 {
        anyhow::bail!("max_connections must be greater than 0");
    }
    
    // Validate process limits
    if policy.process.max_cpu_percent > 100 {
        anyhow::bail!("max_cpu_percent cannot exceed 100");
    }
    
    if policy.process.max_memory_mb == 0 {
        anyhow::bail!("max_memory_mb must be greater than 0");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_policy() {
        let policy_content = r#"
[filesystem]
allowed_read = ["/tmp"]
allowed_write = ["/tmp"]
denied_patterns = ["*.key"]

[network]
allowed_domains = ["example.com"]
blocked_ips = []
max_connections = 10

[process]
allowed_commands = ["ls", "cat"]
max_cpu_percent = 50
max_memory_mb = 1024

[resources]
max_file_size_mb = 100
max_open_files = 1000
        "#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(policy_content.as_bytes()).unwrap();
        
        let policy = load_policy(&file.path().to_path_buf());
        assert!(policy.is_ok());
        
        let policy = policy.unwrap();
        assert_eq!(policy.filesystem.allowed_read.len(), 1);
        assert_eq!(policy.process.allowed_commands.len(), 2);
    }

    #[test]
    fn test_invalid_policy_max_connections() {
        let policy_content = r#"
[filesystem]
allowed_read = ["/tmp"]
allowed_write = ["/tmp"]
denied_patterns = []

[network]
allowed_domains = []
blocked_ips = []
max_connections = 0

[process]
allowed_commands = []
max_cpu_percent = 50
max_memory_mb = 1024

[resources]
max_file_size_mb = 100
max_open_files = 1000
        "#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(policy_content.as_bytes()).unwrap();
        
        let policy = load_policy(&file.path().to_path_buf());
        assert!(policy.is_err());
    }
}
