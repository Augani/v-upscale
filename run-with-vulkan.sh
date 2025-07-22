#!/bin/bash

# V-Upscale Vulkan Environment Setup Script
# This script sets up the necessary environment variables for Vulkan/MoltenVK on macOS

echo "🔧 Setting up Vulkan environment for macOS..."

# Set up Vulkan environment variables for MoltenVK
export DYLD_LIBRARY_PATH="/opt/homebrew/lib:$DYLD_LIBRARY_PATH"
export VK_ICD_FILENAMES="/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json"
export VK_LAYER_PATH="/opt/homebrew/etc/vulkan/explicit_layer.d"

echo "✅ Environment variables set:"
echo "   DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH"
echo "   VK_ICD_FILENAMES=$VK_ICD_FILENAMES"
echo "   VK_LAYER_PATH=$VK_LAYER_PATH"

echo ""
echo "🚀 Starting V-Upscale in development mode..."

# Run the Tauri development server
pnpm tauri dev 