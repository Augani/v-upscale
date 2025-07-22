# GitHub Actions Workflows

This directory contains the CI/CD workflows for V-Upscale that automatically build, test, and release the application across multiple platforms.

## 🔄 Available Workflows

### 1. **test.yml** - Continuous Integration
**Triggers:** Push to `main`/`develop`, Pull Requests to `main`

**Jobs:**
- **Rustfmt** - Code formatting checks
- **Clippy** - Rust linting and best practices
- **Unit Tests** - Run all Rust unit tests
- **Security Audit** - Check for known vulnerabilities
- **Frontend Tests** - TypeScript compilation and tests
- **Code Coverage** - Generate coverage reports

### 2. **build.yml** - Cross-Platform Builds
**Triggers:** Push to `main`/`develop`, Tags `v*`

**Platforms:**
- macOS (Apple Silicon & Intel)
- Windows (x64)
- Linux (x64)

**Outputs:**
- `.app` bundles for macOS
- `.msi` installers for Windows  
- `.deb` packages and AppImages for Linux

### 3. **release.yml** - Automated Releases
**Triggers:** Push tags matching `v[0-9]+.[0-9]+.[0-9]+*`

**Process:**
1. Create GitHub release with auto-generated changelog
2. Build binaries for all platforms in parallel
3. Upload release assets
4. Send Discord notification (optional)
5. Update package managers (future)

## 🔧 Setup Requirements

### Required Secrets

Add these secrets to your GitHub repository settings:

```bash
# Tauri Code Signing (optional but recommended)
TAURI_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
TAURI_KEY_PASSWORD="your-key-password"

# Discord Notifications (optional)
DISCORD_WEBHOOK="https://discord.com/api/webhooks/..."
```

### Platform Dependencies

The workflows automatically install (for building only - not required by end users):
- **Vulkan SDK** - Bundled into the final application
- **Platform Tools** - Native build tools for each OS
- **Rust Toolchain** - Latest stable with required targets
- **Node.js** - For frontend building

**Note**: End users don't need any of these dependencies - everything is bundled into self-contained applications.

## 🚀 Creating a Release

### Automatic Release Process

1. **Commit your changes** to `main`
2. **Create and push a version tag:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. **GitHub Actions will automatically:**
   - Create a GitHub release
   - Build for all platforms
   - Upload release assets
   - Generate changelog from commits

### Pre-release Versions

For pre-release versions, use tags like:
- `v1.0.0-alpha.1` - Alpha release
- `v1.0.0-beta.1` - Beta release
- `v1.0.0-rc.1` - Release candidate

These will be marked as pre-releases automatically.

### Manual Release Process

If you need to create a release manually:

1. Go to **GitHub > Releases > Draft a new release**
2. **Create a tag** following the format `v1.0.0`
3. **Add release notes** describing changes
4. **Publish release** - This will trigger the build workflow

## 📊 Build Status

Add these badges to your README:

```markdown
![Test Status](https://github.com/your-username/v-upscale/workflows/Test%20&%20Quality/badge.svg)
![Build Status](https://github.com/your-username/v-upscale/workflows/Build%20V-Upscale/badge.svg)
![Release](https://github.com/your-username/v-upscale/workflows/Release/badge.svg)
```

## 🔍 Monitoring Builds

### Build Logs
- Go to **Actions** tab in your GitHub repository
- Click on any workflow run to see detailed logs
- Each job shows step-by-step progress

### Common Issues

**Vulkan SDK Installation Fails:**
```yaml
# Update Vulkan SDK version in workflows
VulkanSDK-1.3.290.0-Installer.exe  # Check for latest version
```

**Code Signing Issues:**
```bash
# Generate new signing keys
tauri signer generate --write-keys
# Add public key to tauri.conf.json
# Add private key to TAURI_PRIVATE_KEY secret
```

**Platform-Specific Build Failures:**
- Check each platform's specific dependencies
- Update package versions if needed
- Test locally on the target platform

## 🔐 Security Considerations

- **Secrets** are encrypted and only accessible during builds
- **Code signing** ensures binary authenticity
- **Dependency scanning** checks for vulnerabilities
- **Build isolation** prevents cross-contamination

## 🛠️ Customization

### Adding New Platforms

To add support for additional platforms:

1. **Add to build matrix:**
   ```yaml
   - target: aarch64-unknown-linux-gnu
     os: ubuntu-latest
     name: linux-arm64
   ```

2. **Install platform-specific dependencies**
3. **Test the build locally**
4. **Update documentation**

### Modifying Build Process

- **Frontend changes:** Modify Node.js setup and build commands
- **Backend changes:** Update Rust toolchain and cargo commands
- **Dependencies:** Add installation steps for new requirements

## 📈 Performance Optimization

- **Caching** is enabled for Rust dependencies and npm packages
- **Parallel builds** run simultaneously across platforms
- **Incremental compilation** speeds up subsequent builds
- **Artifact compression** reduces download sizes

## 🤝 Contributing to Workflows

When modifying workflows:

1. **Test locally** with [act](https://github.com/nektos/act) when possible
2. **Start with draft PRs** to test changes
3. **Document changes** in commit messages
4. **Update this README** if adding new features

---

For questions about the CI/CD setup, please open an issue or discussion on GitHub. 