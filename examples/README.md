# JavaScript Examples

This directory contains JavaScript examples showing how to integrate with OpenClaw Enforce from Node.js applications.

## Quick Start

### 1. Install Dependencies

```bash
cd examples
npm install
```

### 2. Start the Daemon

In another terminal:

```bash
cd ..
cargo run --release -- --config examples/policy.toml
```

### 3. Run the Example

```bash
npm run example
```

## What the Example Demonstrates

The `example.js` script shows:

1. **Health Check** - Verifying the daemon is running
2. **Status Query** - Getting daemon version and policy info
3. **Allowed File Read** - Successfully reading from `/tmp/test-allowed/`
4. **Denied Pattern** - Blocking `*.key` files even in allowed paths
5. **Path Whitelisting** - Blocking access to `/etc/passwd`

## Example Output

```
╔════════════════════════════════════════════════════════════╗
║       OpenClaw Enforce - JavaScript Integration Example   ║
╚════════════════════════════════════════════════════════════╝

✅ Connected to daemon

📋 Example 1: Checking daemon health...
   ✅ Daemon is healthy: SERVING

📋 Example 2: Getting daemon status...
   ✅ Status retrieved:
      Version: 0.1.0
      Healthy: true
      Policy: policy.toml

📋 Example 3: Reading file in allowed path...
   ✅ File read successful!
      Content: "Hello from OpenClaw Enforce! 🔐"
      Size: 36 bytes

📋 Example 4: Attempting to read .key file (should be denied)...
   ✅ Access correctly denied!
      Reason: Path not in allowed read list
      This proves .key files are blocked by policy

📋 Example 5: Attempting to read /etc/passwd (should be denied)...
   ✅ Access correctly denied!
      Reason: Path not in allowed read list
      This proves path whitelisting works
```

## Example Files

### `example.js` (Automated Demo)

Demonstrates all features with automated tests:

- Health checks
- File read (allowed paths)
- File read (denied patterns)
- File read (outside allowed paths)

**Run:** `npm run example`

### `interactive.js` (Interactive Testing) ⭐

**NEW!** Interactive CLI tool to test paths in real-time:

```bash
npm run interactive
```

**Features:**

- Test any file path interactively
- See instant allow/deny decisions
- Run batch tests on common paths
- View current policy
- Track statistics
- Create test files
- **Perfect for experimenting and demos!**

See [INTERACTIVE.md](INTERACTIVE.md) for full guide.

## Basic Integration

### Install Dependencies

```bash
npm install @grpc/grpc-js @grpc/proto-loader
```

### Basic Integration

```javascript
const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");
const path = require("path");

// Load protobuf
const PROTO_PATH = path.join(__dirname, "path/to/enforce.proto");
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

const proto = grpc.loadPackageDefinition(packageDefinition);

// Create client
const client = new proto.openclaw.enforce.EnforcementService(
  "localhost:50051",
  grpc.credentials.createInsecure(),
);

// Read a file securely
client.ReadFile({ path: "/tmp/myfile.txt" }, (err, response) => {
  if (err) {
    console.error("Error:", err.message);
  } else if (response.status.allowed) {
    console.log("File contents:", response.data.toString());
  } else {
    console.log("Access denied:", response.status.reason);
  }
});
```

## Available Methods

See `proto/enforce.proto` for the complete API:

- `ReadFile` - Read file with security checks
- `WriteFile` - Write file with security checks (planned)
- `ExecuteCommand` - Execute command with whitelisting (planned)
- `HttpRequest` - Make HTTP request with domain filtering (planned)
- `GetStatus` - Get daemon status
- `RequestCapability` - Request time-limited permissions (planned)

## Policy Configuration

Edit `examples/policy.toml` to customize security rules:

```toml
[filesystem]
allowed_read = [
    "/tmp/test-allowed",
    "/home/user/safe-directory",
]

denied_patterns = [
    "*.key",
    "*.pem",
    "*.env",
]
```

## Troubleshooting

### "Could not connect to OpenClaw Enforce daemon"

Make sure the daemon is running:

```bash
cargo run --release -- --config examples/policy.toml
```

### "Access denied"

Check that the path is in `allowed_read` in your policy file and doesn't match any `denied_patterns`.

### Module errors

Install dependencies:

```bash
npm install
```

## Learn More

- `../ARCHITECTURE.md` - System design and components
- `../README.md` - Full project documentation
- `../proto/enforce.proto` - Complete API specification
