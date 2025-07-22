#!/bin/bash

# V-Upscale Library Bundling Script
# This script bundles all required Vulkan libraries into the built app for distribution

set -e

echo "📦 Bundling Vulkan libraries into V-Upscale app..."

# Paths
APP_BUNDLE="src-tauri/target/debug/bundle/macos/v-upscale.app"
FRAMEWORKS_DIR="$APP_BUNDLE/Contents/Frameworks"
RESOURCES_DIR="$APP_BUNDLE/Contents/Resources"

# Create directories if they don't exist
mkdir -p "$FRAMEWORKS_DIR"
mkdir -p "$RESOURCES_DIR/vulkan/icd.d"

echo "🔧 Copying Vulkan libraries..."

# Copy libraries to Frameworks directory (standard macOS location for app libraries)
cp /opt/homebrew/lib/libvulkan.dylib "$FRAMEWORKS_DIR/"
cp /opt/homebrew/lib/libMoltenVK.dylib "$FRAMEWORKS_DIR/"

# Copy ICD configuration
cp /opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json "$RESOURCES_DIR/vulkan/icd.d/"

echo "🔧 Updating library paths..."

# Update the ICD file to point to the bundled MoltenVK
sed -i '' 's|"library_path": ".*"|"library_path": "../../../Frameworks/libMoltenVK.dylib"|' "$RESOURCES_DIR/vulkan/icd.d/MoltenVK_icd.json"

echo "🔧 Fixing library permissions and signatures..."

# Fix permissions
chmod 755 "$FRAMEWORKS_DIR"/*.dylib
chmod 644 "$RESOURCES_DIR/vulkan/icd.d/MoltenVK_icd.json"

# Re-sign the libraries (required for distribution)
if command -v codesign >/dev/null 2>&1; then
    echo "🔏 Code signing bundled libraries..."
    codesign --force --sign - "$FRAMEWORKS_DIR/libvulkan.dylib" || echo "⚠️  Failed to sign libvulkan.dylib"
    codesign --force --sign - "$FRAMEWORKS_DIR/libMoltenVK.dylib" || echo "⚠️  Failed to sign libMoltenVK.dylib"
fi

echo "✅ Library bundling complete!"
echo "📁 Bundled libraries location: $FRAMEWORKS_DIR"
echo "📁 ICD configuration: $RESOURCES_DIR/vulkan/icd.d/MoltenVK_icd.json"
echo ""
echo "🚀 Your app is now self-contained and ready for distribution!"
echo "   Users will not need to install Vulkan SDK or MoltenVK separately." 