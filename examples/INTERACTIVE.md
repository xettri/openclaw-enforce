# Interactive Testing Tool

## Quick Start

```bash
# Make sure daemon is running in another terminal
cargo run --release -- --config examples/policy.toml

# In this directory
npm run interactive
```

## Features

### Interactive Commands

- `test <path>` - Test if a file path is allowed
- `batch` - Run automated tests on common paths
- `create <path>` - Create a test file
- `policy` - View current security policy
- `stats` - Show access statistics (allowed/denied/errors)
- `help` - Show all commands
- `exit` - Quit

### Example Session

```
openclaw-enforce> test /tmp/test-allowed/demo.txt
Testing: /tmp/test-allowed/demo.txt
   ✅ Access GRANTED
   📄 Content preview: Hello from OpenClaw Enforce!...
   📊 Size: 36 bytes

openclaw-enforce> test /etc/passwd
Testing: /etc/passwd
   ❌ Access DENIED
   📋 Reason: Path not in allowed read list
   ⚠️  Violations: path_not_allowed

openclaw-enforce> batch
Running batch tests...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Testing: /tmp/test-allowed/demo.txt
  Expected: Allowed directory
   ✅ Access GRANTED

Testing: /etc/passwd
  Expected: System file (denied by pattern)
   ❌ Access DENIED

[... more tests ...]

Batch test complete!

Test Statistics:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ✅ Allowed: 2
  ❌ Denied:  4
  ⚠️  Errors:  0
  📊 Total:   6

openclaw-enforce> stats
Test Statistics:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ✅ Allowed: 2
  ❌ Denied:  4
  ⚠️  Errors:  0
  📊 Total:   6

openclaw-enforce> exit
Goodbye! 👋
```

## Use Cases

### 1. Testing Your Policy

Try different paths to see what your policy allows:

```
openclaw-enforce> test /home/user/Documents/secret.pdf
openclaw-enforce> test /home/user/.ssh/id_rsa
openclaw-enforce> test /tmp/test-allowed/data.txt
```

### 2. Understanding Patterns

See how denied patterns work:

```
openclaw-enforce> create /tmp/test-allowed/test.key
openclaw-enforce> test /tmp/test-allowed/test.key
# Should be denied even though /tmp/test-allowed is allowed
```

### 3. Quick Demos

Show OpenClaw Enforce to others:

```
openclaw-enforce> batch
# Runs comprehensive test suite
```

### 4. Debugging

Track what's being allowed/denied:

```
openclaw-enforce> test /my/custom/path
# See exact reason for denial
openclaw-enforce> stats
# Check overall success rate
```

## Tips

- Use `Tab` for...oh wait, this is Node.js readline (no autocomplete yet)
- Press `Ctrl+C` or type `exit` to quit
- Type `help` anytime to see commands
- Run `batch` first to see example paths
- Check `policy` to see current rules

## Troubleshooting

### "Could not connect to daemon"

Start the daemon:

```bash
cd ..
cargo run --release -- --config examples/policy.toml
```

### "Access denied" for allowed path

- Check the path is in `allowed_read` in policy.toml
- Make sure you restarted the daemon after changing policy
- Run `policy` to see current rules

### Colors not showing

Your terminal might not support ANSI colors. The tool will still work, just without colors.

## Advanced Usage

### Test Multiple Paths

```bash
# Create a batch test file
cat > my-tests.txt << EOF
/tmp/test-allowed/file1.txt
/etc/passwd
/home/user/Documents/report.pdf
/tmp/test-allowed/secret.key
EOF

# Then in Node.js
fs.readFileSync('my-tests.txt', 'utf8')
  .split('\n')
  .filter(line => line.trim())
  .forEach(path => {
    console.log(`\nTesting: ${path}`);
    // ... call enforce client
  });
```

### Custom Batch Tests

Edit `interactive.js` and add your own paths to the `batchTest()` function.

## Security Note

This tool connects to the enforcement daemon using the same gRPC API that OpenClaw would use. Everything you test here represents real security checks that will apply to OpenClaw.

---

**Have fun testing!** 🔐
