#!/bin/bash
set -e

echo "🔧 Building Pepakura Next for Linux..."

# Check if we're in the right directory
if [ ! -f "ui-desktop/package.json" ]; then
    echo "❌ Error: Run this script from the project root."
    exit 1
fi

# Install system dependencies
echo "📦 Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf

# Install dependencies if needed
cd ui-desktop
if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node dependencies..."
    pnpm install
fi

# Build Tauri for Linux (x86_64-unknown-linux-gnu)
echo "🏗️  Building Tauri application..."
pnpm tauri build --target x86_64-unknown-linux-gnu

echo "✅ Build complete! Check ui-desktop/src-tauri/target/release/bundle/"