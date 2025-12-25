# Browser Launcher

[![Crates.io](https://img.shields.io/crates/v/browser_launcher.svg)](https://crates.io/crates/browser_launcher)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/chouzz/browser_launcher/ci.yml)](https://github.com/chouzz/browser_launcher/actions)

A powerful, cross-platform Rust library for launching browsers (Chrome, Edge, Safari, Firefox, etc.) with extensive configuration options. Supports multiple browsers on Windows, macOS, and Linux.

## Features

- 🚀 **Cross-platform support**: Windows, macOS, and Linux
- 🌐 **Multiple browsers**: Chrome, Chromium, Edge, Brave, Opera, Vivaldi, etc.
- ⚙️ **Comprehensive options**: Headless, security, networking, and more
- 🧪 **Well-tested**: Unit tests and integration tests included
- 📦 **Easy to use**: Simple API with extensive CLI support
- 🔧 **Extensible**: Modular design for easy customization

## Installation

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
browser_launcher = "0.2.0"
```

### As a CLI Tool

```bash
cargo install browser_launcher
```

## Quick Start

### Library Usage

```rust
use browser_launcher::{Launcher, Options, BrowserType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = Options::default();
    options.starting_url = Some("https://www.google.com".to_string());
    options.browser = Some(BrowserType::Chrome);
    options.headless = Some(true);

    let mut launcher = Launcher::new(options);

    match launcher.launch() {
        Ok(mut launched_browser) => {
            println!("Launched browser with PID: {}", launched_browser.pid);
            println!("Remote debugging port: {}", launched_browser.port);

            // Wait for the browser to exit
            let _ = launched_browser.process.wait()?;
            println!("Browser process has exited.");
        }
        Err(e) => {
            eprintln!("Error launching browser: {}", e);
        }
    }

    Ok(())
}
```

### CLI Usage

Launch Chrome with default settings:
```bash
browser_launcher_cli
```

Launch Edge in headless mode:
```bash
browser_launcher_cli --browser edge --headless --starting-url "https://example.com"
```

Launch with custom window size and security disabled:
```bash
browser_launcher_cli \
  --browser chrome \
  --window-size 1920x1080 \
  --disable-web-security \
  --starting-url "https://example.com"
```

## API Reference

### Browser Types

The library supports multiple browsers:

- `BrowserType::Chrome` - Google Chrome
- `BrowserType::ChromeCanary` - Google Chrome Canary
- `BrowserType::Chromium` - Chromium
- `BrowserType::Edge` - Microsoft Edge
- `BrowserType::Brave` - Brave Browser
- `BrowserType::Opera` - Opera
- `BrowserType::Vivaldi` - Vivaldi
- `BrowserType::Custom(path)` - Custom browser executable

### Options

The `Options` struct provides extensive configuration:

```rust
use browser_launcher::{Options, BrowserType};

let options = Options {
    // Browser selection
    browser: Some(BrowserType::Edge),

    // Basic settings
    starting_url: Some("https://example.com".to_string()),
    port: Some(9222),
    user_data_dir: Some("/tmp/browser-data".to_string()),

    // Launch modes
    headless: Some(true),
    incognito: Some(false),

    // Window settings
    window_size: Some((1920, 1080)),

    // Security settings
    disable_web_security: Some(false),
    no_sandbox: Some(false),
    ignore_ssl_errors: Some(false),

    // Content settings
    disable_images: Some(false),
    disable_javascript: Some(false),

    // Network settings
    proxy_server: Some("http://proxy:8080".to_string()),
    user_agent: Some("Custom User Agent".to_string()),

    // Additional flags
    browser_flags: Some(vec!["--custom-flag".to_string()]),
    additional_args: Some(vec!["--extra-arg=value".to_string()]),

    ..Default::default()
};
```

### Browser Detection

The library automatically detects installed browsers:

```rust
use browser_launcher::{BrowserFinder, BrowserType};

let finder = BrowserFinder::default();
let browsers = finder.find_all();

for browser in browsers {
    println!("Found {}: {}", browser.name(), browser.executable_path);
    if let Some(version) = browser.get_version() {
        println!("Version: {}", version);
    }
}
```

## CLI Options

The CLI tool supports all major browser launch options:

### Browser Selection
- `--browser <BROWSER>`: Browser type (chrome, chromium, edge, brave, opera, vivaldi)

### Launch Modes
- `--headless`: Run in headless mode
- `--incognito`: Run in incognito/private mode

### Window & Display
- `--window-size <WIDTHxHEIGHT>`: Set window size (e.g., 1920x1080)

### Security & Sandbox
- `--no-sandbox`: Disable sandbox (use with caution)
- `--disable-web-security`: Disable web security features
- `--ignore-ssl-errors`: Ignore SSL certificate errors
- `--allow-insecure-content`: Allow running insecure content

### Content Control
- `--disable-extensions`: Disable browser extensions
- `--disable-plugins`: Disable plugins
- `--disable-images`: Disable image loading
- `--disable-javascript`: Disable JavaScript execution

### Network & Proxy
- `--proxy-server <URL>`: Set proxy server
- `--host-resolver-rules <RULES>`: Set host resolver rules

### Advanced
- `--user-agent <STRING>`: Set custom user agent
- `--user-data-dir <PATH>`: Set user data directory
- `--port <PORT>`: Set remote debugging port
- `--starting-url <URL>`: Set initial URL to load

### Additional Arguments
- `--browser-flags <FLAGS>`: Additional browser flags (comma-separated)
- `--additional-args <ARGS>`: Additional arguments (comma-separated)

## Examples

### Headless Screenshots
```rust
use browser_launcher::{Launcher, Options, BrowserType};

let options = Options {
    browser: Some(BrowserType::Chrome),
    headless: Some(true),
    window_size: Some((1920, 1080)),
    starting_url: Some("https://example.com".to_string()),
    browser_flags: Some(vec![
        "--screenshot=output.png".to_string(),
        "--hide-scrollbars".to_string(),
    ]),
    ..Default::default()
};

let mut launcher = Launcher::new(options);
launcher.launch()?;
```

### Web Scraping Setup
```rust
use browser_launcher::{Launcher, Options, BrowserType};

let options = Options {
    browser: Some(BrowserType::Chrome),
    headless: Some(true),
    disable_images: Some(true),
    disable_extensions: Some(true),
    user_agent: Some("Mozilla/5.0 (compatible; WebScraper/1.0)".to_string()),
    starting_url: Some("https://target-site.com".to_string()),
    ..Default::default()
};

let mut launcher = Launcher::new(options);
launcher.launch()?;
```

### Development with Extensions Disabled
```bash
browser_launcher_cli \
  --browser chrome \
  --disable-extensions \
  --disable-plugins \
  --starting-url "http://localhost:3000"
```

### Testing with Security Disabled
```bash
browser_launcher_cli \
  --browser edge \
  --disable-web-security \
  --ignore-ssl-errors \
  --allow-insecure-content \
  --starting-url "https://test-site.com"
```

## Platform Support

### Windows
- Supports all Chromium-based browsers
- Registry-based and path-based detection
- Compatible with Windows 10 and later

### macOS
- Full support for Chrome, Edge, Brave, etc.
- Uses LaunchServices for browser detection
- Compatible with macOS 10.15 and later

### Linux
- Supports system-wide and user installations
- Desktop file parsing for browser detection
- Compatible with major Linux distributions

## Environment Variables

- `CHROME_PATH`: Specify custom Chrome executable path
- `BROWSER_PATH`: Specify custom browser executable path (preferred)
- `LIGHTHOUSE_CHROMIUM_PATH`: Legacy Chromium path (deprecated)
- `HEADLESS`: Set to any value to enable headless mode

## Testing

Run the test suite:

```bash
cargo test
```

Run integration tests:

```bash
cargo test --test integration_tests
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Commit your changes: `git commit -am 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

## Development

### Building

```bash
cargo build
```

### Running CLI

```bash
cargo run --bin browser_launcher_cli -- --help
```

### Code Structure

```
src/
├── browser.rs          # Browser detection and types
├── browser_launcher.rs  # Main launcher implementation
├── cli.rs             # CLI interface
├── flags.rs           # Default Chrome flags
├── lib.rs             # Library exports and tests
└── utils.rs           # Utility functions
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Credits

Inspired by the [chrome-launcher](https://github.com/GoogleChrome/chrome-launcher) TypeScript library.
