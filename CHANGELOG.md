# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0.0] - 2025-12-25

### Added
- **Multi-browser support**: Chrome, Chromium, Edge, Brave, Opera, Vivaldi
- **Cross-platform browser detection**: Windows, macOS, Linux with automatic path resolution
- **Enhanced CLI**: 20+ command line options including `--browser`, `--headless`, `--window-size`, security controls
- **Advanced configuration API**: Comprehensive Options struct with security, networking, and content control settings
- **Environment variable support**: `CHROME_PATH`, `BROWSER_PATH` for custom browser paths
- **Comprehensive testing**: 21 test cases covering unit and integration scenarios
- **Professional documentation**: Complete README with examples and API reference

### Changed
- **Architecture**: Modular design with separate browser detection module
- **API**: Enhanced Launcher struct with extensive configuration options
- **CLI**: Expanded command-line interface with browser selection and advanced options

### Fixed
- **Compilation warnings**: Cleaned up all compiler warnings for cleaner builds
- **Type mismatches**: Fixed Linux desktop file parsing issues
- **Import issues**: Resolved CLI binary compilation problems

## [v0.1.1] - 2025-12-25

### Added

- Added browser_launcher_cli command line tools.

## [v0.1.0] - 2025-12-25

### Added

- Initial created.
