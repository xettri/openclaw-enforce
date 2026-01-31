#!/usr/bin/env node

/**
 * OpenClaw Enforce - NAPI Example
 *
 * Simple example showing how to use the native module
 */

const { EnforcementEngine } = require("../index");
const path = require("path");

console.log("🔐 OpenClaw Enforce - Native Module Example\n");

// Create enforcement engine with policy
const policyPath = path.join(__dirname, "policy.toml");
const enforce = new EnforcementEngine(policyPath);

console.log("✅ Enforcement engine loaded");
console.log("📋 Policy stats:", enforce.getPolicyStats());
console.log("");

// Example 1: Read allowed file
console.log("📝 Example 1: Reading allowed file");
const result1 = enforce.readFileSync("/tmp/test-allowed/demo.txt");
if (result1.status.allowed) {
  console.log("   ✅ Access granted");
  if (result1.data) {
    console.log("   📄 Content:", result1.data.toString().substring(0, 50));
  }
} else {
  console.log("   ❌ Access denied:", result1.status.reason);
}
console.log("");

// Example 2: Try to read denied file
console.log("📝 Example 2: Trying to read /etc/passwd");
const result2 = enforce.readFileSync("/etc/passwd");
if (result2.status.allowed) {
  console.log("   ⚠️  SECURITY ISSUE: Should have been blocked!");
} else {
  console.log("   ✅ Correctly denied:", result2.status.reason);
}
console.log("");

// Example 3: Check access without reading
console.log("📝 Example 3: Checking access to .key file");
const status = enforce.canRead("/tmp/test-allowed/secret.key");
if (status.allowed) {
  console.log("   ⚠️  SECURITY ISSUE: .key files should be blocked!");
} else {
  console.log("   ✅ Correctly denied:", status.reason);
  console.log("   Violations:", status.violations.join(", "));
}
console.log("");

console.log("🎉 All examples complete!");
console.log("");
console.log("💡 Key benefits of native module:");
console.log("   • No separate daemon needed");
console.log("   • Direct function calls (much faster)");
console.log("   • Simple require() - just like any npm package");
console.log("   • Same security enforcement\n");
