#!/bin/bash
set -e

echo "🔧 Building Pepakura Next for macOS..."

# Check if we're in the right directory
if [ ! -f "ui-desktop/package.json" ]; then
    echo "❌ Error: Run this script from the project root."
    exit 1
fi

# Install dependencies if needed
cd ui-desktop
if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node dependencies..."
    pnpm install
fi

# Build Tauri for macOS (aarch64-apple-darwin)
echo "🏗️  Building Tauri application..."
pnpm tauri build --target aarch64-apple-darwin

echo "✅ Build complete! Check ui-desktop/src-tauri/target/release/bundle/"