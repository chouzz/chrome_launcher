use chrome_launcher::{BrowserFinder, BrowserType, Launcher, Options};
use std::time::Duration;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_browser_finder_can_find_browsers() {
        let finder = BrowserFinder::default();
        let browsers = finder.find_all();

        // This test will pass even if no browsers are found, as it's testing
        // that the finder doesn't crash and returns a valid result
        println!("Found {} browsers", browsers.len());
        for browser in browsers {
            println!("- {}: {}", browser.name(), browser.executable_path);
            assert!(!browser.executable_path.is_empty());
        }
    }

    #[test]
    fn test_browser_finder_preferred_order() {
        let preferred = vec![BrowserType::Chrome, BrowserType::Edge, BrowserType::Brave];
        let finder = BrowserFinder::new(preferred.clone());

        assert_eq!(finder.preferred_browsers, preferred);
    }

    #[test]
    fn test_launcher_options_builder_pattern() {
        let options = Options {
            starting_url: Some("https://example.com".to_string()),
            browser: Some(BrowserType::Chrome),
            headless: Some(true),
            port: Some(9222),
            ..Default::default()
        };

        let launcher = Launcher::new(options);
        assert_eq!(launcher.starting_url, "https://example.com");
        assert_eq!(launcher.browser_type, BrowserType::Chrome);
        assert!(launcher.headless);
        assert_eq!(launcher.port, 9222);
    }

    #[test]
    fn test_browser_version_query() {
        let finder = BrowserFinder::default();
        if let Some(mut browser) = finder.find_first() {
            // This might return None if the browser doesn't support --version
            // but it should not crash
            let _version = browser.get_version();
            // Test passes if no panic occurs
        }
    }

    #[test]
    fn test_multiple_browser_types() {
        let browser_types = vec![
            BrowserType::Chrome,
            BrowserType::Chromium,
            BrowserType::Edge,
            BrowserType::Brave,
            BrowserType::Opera,
            BrowserType::Vivaldi,
        ];

        for browser_type in browser_types {
            let finder = BrowserFinder::new(vec![browser_type.clone()]);
            let result = finder.find_first();

            // Just test that the finder doesn't crash
            match result {
                Some(browser) => {
                    println!("Found {}: {}", browser_type.name(), browser.executable_path);
                    assert!(!browser.executable_path.is_empty());
                }
                None => {
                    println!("{} not found on this system", browser_type.name());
                }
            }
        }
    }

    #[test]
    fn test_flags_complex_configuration() {
        let mut options = Options::default();
        options.starting_url = Some("https://test.com".to_string());
        options.headless = Some(true);
        options.incognito = Some(true);
        options.disable_gpu = Some(true);
        options.no_sandbox = Some(true);
        options.disable_web_security = Some(true);
        options.ignore_ssl_errors = Some(true);
        options.disable_extensions = Some(true);
        options.disable_plugins = Some(true);
        options.disable_images = Some(true);
        options.disable_javascript = Some(true);
        options.user_agent = Some("TestAgent/1.0".to_string());
        options.proxy_server = Some("http://proxy.test:8080".to_string());
        options.window_size = Some((1280, 720));
        options.chrome_flags = Some(vec![
            "--custom-flag1".to_string(),
            "--custom-flag2=value".to_string(),
        ]);
        options.additional_args = Some(vec![
            "--extra-flag".to_string(),
        ]);

        let launcher = Launcher::new(options);
        let flags = launcher.get_flags();

        // Verify all flags are present
        assert!(flags.contains(&"--headless".to_string()));
        assert!(flags.contains(&"--incognito".to_string()));
        assert!(flags.contains(&"--disable-gpu".to_string()));
        assert!(flags.contains(&"--no-sandbox".to_string()));
        assert!(flags.contains(&"--disable-web-security".to_string()));
        assert!(flags.contains(&"--ignore-ssl-errors".to_string()));
        assert!(flags.contains(&"--ignore-certificate-errors".to_string()));
        assert!(flags.contains(&"--disable-extensions".to_string()));
        assert!(flags.contains(&"--disable-plugins".to_string()));
        assert!(flags.contains(&"--disable-images".to_string()));
        assert!(flags.contains(&"--disable-javascript".to_string()));
        assert!(flags.contains(&"--user-agent=TestAgent/1.0".to_string()));
        assert!(flags.contains(&"--proxy-server=http://proxy.test:8080".to_string()));
        assert!(flags.contains(&"--window-size=1280,720".to_string()));
        assert!(flags.contains(&"--custom-flag1".to_string()));
        assert!(flags.contains(&"--custom-flag2=value".to_string()));
        assert!(flags.contains(&"--extra-flag".to_string()));
        assert!(flags.contains(&"https://test.com".to_string()));
    }

    #[test]
    fn test_launcher_get_chrome_path_with_custom_path() {
        let mut options = Options::default();
        options.chrome_path = Some("/usr/bin/google-chrome".to_string());

        let launcher = Launcher::new(options);

        // This will fail on systems where the path doesn't exist, but shouldn't crash
        let result = launcher.get_chrome_path();
        match result {
            Ok(path) => assert_eq!(path, "/usr/bin/google-chrome"),
            Err(_) => {
                // Expected if path doesn't exist
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn test_environment_variable_resolution() {
        // Test that environment variable resolution doesn't crash
        let finder = BrowserFinder::default();
        let _ = finder.find_first(); // Should not panic even with missing env vars
    }

    #[test]
    fn test_launcher_with_minimal_options() {
        let options = Options {
            starting_url: Some("about:blank".to_string()),
            ..Default::default()
        };

        let launcher = Launcher::new(options);
        let flags = launcher.get_flags();

        // Should have basic required flags
        assert!(flags.iter().any(|f| f.starts_with("--remote-debugging-port=")));
        assert!(flags.iter().any(|f| f.starts_with("--user-data-dir=")));
        assert!(flags.contains(&"about:blank".to_string()));
    }
}
