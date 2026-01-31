# Examples

## NAPI Example

Simple example showing how to use OpenClaw Enforce as a native Node.js module.

### Quick Start

```bash
# From project root
cargo build --release

# Run the example
node examples/napi-example.js
```

### What It Does

The example demonstrates:

1. **Loading the policy** - Create an `EnforcementEngine` instance
2. **Reading allowed files** - Files in `/tmp/test-allowed/` can be read
3. **Blocking denied paths** - System files like `/etc/passwd` are blocked
4. **Pattern matching** - Files like `*.key` are blocked even in allowed directories

### Expected Output

```
🔐 OpenClaw Enforce - Native Module Example

✅ Enforcement engine loaded
📋 Policy stats: {"filesystem":{"allowed_read":4,...}}

📝 Example 1: Reading allowed file
   ✅ Access granted
   📄 Content: Hello from OpenClaw Enforce! 🔐...

📝 Example 2: Trying to read /etc/passwd
   ✅ Correctly denied: Path not in allowed read list

📝 Example 3: Checking access to .key file
   ✅ Correctly denied: Path not in allowed read list
   Violations: path_not_allowed

🎉 All examples complete!
```

## Usage in Your Project

### Installation

```bash
npm install @openclaw/enforce
```

### Basic Usage

```javascript
const { EnforcementEngine } = require("@openclaw/enforce");

// Create engine with policy file
const enforce = new EnforcementEngine("./config/policy.toml");

// Read file with security check
const result = enforce.readFileSync("/path/to/file.txt");

if (result.status.allowed) {
  console.log("Content:", result.data.toString());
} else {
  console.error("Access denied:", result.status.reason);
}
```

### Check Access Without Reading

```javascript
// Just check permissions
const status = enforce.canRead("/etc/passwd");

if (!status.allowed) {
  console.log("Blocked:", status.reason);
  console.log("Violations:", status.violations);
}
```

### Get Policy Statistics

```javascript
const stats = JSON.parse(enforce.getPolicyStats());
console.log("Allowed read paths:", stats.filesystem.allowed_read);
console.log("Denied patterns:", stats.filesystem.denied_patterns);
```

## Policy Configuration

Edit `examples/policy.toml` to customize security rules:

```toml
[filesystem]
allowed_read = [
    "/tmp/test-allowed",
    "/home/user/Documents",
]

denied_patterns = [
    "*.key",
    "*.pem",
    "/etc/*",
]
```

## API Reference

See `index.d.ts` for complete TypeScript definitions.

### EnforcementEngine

```typescript
class EnforcementEngine {
  constructor(policyPath: string);
  readFileSync(path: string): ReadFileResult;
  canRead(path: string): SecurityStatus;
  canWrite(path: string): SecurityStatus;
  getPolicyStats(): string;
}
```

### Types

```typescript
interface SecurityStatus {
  allowed: boolean;
  reason: string;
  violations: string[];
}

interface ReadFileResult {
  data: Buffer | null;
  status: SecurityStatus;
}
```

## Tips

- **Test your policy** - Use `canRead()` to test paths before reading
- **Handle errors** - Always check `status.allowed` before using `data`
- **Monitor violations** - Log `status.violations` to track security issues
- **Update policy** - Restart your app after changing `policy.toml`

## Troubleshooting

### "Cannot find module '../index'"

Make sure you've built the native module:

```bash
cargo build --release
```

### "Failed to load policy"

Check that the policy file path is correct and the TOML syntax is valid:

```bash
# Validate policy
./target/release/openclaw-enforce --validate --config examples/policy.toml
```

### "Access denied" for allowed path

- Verify the path is in `allowed_read` in your policy file
- Check for typos in the path
- Ensure denied patterns aren't blocking it

---

**Ready to secure your OpenClaw application!** 🔐
