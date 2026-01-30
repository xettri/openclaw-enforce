# 🚀 Quick Start Guide

Get OpenClaw Enforce running in 5 minutes!

## Step 1: Build (1 minute)

```bash
cargo build --release
```

## Step 2: Start Daemon (10 seconds)

```bash
./target/release/openclaw-enforce --config examples/policy.toml
```

You should see:

```
Starting OpenClaw Enforce gRPC server
Services registered:
  - grpc.health.v1.Health
  - openclaw.enforce.EnforcementService
gRPC server listening on 127.0.0.1:50051
```

## Step 3: Run JavaScript Example (1 minute)

In a **new terminal**:

```bash
cd examples
npm install
node example.js
```

## What You'll See

The example will:

- ✅ Connect to the daemon
- ✅ Read a file from an allowed path
- ❌ Block reading `/etc/passwd`
- ❌ Block reading `*.key` files

Example output:

```
╔════════════════════════════════════════════════════════════╗
║       OpenClaw Enforce - JavaScript Integration Example   ║
╚════════════════════════════════════════════════════════════╝

✅ Connected to daemon

📋 Example 3: Reading file in allowed path...
   ✅ File read successful!
      Content: "Hello from OpenClaw Enforce! 🔐"

📋 Example 4: Attempting to read .key file (should be denied)...
   ✅ Access correctly denied!
      Reason: Path not in allowed read list

📋 Example 5: Attempting to read /etc/passwd (should be denied)...
   ✅ Access correctly denied!
      Reason: Path not in allowed read list
```

## Next Steps

### Customize the Policy

Edit `examples/policy.toml`:

```toml
[filesystem]
allowed_read = [
    "/tmp/test-allowed",
    "/your/custom/path",  # Add your paths here
]

denied_patterns = [
    "*.key",              # Block all .key files
    "*.pem",              # Block all .pem files
]
```

### Use in Your Project

See `examples/README.md` for integration guide.

### Test Security

Try these to see enforcement in action:

```bash
# This should work
echo "test" > /tmp/test-allowed/file.txt
node -e 'require("./example").readAllowedFile()'

# This should be blocked (wrong path)
node -e 'const grpc = require("@grpc/grpc-js"); /* try to read /etc/passwd */'
```

## Troubleshooting

### Build fails

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install protobuf compiler
brew install protobuf  # macOS
apt install protobuf-compiler  # Linux
```

### Can't connect

Make sure the daemon is running in another terminal.

### Access denied

Check that your path is in `allowed_read` in the policy file.

---

**🎉 You're all set!** The security enforcement is now protecting your file access.
