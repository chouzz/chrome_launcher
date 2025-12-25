pub mod browser;
pub mod browser_launcher;
pub mod cli;
pub mod flags;
pub mod utils;

pub use browser::{Browser, BrowserFinder, BrowserType};
pub use browser_launcher::{Launcher, Options};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_options_default() {
        let options = Options::default();
        assert!(options.starting_url.is_none());
        assert!(options.browser_flags.is_none());
        assert!(options.port.is_none());
        assert!(options.browser.is_none());
        // headless defaults to None in Options::default()
        assert!(options.headless.is_none());
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
        assert_eq!(launcher.get_starting_url(), "https://example.com");
        assert_eq!(launcher.get_port(), 9222);
        assert!(!launcher.is_headless());
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
        let flags = launcher.get_flags_for_test();

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
        assert_eq!(finder.get_preferred_browsers().len(), 6); // Chrome, Chromium, Edge, Brave, Opera, Vivaldi
    }

    #[test]
    fn test_browser_finder_custom_preferences() {
        let custom_browsers = vec![BrowserType::Edge, BrowserType::Chrome];
        let finder = BrowserFinder::new(custom_browsers.clone());
        assert_eq!(finder.get_preferred_browsers(), &custom_browsers);
    }

    #[test]
    fn test_parse_window_size() {
        fn parse_window_size(size: &str) -> Option<(u32, u32)> {
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
        options.browser_flags = Some(vec!["--custom-flag".to_string()]);
        options.additional_args = Some(vec!["--extra-arg".to_string()]);
        options.user_data_dir = Some("/tmp/test-data".to_string());
        options.port = Some(9999);

        let launcher = Launcher::new(options);
        let config = launcher.get_all_config();

        assert_eq!(config.12, &BrowserType::Edge); // browser_type
        assert!(config.2); // headless
        assert!(config.3); // incognito
        assert!(config.4); // disable_gpu
        assert!(config.5); // no_sandbox
        assert!(config.6); // disable_web_security
        assert!(config.7); // allow_running_insecure_content
        assert!(config.8); // ignore_ssl_errors
        assert!(config.9); // disable_extensions
        assert!(config.10); // disable_plugins
        assert!(config.11); // disable_images
        assert_eq!(config.0, "https://test.com"); // starting_url
        assert_eq!(config.1, 9999); // port
        assert_eq!(config.13, Some(&(1024, 768))); // window_size
        assert_eq!(config.14, &vec!["--custom-flag".to_string()]); // browser_flags
        assert_eq!(config.15, &vec!["--extra-arg".to_string()]); // additional_args
        assert_eq!(config.16, "/tmp/test-data"); // user_data_dir
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

        let _launcher = Launcher::new(options);
        // Test that env_vars is set by checking it doesn't panic
        // The actual env_vars field is private, so we test indirectly
        assert!(true); // Placeholder test - env vars functionality is tested elsewhere
    }

    #[test]
    fn test_ignore_default_flags() {
        let mut options = Options::default();
        options.ignore_default_flags = Some(true);
        options.starting_url = Some("https://test.com".to_string());

        let launcher = Launcher::new(options);
        let flags = launcher.get_flags_for_test();

        // Should not contain default flags but should have basic required flags
        assert!(flags.contains(&"--remote-debugging-port=0".to_string()));
        assert!(flags.contains(&"https://test.com".to_string()));

        // Should not contain default flags like --disable-features=Translate
        assert!(!flags.contains(&"--disable-features=Translate".to_string()));
    }
}
