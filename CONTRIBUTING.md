# Contributing to V-Upscale 🤝

Thank you for your interest in contributing to V-Upscale! This document provides guidelines and information for contributors.

## 🚀 Quick Start

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
3. **Set up** the development environment
4. **Create** a branch for your changes
5. **Make** your changes and test them
6. **Submit** a pull request

## 📋 Development Environment Setup

### Prerequisites

Make sure you have the following installed:

- **Rust** (latest stable) - [rustup.rs](https://rustup.rs/)
- **Node.js** (18+) - [nodejs.org](https://nodejs.org/)
- **Vulkan SDK** - [vulkan.lunarg.com](https://vulkan.lunarg.com/sdk/home)

### macOS Setup

```bash
# Install Vulkan SDK via Homebrew
brew install vulkan-headers vulkan-loader vulkan-tools

# Verify installation
vulkaninfo
```

### Project Setup

```bash
# Clone your fork
git clone https://github.com/YOUR-USERNAME/v-upscale.git
cd v-upscale

# Install dependencies
npm install

# Build Rust backend
cd src-tauri
cargo build
cd ..

# Start development server
npm run tauri:dev
```

## 🔧 Code Style & Standards

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add documentation for public APIs
- Include unit tests for new functionality

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test
```

### TypeScript/React Code

- Use TypeScript strict mode
- Follow React functional components patterns
- Use descriptive variable and function names
- Include JSDoc comments for complex functions

```bash
# Type checking
npm run build

# Linting (if configured)
npm run lint
```

### GLSL Shaders

- Use descriptive variable names
- Add comments explaining complex algorithms
- Optimize for readability first, performance second
- Test on multiple GPU architectures when possible

## 🎯 Areas for Contribution

### 🔥 High Priority

- **Cross-platform support** - Windows and Linux backends
- **Performance optimizations** - Shader improvements, memory management
- **Additional image formats** - TIFF, RAW, HDR support
- **Batch processing** - Multiple image processing
- **CLI interface** - Command-line tool for automation

### 🎨 Features

- **New interpolation algorithms** - ESRGAN, Real-ESRGAN integration
- **Advanced post-processing** - Denoising, color correction
- **GPU backend alternatives** - DirectX, Metal, WebGPU
- **Image preprocessing** - Auto-cropping, format conversion
- **Progress indicators** - Real-time processing feedback

### 🐛 Bug Fixes

- **Memory leaks** - GPU memory management
- **Cross-platform compatibility** - Path handling, file systems
- **Edge cases** - Large images, unusual formats
- **Performance regressions** - Shader optimizations
- **UI/UX improvements** - Better error messages, accessibility

### 📖 Documentation

- **API documentation** - Rust doc comments
- **Tutorial content** - How-to guides
- **Architecture docs** - System design explanations
- **Troubleshooting guides** - Common issues and solutions

## 🚀 Pull Request Process

### Before Submitting

1. **Create an issue** to discuss major changes
2. **Test your changes** thoroughly
3. **Update documentation** if needed
4. **Add tests** for new functionality
5. **Ensure all tests pass**

### PR Guidelines

- **Use descriptive titles** - Clear, concise descriptions
- **Reference issues** - Link to related issues
- **Describe changes** - What, why, and how
- **Include screenshots** - For UI changes
- **Request reviews** - Tag relevant maintainers

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Local testing completed
- [ ] All existing tests pass
- [ ] New tests added (if applicable)

## Screenshots (if applicable)
Add screenshots for UI changes

## Related Issues
Fixes #(issue number)
```

## 🧪 Testing Guidelines

### Unit Tests

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run with coverage
cargo test --features test-coverage
```

### Integration Tests

```bash
# Test full application
npm run tauri:dev
# Manually test key workflows
```

### Performance Testing

```bash
# Build optimized version
npm run tauri:build

# Test with various image sizes
# Measure processing times
# Check memory usage
```

## 🐛 Bug Reports

### Before Reporting

1. **Search existing issues** - Check if already reported
2. **Test latest version** - Ensure bug still exists
3. **Reproduce consistently** - Document exact steps
4. **Gather system info** - OS, GPU, Vulkan version

### Bug Report Template

```markdown
## Bug Description
Clear description of the issue

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., macOS 14.0]
- GPU: [e.g., M1 Pro]
- Vulkan Version: [from vulkaninfo]
- App Version: [e.g., 0.1.0]

## Screenshots/Logs
Add any relevant screenshots or error logs
```

## 💡 Feature Requests

### Before Requesting

1. **Check existing issues** - Avoid duplicates
2. **Consider scope** - Is it aligned with project goals?
3. **Provide use cases** - Why is this needed?
4. **Suggest implementation** - How might it work?

### Feature Request Template

```markdown
## Feature Description
Clear description of the proposed feature

## Use Case
Why is this feature needed?

## Proposed Solution
How might this work?

## Alternatives Considered
Other approaches you've thought about

## Additional Context
Any other relevant information
```

## 🏗️ Architecture Overview

### Project Structure

```
v-upscale/
├── src/                    # React frontend
├── src-tauri/             # Rust backend
│   ├── src/              # Rust source code
│   ├── shaders/          # GLSL compute shaders
│   └── moltenvk/         # Vulkan libraries
├── public/               # Static assets
└── dist/                 # Built frontend
```

### Key Components

- **Frontend (React)** - User interface and file handling
- **Backend (Rust)** - Image processing and Vulkan integration
- **Shaders (GLSL)** - GPU compute kernels for upscaling
- **Bundling (Tauri)** - Cross-platform app packaging

### Data Flow

1. User selects image in React UI
2. Tauri command sends path to Rust backend
3. Rust loads image and uploads to GPU
4. Vulkan compute shader processes image
5. Result downloaded from GPU and saved
6. Frontend displays upscaled image

## 🔒 Security Considerations

- **File access** - Validate all input paths
- **Memory safety** - Use Rust's safety guarantees
- **GPU resources** - Properly clean up Vulkan objects
- **Dependencies** - Keep dependencies updated

## 📞 Getting Help

- **GitHub Issues** - Bug reports and questions
- **GitHub Discussions** - Community chat and ideas
- **Discord** - Real-time community support (coming soon)

## 🎉 Recognition

Contributors will be:

- **Listed in CONTRIBUTORS.md** - Permanent recognition
- **Mentioned in release notes** - For significant contributions
- **Invited to maintainer team** - For sustained contributions

## 📜 Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct. Be respectful, inclusive, and constructive in all interactions.

---

**Thank you for contributing to V-Upscale! Together we're building the future of AI-powered image enhancement.** 🚀 