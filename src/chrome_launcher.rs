use crate::browser::{Browser, BrowserFinder, BrowserType};
use crate::flags::DEFAULT_FLAGS;
use std::path::Path;
use crate::utils::get_default;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default)]
#[derive(Debug)]
pub struct Options {
    pub starting_url: Option<String>,
    pub chrome_flags: Option<Vec<String>>,
    pub prefs: Option<HashMap<String, serde_json::Value>>,
    pub port: Option<u16>,
    pub handle_sigint: Option<bool>,
    pub chrome_path: Option<String>,
    pub user_data_dir: Option<String>,
    pub log_level: Option<String>,
    pub ignore_default_flags: Option<bool>,
    pub connection_poll_interval: Option<u64>,
    pub max_connection_retries: Option<u32>,
    pub env_vars: Option<HashMap<String, String>>,
    pub browser: Option<BrowserType>,
    pub headless: Option<bool>,
    pub window_size: Option<(u32, u32)>,
    pub incognito: Option<bool>,
    pub disable_gpu: Option<bool>,
    pub no_sandbox: Option<bool>,
    pub disable_web_security: Option<bool>,
    pub allow_running_insecure_content: Option<bool>,
    pub ignore_ssl_errors: Option<bool>,
    pub disable_extensions: Option<bool>,
    pub disable_plugins: Option<bool>,
    pub disable_images: Option<bool>,
    pub disable_javascript: Option<bool>,
    pub user_agent: Option<String>,
    pub proxy_server: Option<String>,
    pub host_resolver_rules: Option<String>,
    pub additional_args: Option<Vec<String>>,
}

pub struct LaunchedChrome {
    pub pid: u32,
    pub port: u16,
    pub process: std::process::Child,
}

pub struct Launcher {
    chrome_process: Option<std::process::Child>,
    out_file: PathBuf,
    err_file: PathBuf,
    chrome_path: Option<String>,
    env_vars: HashMap<String, String>,
    port: u16,
    ignore_default_flags: bool,
    connection_poll_interval: u64,
    max_connection_retries: u32,
    user_data_dir: String,
    chrome_flags: Vec<String>,
    starting_url: String,
    browser_type: BrowserType,
    headless: bool,
    window_size: Option<(u32, u32)>,
    incognito: bool,
    disable_gpu: bool,
    no_sandbox: bool,
    disable_web_security: bool,
    allow_running_insecure_content: bool,
    ignore_ssl_errors: bool,
    disable_extensions: bool,
    disable_plugins: bool,
    disable_images: bool,
    disable_javascript: bool,
    user_agent: Option<String>,
    proxy_server: Option<String>,
    host_resolver_rules: Option<String>,
    additional_args: Vec<String>,
}


impl Launcher {
    pub fn new(opts: Options) -> Self {
        let user_data_dir = opts
            .user_data_dir
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(env::temp_dir);
        let out_file = user_data_dir.join("chrome-out.log");
        let err_file = user_data_dir.join("chrome-err.log");
        Self {
            chrome_process: None,
            out_file,
            err_file,
            chrome_path: opts.chrome_path,
            env_vars: get_default(opts.env_vars, || env::vars().collect()),
            port: get_default(opts.port, || 0),
            ignore_default_flags: get_default(opts.ignore_default_flags, || false),
            connection_poll_interval: get_default(opts.connection_poll_interval, || 500),
            max_connection_retries: get_default(opts.max_connection_retries, || 50),
            user_data_dir: get_default(opts.user_data_dir, || user_data_dir.to_string_lossy().to_owned().to_string()),
            chrome_flags: get_default(opts.chrome_flags, || [].to_vec()),
            starting_url: get_default(opts.starting_url, || "about:blank".to_owned()),
            browser_type: get_default(opts.browser, || BrowserType::Chrome),
            headless: get_default(opts.headless, || false),
            window_size: opts.window_size,
            incognito: get_default(opts.incognito, || false),
            disable_gpu: get_default(opts.disable_gpu, || false),
            no_sandbox: get_default(opts.no_sandbox, || false),
            disable_web_security: get_default(opts.disable_web_security, || false),
            allow_running_insecure_content: get_default(opts.allow_running_insecure_content, || false),
            ignore_ssl_errors: get_default(opts.ignore_ssl_errors, || false),
            disable_extensions: get_default(opts.disable_extensions, || false),
            disable_plugins: get_default(opts.disable_plugins, || false),
            disable_images: get_default(opts.disable_images, || false),
            disable_javascript: get_default(opts.disable_javascript, || false),
            user_agent: opts.user_agent,
            proxy_server: opts.proxy_server,
            host_resolver_rules: opts.host_resolver_rules,
            additional_args: get_default(opts.additional_args, || vec![]),
        }
    }

    pub fn launch(&mut self) -> Result<LaunchedChrome, String> {
        let chrome_path = if let Some(ref path) = self.chrome_path {
            path.as_str()
        } else {
            &self.get_chrome_path()?
        };
        let mut command = Command::new(chrome_path);

        command.args(self.get_flags());
        command.stdout(File::create(&self.out_file).map_err(|e| e.to_string())?);
        command.stderr(File::create(&self.err_file).map_err(|e| e.to_string())?);
        command.envs(&self.env_vars);

        let child = command.spawn().map_err(|e| e.to_string())?;
        let pid = child.id();
        self.chrome_process = Some(child);

        let process = self.chrome_process.take().unwrap();
        let port = self.port;

        Ok(LaunchedChrome { pid, port, process })
    }

    pub fn kill(&mut self) {
        if let Some(ref mut process) = self.chrome_process {
            let _ = process.kill();
        }
        self.cleanup();
    }

    // Public getter methods for testing
    #[cfg(test)]
    pub fn get_starting_url(&self) -> &str {
        &self.starting_url
    }

    #[cfg(test)]
    pub fn get_port(&self) -> u16 {
        self.port
    }

    #[cfg(test)]
    pub fn is_headless(&self) -> bool {
        self.headless
    }

    #[cfg(test)]
    pub fn get_browser_type(&self) -> &BrowserType {
        &self.browser_type
    }

    #[cfg(test)]
    pub fn get_flags_for_test(&self) -> Vec<String> {
        self.get_flags()
    }

    #[cfg(test)]
    pub fn get_all_config(&self) -> (&str, u16, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, &BrowserType, Option<&(u32, u32)>, &Vec<String>, &Vec<String>, &str) {
        (
            &self.starting_url,
            self.port,
            self.headless,
            self.incognito,
            self.disable_gpu,
            self.no_sandbox,
            self.disable_web_security,
            self.allow_running_insecure_content,
            self.ignore_ssl_errors,
            self.disable_extensions,
            self.disable_plugins,
            self.disable_images,
            &self.browser_type,
            self.window_size.as_ref(),
            &self.chrome_flags,
            &self.additional_args,
            &self.user_data_dir,
        )
    }

    #[cfg(test)]
    pub fn test_get_flags(&self) -> Vec<String> {
        self.get_flags()
    }

    #[cfg(test)]
    pub fn test_get_chrome_path(&self) -> Result<String, String> {
        self.get_chrome_path()
    }

    fn get_chrome_path(&self) -> Result<String, String> {
        // If a specific path is provided, use it
        if let Some(ref path) = self.chrome_path {
            if Path::new(path).exists() {
                return Ok(path.clone());
            } else {
                return Err(format!("Specified browser path does not exist: {}", path));
            }
        }

        // Find browser using the new BrowserFinder
        let finder = BrowserFinder::new(vec![self.browser_type.clone()]);
        match finder.find_first() {
            Some(browser) => {
                if browser.exists() {
                    Ok(browser.executable_path)
                } else {
                    Err(format!("Browser executable not found: {}", browser.executable_path))
                }
            }
            None => Err(format!("{} browser not found on this system", self.browser_type.name())),
        }
    }

    fn get_flags(&self) -> Vec<String> {
        let mut flags = if self.ignore_default_flags {
            vec![]
        } else {
            DEFAULT_FLAGS
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>()
        };

        // Add remote debugging port
        flags.push(format!("--remote-debugging-port={}", self.port));

        // Platform-specific flags
        if !self.ignore_default_flags && cfg!(target_os = "linux") {
            flags.push("--disable-setuid-sandbox".to_string());
        }

        // User data directory
        flags.push(format!("--user-data-dir={}", self.user_data_dir));

        // Headless mode
        if self.headless || env::var("HEADLESS").is_ok() {
            flags.push("--headless".to_string());
            flags.push("--disable-gpu".to_string()); // GPU is disabled in headless by default
        }

        // Window size
        if let Some((width, height)) = self.window_size {
            flags.push(format!("--window-size={},{}", width, height));
        }

        // Incognito mode
        if self.incognito {
            flags.push("--incognito".to_string());
        }

        // GPU settings
        if self.disable_gpu && !self.headless {
            flags.push("--disable-gpu".to_string());
        }

        // Sandbox settings
        if self.no_sandbox {
            flags.push("--no-sandbox".to_string());
        }

        // Security settings
        if self.disable_web_security {
            flags.push("--disable-web-security".to_string());
        }

        if self.allow_running_insecure_content {
            flags.push("--allow-running-insecure-content".to_string());
        }

        if self.ignore_ssl_errors {
            flags.push("--ignore-ssl-errors".to_string());
            flags.push("--ignore-certificate-errors".to_string());
        }

        // Extension and plugin settings
        if self.disable_extensions && !self.ignore_default_flags {
            flags.push("--disable-extensions".to_string());
        }

        if self.disable_plugins {
            flags.push("--disable-plugins".to_string());
        }

        // Content settings
        if self.disable_images {
            flags.push("--disable-images".to_string());
        }

        if self.disable_javascript {
            flags.push("--disable-javascript".to_string());
        }

        // User agent
        if let Some(ref ua) = self.user_agent {
            flags.push(format!("--user-agent={}", ua));
        }

        // Proxy settings
        if let Some(ref proxy) = self.proxy_server {
            flags.push(format!("--proxy-server={}", proxy));
        }

        // Host resolver rules
        if let Some(ref rules) = self.host_resolver_rules {
            flags.push(format!("--host-resolver-rules={}", rules));
        }

        // Additional custom flags
        flags.extend(self.chrome_flags.clone());

        // Additional args from options
        flags.extend(self.additional_args.clone());

        // Starting URL (must be last)
        flags.push(self.starting_url.clone());

        flags
    }

    fn cleanup(&self) {
        let dir = PathBuf::from(self.user_data_dir.clone());
        if dir.exists() {
            let _ = fs::remove_dir_all(dir);
        }
    }
}
