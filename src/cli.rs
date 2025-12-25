use crate::{Launcher, Options, BrowserType};
use clap::{Parser, ValueEnum};

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

    /// Browser type to launch
    #[arg(long, value_enum)]
    browser: Option<BrowserTypeArg>,

    /// Run in headless mode
    #[arg(long)]
    headless: bool,

    /// Window size (format: WIDTHxHEIGHT)
    #[arg(long)]
    window_size: Option<String>,

    /// Run in incognito mode
    #[arg(long)]
    incognito: bool,

    /// Disable GPU acceleration
    #[arg(long)]
    disable_gpu: bool,

    /// Disable sandbox (use with caution)
    #[arg(long)]
    no_sandbox: bool,

    /// Disable web security
    #[arg(long)]
    disable_web_security: bool,

    /// Allow running insecure content
    #[arg(long)]
    allow_insecure_content: bool,

    /// Ignore SSL/HTTPS errors
    #[arg(long)]
    ignore_ssl_errors: bool,

    /// Disable browser extensions
    #[arg(long)]
    disable_extensions: bool,

    /// Disable plugins
    #[arg(long)]
    disable_plugins: bool,

    /// Disable images
    #[arg(long)]
    disable_images: bool,

    /// Disable JavaScript
    #[arg(long)]
    disable_javascript: bool,

    /// Set custom user agent
    #[arg(long)]
    user_agent: Option<String>,

    /// Set proxy server
    #[arg(long)]
    proxy_server: Option<String>,

    /// Set host resolver rules
    #[arg(long)]
    host_resolver_rules: Option<String>,

    /// User data directory
    #[arg(long)]
    user_data_dir: Option<String>,

    /// Additional Chrome arguments (comma-separated)
    #[arg(long)]
    additional_args: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum BrowserTypeArg {
    Chrome,
    Chromium,
    Edge,
    Brave,
    Opera,
    Vivaldi,
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

    // Browser type
    if let Some(browser) = args.browser {
        options.browser = Some(match browser {
            BrowserTypeArg::Chrome => BrowserType::Chrome,
            BrowserTypeArg::Chromium => BrowserType::Chromium,
            BrowserTypeArg::Edge => BrowserType::Edge,
            BrowserTypeArg::Brave => BrowserType::Brave,
            BrowserTypeArg::Opera => BrowserType::Opera,
            BrowserTypeArg::Vivaldi => BrowserType::Vivaldi,
        });
    }

    // Launch options
    options.headless = Some(args.headless);
    options.incognito = Some(args.incognito);
    options.disable_gpu = Some(args.disable_gpu);
    options.no_sandbox = Some(args.no_sandbox);
    options.disable_web_security = Some(args.disable_web_security);
    options.allow_running_insecure_content = Some(args.allow_insecure_content);
    options.ignore_ssl_errors = Some(args.ignore_ssl_errors);
    options.disable_extensions = Some(args.disable_extensions);
    options.disable_plugins = Some(args.disable_plugins);
    options.disable_images = Some(args.disable_images);
    options.disable_javascript = Some(args.disable_javascript);

    // Window size parsing
    if let Some(window_size) = args.window_size {
        if let Some((width, height)) = parse_window_size(&window_size) {
            options.window_size = Some((width, height));
        } else {
            eprintln!("Invalid window size format. Expected WIDTHxHEIGHT (e.g., 1920x1080)");
            std::process::exit(1);
        }
    }

    // Additional options
    options.user_agent = args.user_agent;
    options.proxy_server = args.proxy_server;
    options.host_resolver_rules = args.host_resolver_rules;
    options.user_data_dir = args.user_data_dir;

    if let Some(additional_args) = args.additional_args {
        options.additional_args = Some(additional_args.split(',').map(String::from).collect());
    }

    // Initialize the Launcher with the parsed options
    let mut launcher = Launcher::new(options);

    match launcher.launch() {
        Ok(mut launched_chrome) => {
            println!("Launched browser with PID: {}", launched_chrome.pid);
            let _ = launched_chrome
                .process
                .wait()
                .map_err(|e: std::io::Error| e.to_string())
                .unwrap();
            println!("Browser process has exited.");
        }
        Err(e) => {
            eprintln!("Error launching browser: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn parse_window_size(size: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = size.split('x').collect();
    if parts.len() == 2 {
        if let (Ok(width), Ok(height)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            Some((width, height))
        } else {
            None
        }
    } else {
        None
    }
}
