use chrome_launcher::{Launcher, Options}; 
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets the starting URL
    #[arg(long)]
    starting_url: Option<String>,

    /// Sets Chrome flags (comma-separated)
    #[arg(long)]
    chrome_flags: Option<String>,

    /// Sets the port for Chrome
    #[arg(long)]
    port: Option<u16>,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize Options and populate from parsed arguments
    let mut options = Options::default();

    if let Some(url) = args.starting_url {
        options.starting_url = Some(url);
    }

    if let Some(flags) = args.chrome_flags {
        options.chrome_flags = Some(flags.split(',').map(String::from).collect());
    }

    if let Some(port) = args.port {
        options.port = Some(port);
    }

    // Initialize the Launcher with the parsed options
    let mut launcher = Launcher::new(options);

    match launcher.launch() {
        Ok(mut launched_chrome) => {
            println!("Launched Chrome with PID: {}", launched_chrome.pid);
            let _ = launched_chrome
                .process
                .wait()
                .map_err(|e| e.to_string())
                .unwrap();
            println!("Chrome process has exited.");
        }
        Err(e) => {
            eprintln!("Error launching Chrome: {}", e);
        }
    }
}
