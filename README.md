# OpenClaw Enforce

**Native Node.js security enforcement module**

A lightweight, high-performance security layer written in Rust that provides OS-level sandboxing for Node.js applications.

## Features

- 🔒 **File System Control** - Whitelist/blacklist paths with pattern matching
- 🌐 **Network Security** - Domain filtering and connection limits
- ⚙️ **Process Control** - Command whitelisting and resource limits
- ⚡ **Zero Overhead** - Native Rust performance
- 🎯 **Simple API** - Just `require()` and use
- 📦 **Tiny** - Only 509KB native module
- 🔐 **Memory Safe** - Built with Rust
- ✅ **TypeScript** - Full type definitions included

## Installation

```bash
npm install openclaw-enforce
```

## Quick Start

```javascript
const { EnforcementEngine } = require("openclaw-enforce");

// Load security policy
const enforce = new EnforcementEngine("./policy.toml");

// Read file with security check
const result = enforce.readFileSync("/path/to/file.txt");

if (result.status.allowed) {
  console.log("Content:", result.data.toString());
} else {
  console.error("Access denied:", result.status.reason);
}
```

## Usage

### Basic File Reading

```javascript
const { EnforcementEngine } = require("openclaw-enforce");

const enforce = new EnforcementEngine("./policy.toml");

// Read file
const result = enforce.readFileSync("/tmp/myfile.txt");
if (result.status.allowed) {
  const content = result.data.toString();
}
```

### Check Access

```javascript
// Check without reading
const status = enforce.canRead("/etc/passwd");

console.log("Allowed:", status.allowed);
console.log("Reason:", status.reason);
console.log("Violations:", status.violations);
```

### Network Security

```javascript
// Check if network request is allowed
const netResult = enforce.canNetworkRequest("api.openai.com");

if (netResult.allowed) {
  console.log("Network request allowed");
} else {
  console.error("Blocked:", netResult.reason);
}
```

### Process Control

```javascript
// Check if command execution is allowed
const procResult = enforce.canExecuteCommand("git");

if (procResult.allowed) {
  console.log("Command allowed");
} else {
  console.error("Blocked:", procResult.reason);
}
```

### Policy Stats

```javascript
const stats = JSON.parse(enforce.getPolicyStats());
console.log("Allowed paths:", stats.filesystem.allowed_read);
console.log("Denied patterns:", stats.filesystem.denied_patterns);
```

### Complete Example

```javascript
const { EnforcementEngine } = require("openclaw-enforce");

const enforce = new EnforcementEngine("./policy.toml");

// File access
const file = enforce.readFileSync("/path/to/file.txt");
if (file.status.allowed) {
  console.log(file.data.toString());
}

// Network access
const net = enforce.canNetworkRequest("api.anthropic.com");
if (!net.allowed) {
  throw new Error(`Network denied: ${net.reason}`);
}

// Process execution
const proc = enforce.canExecuteCommand("npm");
if (!proc.allowed) {
  throw new Error(`Command denied: ${proc.reason}`);
}
```

## Standalone Functions (One-Shot API)

For quick, one-off checks without creating an engine instance:

### File Operations

```javascript
const { readFileSecure, checkReadAccess } = require("openclaw-enforce");

// Read file with security check (one-liner)
const result = readFileSecure("./policy.toml", "/path/to/file.txt");

// Just check access without reading
const status = checkReadAccess("./policy.toml", "/etc/passwd");
```

### Network Operations

```javascript
const { checkNetworkRequest } = require("openclaw-enforce");

// Check if domain is allowed
const result = checkNetworkRequest("./policy.toml", "api.openai.com");
if (!result.allowed) {
  throw new Error(`Network denied: ${result.reason}`);
}
```

### Process Operations

```javascript
const { checkCommandExecution } = require("openclaw-enforce");

// Check if command is allowed
const result = checkCommandExecution("./policy.toml", "git");
if (!result.allowed) {
  throw new Error(`Command denied: ${result.reason}`);
}
```

**When to use:**

- ✅ One-off checks in scripts
- ✅ Lambda/serverless functions
- ✅ CLI tools
- ❌ Repeated checks (use `EnforcementEngine` class instead)

## Policy Configuration

Create a `policy.toml` file:

```toml
[filesystem]
allowed_read = [
    "/tmp/openclaw",
    "/home/user/Documents",
]

allowed_write = [
    "/tmp/openclaw/output",
]

denied_patterns = [
    "*.key",
    "*.pem",
    "/etc/*",
]

[network]
allowed_domains = [
    "api.anthropic.com",
    "api.openai.com",
]

max_connections = 10

[process]
allowed_commands = ["git", "npm", "node"]
max_cpu_percent = 50
max_memory_mb = 2048
```

## API Reference

### EnforcementEngine

```typescript
class EnforcementEngine {
  constructor(policyPath: string);

  readFileSync(path: string): ReadFileResult;
  canRead(path: string): SecurityStatus;
  canWrite(path: string): SecurityStatus;
  canNetworkRequest(domain: string): NetworkCheckResult;
  canExecuteCommand(command: string): ProcessCheckResult;
  getPolicyStats(): string;
}

interface ReadFileResult {
  data: Buffer | null;
  status: SecurityStatus;
}

interface SecurityStatus {
  allowed: boolean;
  reason: string;
  violations: string[];
}

interface NetworkCheckResult {
  allowed: boolean;
  reason: string;
  violations: string[];
}

interface ProcessCheckResult {
  allowed: boolean;
  reason: string;
  violations: string[];
}
```

### Standalone Functions

```typescript
// One-off operations
readFileSecure(policyPath: string, filePath: string): ReadFileResult;
checkReadAccess(policyPath: string, filePath: string): SecurityStatus;
checkNetworkRequest(policyPath: string, domain: string): NetworkCheckResult;
checkCommandExecution(policyPath: string, command: string): ProcessCheckResult;
```

## Development

### Building from Source

```bash
# Install dependencies
npm install

# Build native module
npm run build

# Run tests
npm test
```

### Project Structure

```
openclaw-enforce/
├── src/
│   ├── lib.rs          # NAPI bindings
│   ├── fs/             # File validation
│   └── policy/         # Policy parsing
├── index.js            # Platform loader
├── index.d.ts          # TypeScript definitions
└── examples/           # Usage examples
```

## Security

This module provides:

- ✅ Rust memory safety
- ✅ Policy-driven access control
- ✅ Path canonicalization (prevents `../../../` attacks)
- ✅ Pattern-based denials

**Note:** Runs in same process as Node.js. For maximum isolation, consider the separate daemon version.

## License

MIT
