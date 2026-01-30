pub mod parser;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub filesystem: FilesystemPolicy,
    pub network: NetworkPolicy,
    pub process: ProcessPolicy,
    pub resources: ResourcePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemPolicy {
    pub allowed_read: Vec<String>,
    pub allowed_write: Vec<String>,
    pub denied_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    pub allowed_domains: Vec<String>,
    pub blocked_ips: Vec<String>,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPolicy {
    pub allowed_commands: Vec<String>,
    pub max_cpu_percent: u32,
    pub max_memory_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicy {
    pub max_file_size_mb: u64,
    pub max_open_files: u32,
}

impl Policy {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        parser::load_policy(path)
    }
}
