# OpenClaw Enforce

Security enforcement layer for OpenClaw AI Assistant

---

## 🎯 Overview

OpenClaw Enforce is a **lightweight**, **high-performance** security daemon written in Rust that provides OS-level sandboxing and policy enforcement for AI assistants. It addresses critical security concerns around **prompt injection** and **unauthorized resource access** by enforcing strict, auditable policies.

### Why OpenClaw Enforce?

JavaScript/Node.js applications traditionally run with full user-level permissions:

- ❌ Unrestricted file system access
- ❌ Unrestricted network access
- ❌ Unrestricted process execution
- ❌ Vulnerable to prompt injection attacks

**OpenClaw Enforce provides:**

- ✅ Process-level isolation (Rust ↔ Node.js boundary)
- ✅ Declarative security policies (TOML)
- ✅ Defense in depth architecture
- ✅ Only 1.3 MB binary, ~8 MB RAM

## ✨ Features

### Core Security

- 🔒 **File System Isolation** - Whitelist/blacklist with glob patterns
- 🌐 **Network Security** - Domain filtering and connection limits
- ⚙️ **Process Sandboxing** - Command whitelisting and resource limits
- 🎫 **Capability Tokens** - Time-limited, revocable permissions (planned)
- 📝 **Audit Logging** - Tamper-evident security trails (planned)

### Developer Experience

- 🚀 **gRPC Interface** - High-performance IPC
- 📦 **Small Footprint** - 1.3 MB binary, 8 MB RAM
- ⚡ **Fast** - <2ms overhead per operation
- 🧪 **Interactive Testing** - Real-time policy testing CLI
- 📚 **Well Documented** - Comprehensive guides and examples

## 🚀 Quick Start

### Installation

**Prerequisites:**

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Protocol Buffers compiler

```bash
# macOS
brew install protobuf

# Linux (Debian/Ubuntu)
sudo apt-get install protobuf-compiler

# Linux (Fedora/RHEL)
sudo dnf install protobuf-compiler
```

**Build from source:**

```bash
git clone https://github.com/xettri/openclaw-enforce.git
cd openclaw-enforce
cargo build --release
```

Binary will be at: `target/release/openclaw-enforce` (1.3 MB)

### Running

```bash
# Start the daemon
./target/release/openclaw-enforce --config examples/policy.toml

# In another terminal, test it
cd examples
npm install
npm run interactive
```

See **[QUICKSTART.md](QUICKSTART.md)** for a 5-minute tutorial.

## 🎮 Interactive Demo

Try the interactive testing tool to see security enforcement in action:

```bash
cd examples
npm run interactive
```

See **[examples/INTERACTIVE.md](examples/INTERACTIVE.md)** for full guide.

## 🔧 Usage

### Command Line

```bash
# Start server with custom config
openclaw-enforce --config /etc/openclaw-enforce/policy.toml

# Validate policy without starting
openclaw-enforce --config policy.toml --validate

# Show policy summary
openclaw-enforce --config policy.toml --show-policy

# Adjust log level
openclaw-enforce --log-level debug

# Show version
openclaw-enforce --version
```

### Integration

```javascript
const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");

// Load protobuf
const proto = grpc.loadPackageDefinition(
  protoLoader.loadSync("proto/enforce.proto"),
);

// Create client
const client = new proto.openclaw.enforce.EnforcementService(
  "localhost:50051",
  grpc.credentials.createInsecure(),
);

// Read file with security check
client.ReadFile({ path: "/tmp/myfile.txt" }, (err, response) => {
  if (response.status.allowed) {
    console.log("File contents:", response.data.toString());
  } else {
    console.log("Access denied:", response.status.reason);
  }
});
```

See **[examples/README.md](examples/README.md)** for complete integration guide.

## 📊 Performance

| Metric           | Value  |
| ---------------- | ------ |
| Binary Size      | 1.3 MB |
| Memory Usage     | ~8 MB  |
| Latency Overhead | <2 ms  |

**Comparison to alternatives:**

| Solution               | Size       | RAM      | Security             |
| ---------------------- | ---------- | -------- | -------------------- |
| **OpenClaw Enforce**   | **1.3 MB** | **8 MB** | ✅ Process isolation |
| Native addon (napi-rs) | 800 KB     | 5 MB     | ❌ Same process      |
| Python + gRPC          | 50 MB      | 30 MB    | ✅ Process isolation |

## 🔒 Security

### Threat Model

OpenClaw Enforce defends against:

- ✅ Prompt injection causing malicious file access
- ✅ Accidental access to sensitive files
- ✅ Unauthorized network requests
- ✅ Resource exhaustion attacks

### Security Features

- **Fail-secure design** - Deny by default
- **Process isolation** - Separate from Node.js
- **Memory safety** - Written in Rust
- **Policy validation** - Checked on startup
- **Audit trail** - All decisions logged (planned)

### Reporting Vulnerabilities

**Do not report security issues publicly.** Email: security@openclaw.dev

See **[SECURITY.md](SECURITY.md)** for our security policy.

## 🛠️ Development

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo clippy
cargo fmt --check
```

### Development Setup

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/openclaw-enforce.git
cd openclaw-enforce
```

# Create branch

git checkout -b feature/my-feature

# Make changes, test

cargo test
cargo clippy

## 📜 License

MIT
