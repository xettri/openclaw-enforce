#!/usr/bin/env node

/**
 * OpenClaw Enforce - Interactive Testing Tool
 *
 * Test file access in real-time with an interactive CLI
 * Run: node interactive.js
 */

const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");
const path = require("path");
const readline = require("readline");
const fs = require("fs");

// Configuration
const DAEMON_ADDRESS = "localhost:50051";
const PROTO_PATH = path.join(__dirname, "../proto/enforce.proto");

// Load protobuf
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

const proto = grpc.loadPackageDefinition(packageDefinition);
const client = new proto.openclaw.enforce.EnforcementService(
  DAEMON_ADDRESS,
  grpc.credentials.createInsecure(),
);

// Create readline interface
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

// Stats tracking
const stats = {
  allowed: 0,
  denied: 0,
  errors: 0,
};

// Color codes
const colors = {
  reset: "\x1b[0m",
  green: "\x1b[32m",
  red: "\x1b[31m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  cyan: "\x1b[36m",
  gray: "\x1b[90m",
};

function colorize(text, color) {
  return `${colors[color]}${text}${colors.reset}`;
}

// Test a file path
async function testFilePath(filePath) {
  return new Promise((resolve) => {
    client.ReadFile({ path: filePath }, (err, response) => {
      if (err) {
        stats.errors++;
        console.log(colorize(`   ❌ Error: ${err.message}`, "red"));
        resolve({ success: false, error: err.message });
      } else if (!response.status) {
        stats.errors++;
        console.log(colorize("   ❌ No status in response", "red"));
        resolve({ success: false, error: "No status" });
      } else if (response.status.allowed) {
        stats.allowed++;
        console.log(colorize("   ✅ Access GRANTED", "green"));
        if (response.data && response.data.length > 0) {
          const preview = response.data.toString().substring(0, 100);
          console.log(colorize(`   📄 Content preview: ${preview}...`, "gray"));
          console.log(
            colorize(`   📊 Size: ${response.data.length} bytes`, "gray"),
          );
        }
        resolve({ success: true, allowed: true });
      } else {
        stats.denied++;
        console.log(colorize("   ❌ Access DENIED", "red"));
        console.log(
          colorize(`   📋 Reason: ${response.status.reason}`, "yellow"),
        );
        if (
          response.status.violations &&
          response.status.violations.length > 0
        ) {
          console.log(
            colorize(
              `   ⚠️  Violations: ${response.status.violations.join(", ")}`,
              "yellow",
            ),
          );
        }
        resolve({ success: true, allowed: false });
      }
    });
  });
}

// Show help
function showHelp() {
  console.log("\n" + colorize("━".repeat(60), "cyan"));
  console.log(colorize("Commands:", "cyan"));
  console.log(
    "  " +
      colorize("test <path>", "blue") +
      "       - Test if a path can be read",
  );
  console.log(
    "  " +
      colorize("batch", "blue") +
      "            - Run batch tests on common paths",
  );
  console.log(
    "  " + colorize("create <path>", "blue") + "    - Create a test file",
  );
  console.log(
    "  " + colorize("policy", "blue") + "           - Show current policy",
  );
  console.log(
    "  " + colorize("stats", "blue") + "            - Show test statistics",
  );
  console.log("  " + colorize("clear", "blue") + "            - Clear screen");
  console.log(
    "  " + colorize("help", "blue") + "             - Show this help",
  );
  console.log("  " + colorize("exit", "blue") + "             - Exit the tool");
  console.log(colorize("━".repeat(60), "cyan") + "\n");
}

// Show policy
function showPolicy() {
  const policyPath = path.join(__dirname, "policy.toml");
  try {
    const policy = fs.readFileSync(policyPath, "utf8");
    console.log("\n" + colorize("Current Policy:", "cyan"));
    console.log(colorize("━".repeat(60), "cyan"));
    console.log(policy);
    console.log(colorize("━".repeat(60), "cyan") + "\n");
  } catch (e) {
    console.log(colorize(`Could not read policy: ${e.message}`, "red"));
  }
}

// Show stats
function showStats() {
  console.log("\n" + colorize("Test Statistics:", "cyan"));
  console.log(colorize("━".repeat(60), "cyan"));
  console.log(colorize(`  ✅ Allowed: ${stats.allowed}`, "green"));
  console.log(colorize(`  ❌ Denied:  ${stats.denied}`, "red"));
  console.log(colorize(`  ⚠️  Errors:  ${stats.errors}`, "yellow"));
  console.log(
    colorize(
      `  📊 Total:   ${stats.allowed + stats.denied + stats.errors}`,
      "blue",
    ),
  );
  console.log(colorize("━".repeat(60), "cyan") + "\n");
}

// Batch test common paths
async function batchTest() {
  console.log("\n" + colorize("Running batch tests...", "cyan"));
  console.log(colorize("━".repeat(60), "cyan") + "\n");

  const testPaths = [
    {
      path: "/tmp/test-allowed/demo.txt",
      description: "Allowed directory",
      shouldAllow: true,
    },
    {
      path: "/tmp/openclaw/test.txt",
      description: "Another allowed directory",
      shouldAllow: true,
    },
    {
      path: "/etc/passwd",
      description: "System file (denied by pattern)",
      shouldAllow: false,
    },
    {
      path: "/tmp/test-allowed/secret.key",
      description: "Key file (denied by pattern)",
      shouldAllow: false,
    },
    {
      path: "/home/user/Documents/file.txt",
      description: "Allowed user directory",
      shouldAllow: true,
    },
    {
      path: "/root/secret.txt",
      description: "Root directory (not in allowed list)",
      shouldAllow: false,
    },
  ];

  for (const test of testPaths) {
    console.log(colorize(`Testing: ${test.path}`, "blue"));
    console.log(colorize(`  Expected: ${test.description}`, "gray"));
    await testFilePath(test.path);
    console.log("");
  }

  console.log(colorize("Batch test complete!", "cyan"));
  showStats();
}

// Create test file
function createTestFile(filePath) {
  try {
    const dir = path.dirname(filePath);
    fs.mkdirSync(dir, { recursive: true });
    fs.writeFileSync(
      filePath,
      `Test file created at ${new Date().toISOString()}\nThis is a test file for OpenClaw Enforce.`,
    );
    console.log(colorize(`✅ Created: ${filePath}`, "green"));
  } catch (e) {
    console.log(colorize(`❌ Failed to create file: ${e.message}`, "red"));
  }
}

// Process command
async function processCommand(input) {
  const parts = input.trim().split(" ");
  const command = parts[0].toLowerCase();
  const args = parts.slice(1).join(" ");

  switch (command) {
    case "test":
      if (!args) {
        console.log(colorize("Usage: test <path>", "yellow"));
        console.log(
          colorize("Example: test /tmp/test-allowed/file.txt", "gray"),
        );
      } else {
        console.log(colorize(`\nTesting: ${args}`, "blue"));
        await testFilePath(args);
        console.log("");
      }
      break;

    case "batch":
      await batchTest();
      break;

    case "create":
      if (!args) {
        console.log(colorize("Usage: create <path>", "yellow"));
        console.log(
          colorize("Example: create /tmp/test-allowed/myfile.txt", "gray"),
        );
      } else {
        createTestFile(args);
      }
      break;

    case "policy":
      showPolicy();
      break;

    case "stats":
      showStats();
      break;

    case "clear":
      console.clear();
      showBanner();
      break;

    case "help":
      showHelp();
      break;

    case "exit":
    case "quit":
      console.log(colorize("\nGoodbye! 👋\n", "cyan"));
      rl.close();
      process.exit(0);
      break;

    case "":
      break;

    default:
      console.log(colorize(`Unknown command: ${command}`, "red"));
      console.log(colorize("Type 'help' for available commands", "yellow"));
  }
}

// Show banner
function showBanner() {
  console.log(
    colorize(
      "\n╔════════════════════════════════════════════════════════════╗",
      "cyan",
    ),
  );
  console.log(
    colorize(
      "║     OpenClaw Enforce - Interactive Testing Tool 🔐        ║",
      "cyan",
    ),
  );
  console.log(
    colorize(
      "╚════════════════════════════════════════════════════════════╝\n",
      "cyan",
    ),
  );
  console.log(colorize("Test file access policies in real-time!", "blue"));
  console.log(
    colorize(
      "Type 'help' for commands, 'batch' for quick test, 'exit' to quit\n",
      "gray",
    ),
  );
}

// Main REPL loop
async function startInteractive() {
  showBanner();

  // Wait for connection
  console.log(colorize("Connecting to daemon...", "yellow"));
  await new Promise((resolve, reject) => {
    client.waitForReady(Date.now() + 5000, (err) => {
      if (err) {
        console.log(
          colorize("\n❌ Could not connect to OpenClaw Enforce daemon", "red"),
        );
        console.log(colorize("\nMake sure the daemon is running:", "yellow"));
        console.log(colorize("  cd ..", "gray"));
        console.log(
          colorize(
            "  cargo run --release -- --config examples/policy.toml\n",
            "gray",
          ),
        );
        process.exit(1);
      }
      console.log(colorize("✅ Connected!\n", "green"));
      resolve();
    });
  });

  // Show quick tips
  console.log(colorize("💡 Quick Tips:", "cyan"));
  console.log(colorize("  • Try: test /tmp/test-allowed/demo.txt", "gray"));
  console.log(colorize("  • Try: test /etc/passwd (should be denied)", "gray"));
  console.log(colorize("  • Run 'batch' for automated tests\n", "gray"));

  // Start REPL
  const prompt = () => {
    rl.question(colorize("openclaw-enforce> ", "cyan"), async (input) => {
      await processCommand(input);
      prompt();
    });
  };

  prompt();
}

// Handle Ctrl+C
rl.on("SIGINT", () => {
  console.log(colorize("\n\nGoodbye! 👋\n", "cyan"));
  process.exit(0);
});

// Start
startInteractive().catch((err) => {
  console.error(colorize(`\n❌ Fatal error: ${err.message}\n`, "red"));
  process.exit(1);
});
