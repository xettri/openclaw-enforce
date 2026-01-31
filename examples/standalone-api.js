#!/usr/bin/env node

/**
 * OpenClaw Enforce - Standalone API Demo
 *
 * Demonstrates all standalone (one-shot) API functions
 */

const {
  readFileSecure,
  checkReadAccess,
  checkNetworkRequest,
  checkCommandExecution,
} = require("../index");

const path = require("path");

console.log("🔐 OpenClaw Enforce - Standalone API Demo\n");

const policyPath = path.join(__dirname, "policy.toml");

console.log("━".repeat(60));
console.log("📋 STANDALONE APIS");
console.log("━".repeat(60) + "\n");

// 1. readFileSecure - Read file in one call
console.log("1️⃣  readFileSecure(policy, path)");
try {
  const result = readFileSecure(policyPath, "/tmp/test-allowed/demo.txt");
  console.log(`   ✅ ${result.status.allowed ? "Allowed" : "Denied"}`);
  console.log(`   Reason: ${result.status.reason}`);
  if (result.data) {
    console.log(`   Size: ${result.data.length} bytes`);
  }
} catch (e) {
  console.log(`   ❌ Error: ${e.message}`);
}
console.log("");

// 2. checkReadAccess - Check without reading
console.log("2️⃣  checkReadAccess(policy, path)");
try {
  const result = checkReadAccess(policyPath, "/etc/passwd");
  console.log(`   ${result.allowed ? "⚠️  Allowed" : "✅ Denied"}`);
  console.log(`   Reason: ${result.reason}`);
  console.log(`   Violations: ${result.violations.join(", ")}`);
} catch (e) {
  console.log(`   ❌ Error: ${e.message}`);
}
console.log("");

// 3. checkNetworkRequest - Check network access
console.log("3️⃣  checkNetworkRequest(policy, domain)");
try {
  const result = checkNetworkRequest(policyPath, "api.openai.com");
  console.log(`   ${result.allowed ? "✅ Allowed" : "❌ Denied"}`);
  console.log(`   Reason: ${result.reason}`);
} catch (e) {
  console.log(`   ❌ Error: ${e.message}`);
}
console.log("");

// 4. checkCommandExecution - Check process execution
console.log("4️⃣  checkCommandExecution(policy, command)");
try {
  const result = checkCommandExecution(policyPath, "git");
  console.log(`   ${result.allowed ? "✅ Allowed" : "❌ Denied"}`);
  console.log(`   Reason: ${result.reason}`);
} catch (e) {
  console.log(`   ❌ Error: ${e.message}`);
}
console.log("");

// 5. Multiple standalone calls
console.log("5️⃣  Multiple checks (different domains)");
const domains = ["api.anthropic.com", "evil.com", "api.google.com"];
domains.forEach((domain) => {
  try {
    const result = checkNetworkRequest(policyPath, domain);
    console.log(`   ${domain}: ${result.allowed ? "✅" : "❌"}`);
  } catch (e) {
    console.log(`   ${domain}: ❌ Error`);
  }
});
console.log("");

console.log("━".repeat(60));
console.log("💡 USE CASES");
console.log("━".repeat(60) + "\n");

console.log("Standalone APIs are perfect for:");
console.log("  • One-off checks");
console.log("  • Lambda/serverless functions");
console.log("  • CLI tools");
console.log("  • Scripts\n");

console.log("Example usage:");
console.log('  const { checkReadAccess } = require("openclaw-enforce");\n');
console.log('  const result = checkReadAccess("./policy.toml", filePath);');
console.log("  if (!result.allowed) {");
console.log("    throw new Error(`Access denied: ${result.reason}`);");
console.log("  }\n");

console.log("For repeated checks, use EnforcementEngine class:");
console.log('  const { EnforcementEngine } = require("openclaw-enforce");\n');
console.log('  const enforce = new EnforcementEngine("./policy.toml");');
console.log("  const result = enforce.canRead(filePath);");
console.log("");
