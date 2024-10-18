# chrome_launcher

Launch chrome using rust.

Note: MacOS is not tested yet.

## Installation

With Cargo:
```bash
cargo install chrome_launcher
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
chrome_launcher = "0.1.0"
```

Code:

```rust
mod chrome_launcher;
mod chrome_finder;
mod flags;
mod utils;
use crate::chrome_launcher::Launcher;
use chrome_launcher::Options;

fn main() {
    let mut options = Options::default();
    options.starting_url = Some("https://google.com".to_string());

    let mut launcher = Launcher::new(options);
    match launcher.launch() {
        Ok(mut launched_chrome) => {
            println!("Launched Chrome with PID: {}", launched_chrome.pid);
            let _ = launched_chrome.process.wait().map_err(|e| e.to_string()).unwrap();
            println!("Chrome process has exited.");
        }
        Err(e) => {
            eprintln!("Error launching Chrome: {}", e);
        }
    }
}

```

## Contributing
1. Clone the repo
    ```bash
    git clone https://github.com/chouzz/chrome_launcher.git
    ```
2. Install cargo package
    ```bash
    cargo build
    ```
3. Run test
    ```bash
    cargo test
    ```


## License

chrome_launcher is released under the Apache-2.0 License. See the bundled
[LICENSE](./LICENSE) file for details.

## Credit

The idea comes from typescript version [chrome-launcher](https://github.com/GoogleChrome/chrome-launcher)
