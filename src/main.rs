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
