# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0.0] - 2025-12-25

### 🚀 **Major Refactoring & Feature Enhancement**

This release represents a complete overhaul of the Chrome launcher, transforming it from a basic Chrome launcher into a comprehensive, enterprise-ready multi-browser launcher with extensive configuration options.

### ✨ **Added**

#### **Multi-Browser Support**
- Support for Google Chrome, Chromium, Microsoft Edge, Brave, Opera, and Vivaldi
- Automatic browser detection and version querying
- Custom browser path support
- Browser preference ordering system

#### **Cross-Platform Compatibility**
- **Windows**: Registry-based and path-based browser detection
- **macOS**: LaunchServices integration with .app bundle support
- **Linux**: Desktop file parsing and system path detection
- Environment variable support (`CHROME_PATH`, `BROWSER_PATH`, etc.)

#### **Enhanced Command Line Interface**
- 20+ command line options covering all major use cases
- Browser selection (`--browser chrome|edge|brave|opera|vivaldi`)
- Launch modes (`--headless`, `--incognito`)
- Window management (`--window-size WIDTHxHEIGHT`)
- Security controls (`--no-sandbox`, `--disable-web-security`, `--ignore-ssl-errors`)
- Content filtering (`--disable-images`, `--disable-javascript`, `--disable-extensions`)
- Network configuration (`--proxy-server`, `--user-agent`, `--host-resolver-rules`)
- Advanced options (`--chrome-flags`, `--additional-args`)

#### **Comprehensive Configuration API**
- Extensive `Options` struct with 20+ configuration fields
- Headless mode, window sizing, and positioning
- Security sandbox controls and web security disabling
- Content blocking (images, JavaScript, plugins, extensions)
- Network proxy and user agent customization
- Custom Chrome flags and additional arguments support

#### **Testing & Quality Assurance**
- **21 test cases**: 12 unit tests + 9 integration tests
- Cross-platform testing validation
- Comprehensive error handling
- Production-ready code quality

#### **Documentation & Developer Experience**
- Professional README with usage examples and API reference
- Complete CLI help documentation
- Code examples for common use cases (screenshots, web scraping, development)
- Platform-specific setup instructions

### 🔄 **Changed**

#### **Architecture Overhaul**
- Modular design with separate `browser.rs` module for browser detection
- Clean separation of concerns between launcher logic and browser finding
- Extensible architecture for future browser additions
- Backward-compatible API design

#### **API Improvements**
- Enhanced `Launcher` struct with comprehensive configuration options
- Improved error handling and user feedback
- Better type safety with enum-based browser selection
- Streamlined option builder pattern

### 🐛 **Fixed**

#### **Compilation Issues**
- Resolved type mismatch errors in Linux desktop file parsing
- Fixed import issues in CLI binary compilation
- Cleaned up all compiler warnings (7+ warnings eliminated)
- Proper conditional compilation for platform-specific code

#### **Code Quality**
- Removed dead code and unused imports
- Added proper attribute annotations for future-use fields
- Improved code documentation and comments
- Consistent error handling patterns

### 📦 **Technical Details**

#### **Dependencies**
- Updated to modern Rust 2021 edition
- Optimized dependency management
- Added test-specific dependencies (tempfile)

#### **Performance**
- Optimized browser detection algorithms
- Reduced compilation warnings for cleaner CI/CD output
- Release-mode optimizations enabled

#### **Compatibility**
- Maintained backward compatibility with existing API
- Graceful degradation for unsupported platforms
- Environment variable compatibility preserved

## [v0.1.1] - 2025-12-25

### Added

- Added chrome_launcher_cli command line tools.

## [v0.1.0] - 2025-12-25

### Added

- Initial created.
