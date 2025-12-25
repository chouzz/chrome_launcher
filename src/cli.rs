use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Sets the starting URL
    #[arg(long)]
    pub starting_url: Option<String>,

    /// Sets browser flags (comma-separated)
    #[arg(long)]
    pub browser_flags: Option<String>,

    /// Sets the port for remote debugging
    #[arg(long)]
    pub port: Option<u16>,

    /// Browser type to launch
    #[arg(long, value_enum)]
    pub browser: Option<BrowserTypeArg>,

    /// Run in headless mode
    #[arg(long)]
    pub headless: bool,

    /// Window size (format: WIDTHxHEIGHT)
    #[arg(long)]
    pub window_size: Option<String>,

    /// Run in incognito mode
    #[arg(long)]
    pub incognito: bool,

    /// Disable GPU acceleration
    #[arg(long)]
    pub disable_gpu: bool,

    /// Disable sandbox (use with caution)
    #[arg(long)]
    pub no_sandbox: bool,

    /// Disable web security
    #[arg(long)]
    pub disable_web_security: bool,

    /// Allow running insecure content
    #[arg(long)]
    pub allow_insecure_content: bool,

    /// Ignore SSL/HTTPS errors
    #[arg(long)]
    pub ignore_ssl_errors: bool,

    /// Disable browser extensions
    #[arg(long)]
    pub disable_extensions: bool,

    /// Disable plugins
    #[arg(long)]
    pub disable_plugins: bool,

    /// Disable images
    #[arg(long)]
    pub disable_images: bool,

    /// Disable JavaScript
    #[arg(long)]
    pub disable_javascript: bool,

    /// Set custom user agent
    #[arg(long)]
    pub user_agent: Option<String>,

    /// Set proxy server
    #[arg(long)]
    pub proxy_server: Option<String>,

    /// Set host resolver rules
    #[arg(long)]
    pub host_resolver_rules: Option<String>,

    /// User data directory
    #[arg(long)]
    pub user_data_dir: Option<String>,

    /// Additional browser arguments (comma-separated)
    #[arg(long)]
    pub additional_args: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum BrowserTypeArg {
    Chrome,
    Chromium,
    Edge,
    Brave,
    Opera,
    Vivaldi,
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
