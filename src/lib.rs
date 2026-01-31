use napi::bindgen_prelude::*;
use napi_derive::napi;

mod fs;
mod policy;

use crate::fs::PathValidator;
use crate::policy::Policy;

/// Security status returned from enforcement checks
#[napi(object)]
pub struct SecurityStatus {
    /// Whether the operation is allowed
    pub allowed: bool,
    /// Reason for the decision
    pub reason: String,
    /// List of policy violations
    pub violations: Vec<String>,
}

/// Result of a file read operation
#[napi(object)]
pub struct ReadFileResult {
    /// File contents (if allowed and successful)
    pub data: Option<Buffer>,
    /// Security status
    pub status: SecurityStatus,
}

/// Network request validation result
#[napi(object)]
pub struct NetworkCheckResult {
    /// Whether the request is allowed
    pub allowed: bool,
    /// Reason for the decision
    pub reason: String,
    /// Detected violations
    pub violations: Vec<String>,
}

/// Process execution validation result
#[napi(object)]
pub struct ProcessCheckResult {
    /// Whether the command is allowed
    pub allowed: bool,
    /// Reason for the decision
    pub reason: String,
    /// Detected violations
    pub violations: Vec<String>,
}

/// OpenClaw Enforce - Native security enforcement
#[napi]
pub struct EnforcementEngine {
    policy: Policy,
    validator: PathValidator,
}

#[napi]
impl EnforcementEngine {
    /// Create a new enforcement engine with a policy file
    #[napi(constructor)]
    pub fn new(policy_path: String) -> Result<Self> {
        let policy = Policy::from_file(&std::path::PathBuf::from(&policy_path))
            .map_err(|e| Error::from_reason(format!("Failed to load policy: {}", e)))?;

        let validator = PathValidator::new(policy.filesystem.clone());

        Ok(Self { policy, validator })
    }

    /// Read a file with security checks
    #[napi]
    pub fn read_file_sync(&self, path: String) -> Result<ReadFileResult> {
        let file_path = std::path::Path::new(&path);

        // Validate path
        match self.validator.can_read(file_path) {
            Ok(true) => {
                // Attempt to read file
                match std::fs::read(&path) {
                    Ok(data) => Ok(ReadFileResult {
                        data: Some(data.into()),
                        status: SecurityStatus {
                            allowed: true,
                            reason: "Access granted".to_string(),
                            violations: vec![],
                        },
                    }),
                    Err(e) => Ok(ReadFileResult {
                        data: None,
                        status: SecurityStatus {
                            allowed: false,
                            reason: format!("File read error: {}", e),
                            violations: vec!["file_read_error".to_string()],
                        },
                    }),
                }
            }
            Ok(false) => Ok(ReadFileResult {
                data: None,
                status: SecurityStatus {
                    allowed: false,
                    reason: "Path not in allowed read list".to_string(),
                    violations: vec!["path_not_allowed".to_string()],
                },
            }),
            Err(e) => Ok(ReadFileResult {
                data: None,
                status: SecurityStatus {
                    allowed: false,
                    reason: format!("Path validation failed: {}", e),
                    violations: vec!["validation_error".to_string()],
                },
            }),
        }
    }

    /// Check if a path can be read without actually reading it
    #[napi]
    pub fn can_read(&self, path: String) -> Result<SecurityStatus> {
        let file_path = std::path::Path::new(&path);

        match self.validator.can_read(file_path) {
            Ok(true) => Ok(SecurityStatus {
                allowed: true,
                reason: "Path is in allowed read list".to_string(),
                violations: vec![],
            }),
            Ok(false) => Ok(SecurityStatus {
                allowed: false,
                reason: "Path not in allowed read list".to_string(),
                violations: vec!["path_not_allowed".to_string()],
            }),
            Err(e) => Ok(SecurityStatus {
                allowed: false,
                reason: format!("Validation error: {}", e),
                violations: vec!["validation_error".to_string()],
            }),
        }
    }

    /// Check if a path can be written to
    #[napi]
    pub fn can_write(&self, path: String) -> Result<SecurityStatus> {
        let file_path = std::path::Path::new(&path);

        match self.validator.can_write(file_path) {
            Ok(true) => Ok(SecurityStatus {
                allowed: true,
                reason: "Path is in allowed write list".to_string(),
                violations: vec![],
            }),
            Ok(false) => Ok(SecurityStatus {
                allowed: false,
                reason: "Path not in allowed write list".to_string(),
                violations: vec!["path_not_allowed".to_string()],
            }),
            Err(e) => Ok(SecurityStatus {
                allowed: false,
                reason: format!("Validation error: {}", e),
                violations: vec!["validation_error".to_string()],
            }),
        }
    }

    /// Check if a network request is allowed
    #[napi]
    pub fn can_network_request(&self, domain: String) -> Result<NetworkCheckResult> {
        // Check if domain is in allowed list
        let is_allowed = self.policy.network.allowed_domains.iter()
            .any(|allowed| domain.ends_with(allowed) || allowed == &domain);

        if is_allowed {
            Ok(NetworkCheckResult {
                allowed: true,
                reason: "Domain is in allowed list".to_string(),
                violations: vec![],
            })
        } else {
            Ok(NetworkCheckResult {
                allowed: false,
                reason: format!("Domain '{}' not in allowed list", domain),
                violations: vec!["domain_not_allowed".to_string()],
            })
        }
    }

    /// Check if a command can be executed
    #[napi]
    pub fn can_execute_command(&self, command: String) -> Result<ProcessCheckResult> {
        // Check if command is in allowed list
        let is_allowed = self.policy.process.allowed_commands.iter()
            .any(|cmd| cmd == &command);

        if is_allowed {
            Ok(ProcessCheckResult {
                allowed: true,
                reason: "Command is in allowed list".to_string(),
                violations: vec![],
            })
        } else {
            Ok(ProcessCheckResult {
                allowed: false,
                reason: format!("Command '{}' not in allowed list", command),
                violations: vec!["command_not_allowed".to_string()],
            })
        }
    }

    /// Get policy statistics
    #[napi]
    pub fn get_policy_stats(&self) -> Result<String> {
        let stats = serde_json::json!({
            "filesystem": {
                "allowed_read": self.policy.filesystem.allowed_read.len(),
                "allowed_write": self.policy.filesystem.allowed_write.len(),
                "denied_patterns": self.policy.filesystem.denied_patterns.len(),
            },
            "network": {
                "allowed_domains": self.policy.network.allowed_domains.len(),
                "max_connections": self.policy.network.max_connections,
            },
            "process": {
                "allowed_commands": self.policy.process.allowed_commands.len(),
                "max_cpu_percent": self.policy.process.max_cpu_percent,
                "max_memory_mb": self.policy.process.max_memory_mb,
            }
        });

        Ok(stats.to_string())
    }
}

/// Simple API: Read file with security check
#[napi]
pub fn read_file_secure(policy_path: String, file_path: String) -> Result<ReadFileResult> {
    let engine = EnforcementEngine::new(policy_path)?;
    engine.read_file_sync(file_path)
}

/// Simple API: Check if path can be read
#[napi]
pub fn check_read_access(policy_path: String, file_path: String) -> Result<SecurityStatus> {
    let engine = EnforcementEngine::new(policy_path)?;
    engine.can_read(file_path)
}

/// Simple API: Check network request
#[napi]
pub fn check_network_request(policy_path: String, domain: String) -> Result<NetworkCheckResult> {
    let engine = EnforcementEngine::new(policy_path)?;
    engine.can_network_request(domain)
}

/// Simple API: Check command execution
#[napi]
pub fn check_command_execution(policy_path: String, command: String) -> Result<ProcessCheckResult> {
    let engine = EnforcementEngine::new(policy_path)?;
    engine.can_execute_command(command)
}
