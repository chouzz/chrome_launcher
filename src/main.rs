mod browser;
mod chrome_launcher;
mod flags;
mod utils;
use crate::browser::BrowserType;
use crate::chrome_launcher::Launcher;
use chrome_launcher::Options;

fn main() {
    let mut options = Options::default();
    options.starting_url = Some("https://google.com".to_string());
    options.browser = Some(BrowserType::Chrome);
    options.headless = Some(false);

    let mut launcher = Launcher::new(options);
    match launcher.launch() {
        Ok(mut launched_chrome) => {
            println!("Launched browser with PID: {}", launched_chrome.pid);
            let _ = launched_chrome.process.wait().map_err(|e| e.to_string()).unwrap();
            println!("Browser process has exited.");
        }
        Err(e) => {
            eprintln!("Error launching browser: {}", e);
        }
    }
}
