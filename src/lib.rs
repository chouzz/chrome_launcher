pub mod browser;
pub mod chrome_launcher;
pub mod flags;
pub mod utils;

pub use browser::{Browser, BrowserType};
pub use chrome_launcher::{Launcher, Options};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_options_default() {
        let options = Options::default();
        assert!(options.starting_url.is_none());
        assert!(options.chrome_flags.is_none());
        assert!(options.port.is_none());
        assert!(options.browser.is_none());
        assert_eq!(options.headless, Some(false));
    }

    #[test]
    fn test_browser_type_names() {
        assert_eq!(BrowserType::Chrome.name(), "Google Chrome");
        assert_eq!(BrowserType::Edge.name(), "Microsoft Edge");
        assert_eq!(BrowserType::Brave.name(), "Brave");
        assert_eq!(BrowserType::Chromium.name(), "Chromium");
    }

    #[test]
    fn test_browser_type_executables() {
        let chrome_executables = BrowserType::Chrome.executables();
        assert!(chrome_executables.contains(&"google-chrome-stable"));
        assert!(chrome_executables.contains(&"google-chrome"));

        let edge_executables = BrowserType::Edge.executables();
        assert!(edge_executables.contains(&"microsoft-edge"));
        assert!(edge_executables.contains(&"msedge"));
    }

    #[test]
    fn test_launcher_creation() {
        let mut options = Options::default();
        options.starting_url = Some("https://example.com".to_string());
        options.browser = Some(BrowserType::Chrome);
        options.port = Some(9222);

        let launcher = Launcher::new(options);
        assert_eq!(launcher.starting_url, "https://example.com");
        assert_eq!(launcher.port, 9222);
        assert!(!launcher.headless);
    }

    #[test]
    fn test_launcher_flags_generation() {
        let mut options = Options::default();
        options.starting_url = Some("https://test.com".to_string());
        options.headless = Some(true);
        options.incognito = Some(true);
        options.disable_gpu = Some(true);
        options.window_size = Some((1920, 1080));
        options.user_agent = Some("TestAgent/1.0".to_string());

        let launcher = Launcher::new(options);
        let flags = launcher.get_flags();

        assert!(flags.contains(&"--headless".to_string()));
        assert!(flags.contains(&"--incognito".to_string()));
        assert!(flags.contains(&"--disable-gpu".to_string()));
        assert!(flags.contains(&"--window-size=1920,1080".to_string()));
        assert!(flags.contains(&"--user-agent=TestAgent/1.0".to_string()));
        assert!(flags.contains(&"https://test.com".to_string()));
    }

    #[test]
    fn test_browser_finder_creation() {
        let finder = BrowserFinder::default();
        assert_eq!(finder.preferred_browsers.len(), 6); // Chrome, Chromium, Edge, Brave, Opera, Vivaldi
    }

    #[test]
    fn test_browser_finder_custom_preferences() {
        let custom_browsers = vec![BrowserType::Edge, BrowserType::Chrome];
        let finder = BrowserFinder::new(custom_browsers.clone());
        assert_eq!(finder.preferred_browsers, custom_browsers);
    }

    #[test]
    fn test_parse_window_size() {
        use crate::cli::parse_window_size;

        assert_eq!(parse_window_size("1920x1080"), Some((1920, 1080)));
        assert_eq!(parse_window_size("800x600"), Some((800, 600)));
        assert_eq!(parse_window_size("invalid"), None);
        assert_eq!(parse_window_size("1920"), None);
        assert_eq!(parse_window_size("ax1080"), None);
    }

    #[test]
    fn test_options_with_all_features() {
        let mut options = Options::default();
        options.starting_url = Some("https://test.com".to_string());
        options.browser = Some(BrowserType::Edge);
        options.headless = Some(true);
        options.incognito = Some(true);
        options.disable_gpu = Some(true);
        options.no_sandbox = Some(true);
        options.disable_web_security = Some(true);
        options.allow_running_insecure_content = Some(true);
        options.ignore_ssl_errors = Some(true);
        options.disable_extensions = Some(true);
        options.disable_plugins = Some(true);
        options.disable_images = Some(true);
        options.disable_javascript = Some(true);
        options.user_agent = Some("CustomAgent".to_string());
        options.proxy_server = Some("http://proxy:8080".to_string());
        options.host_resolver_rules = Some("MAP *.example.com 127.0.0.1".to_string());
        options.window_size = Some((1024, 768));
        options.chrome_flags = Some(vec!["--custom-flag".to_string()]);
        options.additional_args = Some(vec!["--extra-arg".to_string()]);
        options.user_data_dir = Some("/tmp/test-data".to_string());
        options.port = Some(9999);

        let launcher = Launcher::new(options);
        assert_eq!(launcher.browser_type, BrowserType::Edge);
        assert!(launcher.headless);
        assert!(launcher.incognito);
        assert!(launcher.disable_gpu);
        assert!(launcher.no_sandbox);
        assert!(launcher.disable_web_security);
        assert!(launcher.allow_running_insecure_content);
        assert!(launcher.ignore_ssl_errors);
        assert!(launcher.disable_extensions);
        assert!(launcher.disable_plugins);
        assert!(launcher.disable_images);
        assert!(launcher.disable_javascript);
        assert_eq!(launcher.user_agent, Some("CustomAgent".to_string()));
        assert_eq!(launcher.proxy_server, Some("http://proxy:8080".to_string()));
        assert_eq!(launcher.host_resolver_rules, Some("MAP *.example.com 127.0.0.1".to_string()));
        assert_eq!(launcher.window_size, Some((1024, 768)));
        assert_eq!(launcher.chrome_flags, vec!["--custom-flag".to_string()]);
        assert_eq!(launcher.additional_args, vec!["--extra-arg".to_string()]);
        assert_eq!(launcher.user_data_dir, "/tmp/test-data");
        assert_eq!(launcher.port, 9999);
        assert_eq!(launcher.starting_url, "https://test.com");
    }

    #[test]
    fn test_browser_custom_type() {
        let custom_browser = BrowserType::Custom("/usr/bin/custom-browser".to_string());
        assert_eq!(custom_browser.name(), "/usr/bin/custom-browser");
        assert_eq!(custom_browser.executables(), &["/usr/bin/custom-browser"]);
    }

    #[test]
    fn test_launcher_with_env_vars() {
        let mut env_vars = HashMap::new();
        env_vars.insert("TEST_VAR".to_string(), "test_value".to_string());

        let mut options = Options::default();
        options.env_vars = Some(env_vars);

        let launcher = Launcher::new(options);
        assert_eq!(launcher.env_vars.get("TEST_VAR"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_ignore_default_flags() {
        let mut options = Options::default();
        options.ignore_default_flags = Some(true);
        options.starting_url = Some("https://test.com".to_string());

        let launcher = Launcher::new(options);
        let flags = launcher.get_flags();

        // Should not contain default flags but should have basic required flags
        assert!(flags.contains(&"--remote-debugging-port=0".to_string()));
        assert!(flags.contains(&"https://test.com".to_string()));

        // Should not contain default flags like --disable-features=Translate
        assert!(!flags.contains(&"--disable-features=Translate".to_string()));
    }
} 
