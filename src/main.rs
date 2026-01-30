use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod audit;
mod capabilities;
mod fs;
mod grpc;
mod network;
mod policy;
mod process;
mod proto;

use crate::grpc::server::serve;
use crate::policy::Policy;

#[derive(Parser, Debug)]
#[command(name = "openclaw-enforce")]
#[command(version, author = "OpenClaw Security Team")]
#[command(about = "Security enforcement layer for OpenClaw AI Assistant", long_about = None)]
struct Args {
    /// Path to policy configuration file
    #[arg(short, long, default_value = "policy.toml")]
    config: PathBuf,

    /// gRPC server address
    #[arg(short = 'a', long, default_value = "127.0.0.1:50051")]
    address: String,

    /// Validate policy and exit (don't start server)
    #[arg(long)]
    validate: bool,

    /// Show policy summary and exit
    #[arg(long)]
    show_policy: bool,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = match args.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("OpenClaw Enforce v{}", env!("CARGO_PKG_VERSION"));
    info!("Security enforcement layer for OpenClaw AI Assistant");

    // Load and validate policy
    info!("Loading policy from: {:?}", args.config);
    let policy = match Policy::from_file(&args.config) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to load policy: {}", e);
            std::process::exit(1);
        }
    };
    
    info!("✅ Policy loaded successfully");
    info!("  Allowed read paths: {}", policy.filesystem.allowed_read.len());
    info!("  Allowed write paths: {}", policy.filesystem.allowed_write.len());
    info!("  Denied patterns: {}", policy.filesystem.denied_patterns.len());
    info!("  Allowed commands: {}", policy.process.allowed_commands.len());
    info!("  Allowed domains: {}", policy.network.allowed_domains.len());

    // If validate-only mode, exit here
    if args.validate {
        println!("✅ Policy validation successful");
        println!("Configuration: {:?}", args.config);
        return Ok(());
    }

    // If show-policy mode, display and exit
    if args.show_policy {
        println!("\n=== Policy Summary ===\n");
        println!("Filesystem:");
        println!("  Allowed read: {} paths", policy.filesystem.allowed_read.len());
        println!("  Allowed write: {} paths", policy.filesystem.allowed_write.len());
        println!("  Denied patterns: {} patterns", policy.filesystem.denied_patterns.len());
        println!("\nNetwork:");
        println!("  Allowed domains: {} domains", policy.network.allowed_domains.len());
        println!("  Max connections: {}", policy.network.max_connections);
        println!("\nProcess:");
        println!("  Allowed commands: {} commands", policy.process.allowed_commands.len());
        println!("  Max CPU: {}%", policy.process.max_cpu_percent);
        println!("  Max memory: {} MB", policy.process.max_memory_mb);
        println!("\nResources:");
        println!("  Max file size: {} MB", policy.resources.max_file_size_mb);
        println!("  Max open files: {}", policy.resources.max_open_files);
        println!();
        return Ok(());
    }

    // Start gRPC server
    let addr = args.address.parse()?;
    info!("🚀 Starting gRPC server on {}", addr);
    
    serve(addr, policy).await?;

    info!("Shutting down");
    Ok(())
}
