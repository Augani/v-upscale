#!/bin/bash

# V-Upscale Vulkan Production Build Script
# This script sets up the necessary environment variables for Vulkan/MoltenVK on macOS and builds the app

echo "🔧 Setting up Vulkan environment for macOS..."

# Set up Vulkan environment variables for MoltenVK
export DYLD_LIBRARY_PATH="/opt/homebrew/lib:$DYLD_LIBRARY_PATH"
export VK_ICD_FILENAMES="/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json"
export VK_LAYER_PATH="/opt/homebrew/etc/vulkan/explicit_layer.d"

echo "✅ Environment variables set for production build"

echo ""
echo "🏗️  Building V-Upscale for production..."

# Build the Tauri app for production
pnpm tauri build

echo ""
echo "📦 Bundling Vulkan libraries for distribution..."

# Update bundle script to work with production build
sed -i.bak 's|src-tauri/target/debug/bundle|src-tauri/target/release/bundle|g' bundle-libraries.sh

# Bundle the libraries
./bundle-libraries.sh

# Restore the original script (for debug builds)
mv bundle-libraries.sh.bak bundle-libraries.sh

echo "✅ Production build with bundled libraries complete!" 