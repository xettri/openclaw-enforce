#!/usr/bin/env node

/**
 * OpenClaw Enforce - JavaScript Example
 *
 * This example shows how to integrate with OpenClaw Enforce from Node.js
 * Run: npm install && node example.js
 */
const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");
const path = require("path");
const fs = require("fs");

// Configuration
const DAEMON_ADDRESS = "localhost:50051";
const PROTO_PATH = path.join(__dirname, "../proto/enforce.proto");

// Load the protobuf
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

const proto = grpc.loadPackageDefinition(packageDefinition);

// Create gRPC client
const client = new proto.openclaw.enforce.EnforcementService(
  DAEMON_ADDRESS,
  grpc.credentials.createInsecure(),
);

// ============================================================================
// Example 1: Check daemon connection
// ============================================================================

async function checkHealth() {
  console.log("\n📋 Example 1: Checking daemon connection...");

  return new Promise((resolve, reject) => {
    // Use the enforcement service's GetStatus as a health check
    client.GetStatus({}, (err) => {
      if (err) {
        console.log("   ❌ Daemon not responding:", err.message);
        reject(err);
      } else {
        console.log("   ✅ Daemon is healthy and responding");
        resolve();
      }
    });
  });
}

// ============================================================================
// Example 2: Get daemon status
// ============================================================================

async function getStatus() {
  console.log("\n📋 Example 2: Getting daemon status...");

  return new Promise((resolve, reject) => {
    client.GetStatus({}, (err, response) => {
      if (err) {
        console.log("   ❌ Failed to get status:", err.message);
        reject(err);
      } else {
        console.log("   ✅ Status retrieved:");
        console.log(`      Version: ${response.version}`);
        console.log(`      Healthy: ${response.healthy}`);
        if (response.active_policy) {
          console.log(`      Policy: ${response.active_policy.path}`);
        }
        resolve();
      }
    });
  });
}

// ============================================================================
// Example 3: Read an allowed file
// ============================================================================

async function readAllowedFile() {
  console.log("\n📋 Example 3: Reading file in allowed path...");

  // Create a test file
  const testPath = "/tmp/test-allowed";
  const testFile = path.join(testPath, "example.txt");

  try {
    fs.mkdirSync(testPath, { recursive: true });
    fs.writeFileSync(testFile, "Hello from OpenClaw Enforce! 🔐");
    console.log(`   Created test file: ${testFile}`);
  } catch (e) {
    console.log(`   ⚠️  Could not create test file: ${e.message}`);
  }

  return new Promise((resolve, reject) => {
    client.ReadFile({ path: testFile }, (err, response) => {
      if (err) {
        console.log(`   ❌ gRPC Error: ${err.message}`);
        reject(err);
      } else if (!response.status) {
        console.log(`   ⚠️  No status in response`);
        resolve();
      } else if (response.status.allowed) {
        const content = response.data.toString();
        console.log(`   ✅ File read successful!`);
        console.log(`      Content: "${content}"`);
        console.log(`      Size: ${response.data.length} bytes`);
        resolve();
      } else {
        console.log(`   ❌ Access denied: ${response.status.reason}`);
        console.log(
          `      Violations: ${response.status.violations.join(", ")}`,
        );
        reject(new Error(response.status.reason));
      }
    });
  });
}

// ============================================================================
// Example 4: Try to read a forbidden file (pattern match)
// ============================================================================

async function readForbiddenFile() {
  console.log(
    "\n📋 Example 4: Attempting to read .key file (should be denied)...",
  );

  const forbiddenFile = "/tmp/test-allowed/secret.key";

  try {
    fs.writeFileSync(forbiddenFile, "FAKE_PRIVATE_KEY_DATA");
    console.log(`   Created test file: ${forbiddenFile}`);
  } catch (e) {
    console.log(`   ⚠️  Could not create test file: ${e.message}`);
  }

  return new Promise((resolve) => {
    client.ReadFile({ path: forbiddenFile }, (err, response) => {
      if (err) {
        console.log(`   ✅ Access correctly denied (gRPC error)`);
        console.log(`      Reason: ${err.message}`);
        resolve();
      } else if (!response.status) {
        console.log(`   ⚠️  No status in response`);
        resolve();
      } else if (!response.status.allowed) {
        console.log(`   ✅ Access correctly denied!`);
        console.log(`      Reason: ${response.status.reason}`);
        console.log(`      This proves .key files are blocked by policy`);
        resolve();
      } else {
        console.log(
          `   ❌ SECURITY ISSUE: .key file should have been blocked!`,
        );
        resolve();
      }
    });
  });
}

// ============================================================================
// Example 5: Try to read outside allowed paths
// ============================================================================

async function readOutsideAllowed() {
  console.log(
    "\n📋 Example 5: Attempting to read /etc/passwd (should be denied)...",
  );

  return new Promise((resolve) => {
    client.ReadFile({ path: "/etc/passwd" }, (err, response) => {
      if (err) {
        console.log(`   ✅ Access correctly denied (gRPC error)`);
        console.log(`      Reason: ${err.message}`);
        resolve();
      } else if (!response.status) {
        console.log(`   ⚠️  No status in response`);
        resolve();
      } else if (!response.status.allowed) {
        console.log(`   ✅ Access correctly denied!`);
        console.log(`      Reason: ${response.status.reason}`);
        console.log(`      This proves path whitelisting works`);
        resolve();
      } else {
        console.log(
          `   ❌ SECURITY ISSUE: /etc/passwd should have been blocked!`,
        );
        resolve();
      }
    });
  });
}

// ============================================================================
// Main execution
// ============================================================================

async function main() {
  console.log("╔════════════════════════════════════════════════════════════╗");
  console.log("║       OpenClaw Enforce - JavaScript Integration Example   ║");
  console.log("╚════════════════════════════════════════════════════════════╝");

  console.log(`\nConnecting to: ${DAEMON_ADDRESS}`);
  console.log("Make sure the daemon is running:");
  console.log("  cargo run --release -- --config examples/policy.toml\n");

  // Wait for connection
  await new Promise((resolve, reject) => {
    client.waitForReady(Date.now() + 5000, (err) => {
      if (err) {
        console.log("\n❌ Could not connect to OpenClaw Enforce daemon");
        console.log("\nStart the daemon first:");
        console.log("  cd ..");
        console.log("  cargo run --release -- --config examples/policy.toml\n");
        process.exit(1);
      }
      console.log("✅ Connected to daemon\n");
      resolve();
    });
  });

  try {
    // Run all examples
    await checkHealth();
    await getStatus();
    await readAllowedFile();
    await readForbiddenFile();
    await readOutsideAllowed();

    console.log(
      "\n╔════════════════════════════════════════════════════════════╗",
    );
    console.log(
      "║                   ✅ All Examples Complete                  ║",
    );
    console.log(
      "╚════════════════════════════════════════════════════════════╝",
    );

    console.log("\n📚 What you learned:");
    console.log("   1. How to connect to OpenClaw Enforce from Node.js");
    console.log("   2. How to check daemon health and status");
    console.log("   3. How to read files through the security layer");
    console.log("   4. How denied patterns (*.key) are enforced");
    console.log("   5. How path whitelisting protects sensitive files");

    console.log("\n💡 Next steps:");
    console.log("   - Modify examples/policy.toml to test different rules");
    console.log("   - Add your own file paths to allowed_read");
    console.log("   - Integrate this into your OpenClaw project");
    console.log("   - Check ARCHITECTURE.md for implementation details\n");
  } catch (error) {
    console.log("\n❌ Example failed:", error.message);
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}

// Export for use as module
module.exports = { checkHealth, readAllowedFile, client };
