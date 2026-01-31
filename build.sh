#!/bin/bash

# Build script for NAPI native module

echo "🔨 Building native module..."

# Build with cargo
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

# Determine platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    ARCH=$(uname -m)
    if [ "$ARCH" = "arm64" ]; then
        TARGET_NAME="openclaw-enforce.darwin-arm64.node"
        SOURCE="target/release/libopenclaw_enforce.dylib"
    else
        TARGET_NAME="openclaw-enforce.darwin-x64.node"
        SOURCE="target/release/libopenclaw_enforce.dylib"
    fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    TARGET_NAME="openclaw-enforce.linux-x64-gnu.node"
    SOURCE="target/release/libopenclaw_enforce.so"
else
    echo "❌ Unsupported platform: $OSTYPE"
    exit 1
fi

# Copy to root with correct name
echo "📦 Copying $SOURCE to $TARGET_NAME"
cp "$SOURCE" "$TARGET_NAME"

if [ $? -eq 0 ]; then
    echo "✅ Native module ready: $TARGET_NAME"
    ls -lh "$TARGET_NAME"
else
    echo "❌ Failed to copy module"
    exit 1
fi

echo ""
echo "🎉 Build complete! You can now:"
echo "   node examples/napi-example.js"
