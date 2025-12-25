#[cfg(target_os = "linux")]
use home::home_dir;
use std::env;
use std::path::Path;
use std::process::Command;

/// Represents different types of Chromium-based browsers
#[derive(Debug, Clone, PartialEq)]
pub enum BrowserType {
    Chrome,
    ChromeCanary,
    Chromium,
    Edge,
    Brave,
    Opera,
    Vivaldi,
    Custom(String),
}

impl BrowserType {
    /// Get the display name for the browser type
    pub fn name(&self) -> &str {
        match self {
            BrowserType::Chrome => "Google Chrome",
            BrowserType::ChromeCanary => "Google Chrome Canary",
            BrowserType::Chromium => "Chromium",
            BrowserType::Edge => "Microsoft Edge",
            BrowserType::Brave => "Brave",
            BrowserType::Opera => "Opera",
            BrowserType::Vivaldi => "Vivaldi",
            BrowserType::Custom(name) => name,
        }
    }

    /// Get the executable names for this browser type
    pub fn executables(&self) -> Vec<&str> {
        match self {
            BrowserType::Chrome => vec!["google-chrome-stable", "google-chrome", "chrome"],
            BrowserType::ChromeCanary => vec!["google-chrome-canary"],
            BrowserType::Chromium => vec!["chromium-browser", "chromium"],
            BrowserType::Edge => vec!["microsoft-edge-stable", "microsoft-edge", "msedge"],
            BrowserType::Brave => vec!["brave-browser-stable", "brave-browser", "brave"],
            BrowserType::Opera => vec!["opera-stable", "opera"],
            BrowserType::Vivaldi => vec!["vivaldi-stable", "vivaldi"],
            BrowserType::Custom(executable) => vec![executable],
        }
    }

    /// Get macOS application paths for this browser type
    pub fn macos_app_paths(&self) -> Vec<&str> {
        match self {
            BrowserType::Chrome => vec![
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
            ],
            BrowserType::Chromium => vec![
                "/Applications/Chromium.app/Contents/MacOS/Chromium",
            ],
            BrowserType::Edge => vec![
                "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
            ],
            BrowserType::Brave => vec![
                "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
            ],
            BrowserType::Opera => vec![
                "/Applications/Opera.app/Contents/MacOS/Opera",
            ],
            BrowserType::Vivaldi => vec![
                "/Applications/Vivaldi.app/Contents/MacOS/Vivaldi",
            ],
            _ => vec![],
        }
    }

    /// Get Windows registry paths for this browser type
    pub fn windows_registry_paths(&self) -> Vec<&str> {
        match self {
            BrowserType::Chrome => vec![
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe",
                r"SOFTWARE\Google\Chrome\BLBeacon",
            ],
            BrowserType::Edge => vec![
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\msedge.exe",
                r"SOFTWARE\Microsoft\EdgeUpdate\Clients",
            ],
            BrowserType::Brave => vec![
                r"SOFTWARE\BraveSoftware\Brave-Browser",
            ],
            BrowserType::Opera => vec![
                r"SOFTWARE\Opera Software\Opera Stable",
            ],
            BrowserType::Vivaldi => vec![
                r"SOFTWARE\Vivaldi",
            ],
            _ => vec![],
        }
    }
}

/// Represents a found browser installation
#[derive(Debug, Clone)]
pub struct Browser {
    pub browser_type: BrowserType,
    pub executable_path: String,
    pub version: Option<String>,
}

impl Browser {
    /// Create a new Browser instance
    pub fn new(browser_type: BrowserType, executable_path: String) -> Self {
        Self {
            browser_type,
            executable_path,
            version: None,
        }
    }

    /// Get the display name
    pub fn name(&self) -> &str {
        self.browser_type.name()
    }

    /// Check if the browser executable exists
    pub fn exists(&self) -> bool {
        Path::new(&self.executable_path).exists()
    }

    /// Try to get the browser version
    pub fn get_version(&mut self) -> Option<String> {
        if let Some(version) = &self.version {
            return Some(version.clone());
        }

        let version = self.query_version();
        self.version = version.clone();
        version
    }

    fn query_version(&self) -> Option<String> {
        // Try to get version using --version flag
        if let Ok(output) = Command::new(&self.executable_path)
            .arg("--version")
            .output()
        {
            if let Ok(version_str) = String::from_utf8(output.stdout) {
                // Parse version from output (usually format: "BrowserName Version.X.X.X")
                let version_line = version_str.lines().next()?;
                let parts: Vec<&str> = version_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Some(parts[1].to_string());
                }
            }
        }
        None
    }
}

/// Browser finder that can locate multiple Chromium-based browsers
pub struct BrowserFinder {
    preferred_browsers: Vec<BrowserType>,
}

impl Default for BrowserFinder {
    fn default() -> Self {
        Self {
            preferred_browsers: vec![
                BrowserType::Chrome,
                BrowserType::Chromium,
                BrowserType::Edge,
                BrowserType::Brave,
                BrowserType::Opera,
                BrowserType::Vivaldi,
            ],
        }
    }
}

impl BrowserFinder {
    #[cfg(test)]
    pub fn get_preferred_browsers(&self) -> &Vec<BrowserType> {
        &self.preferred_browsers
    }
}

impl BrowserFinder {
    /// Create a new BrowserFinder with custom preferences
    pub fn new(preferred_browsers: Vec<BrowserType>) -> Self {
        Self { preferred_browsers }
    }

    /// Find the first available browser from the preferred list
    pub fn find_first(&self) -> Option<Browser> {
        for browser_type in &self.preferred_browsers {
            if let Some(browser) = self.find_browser(browser_type) {
                return Some(browser);
            }
        }
        None
    }

    /// Find a specific browser type
    pub fn find_browser(&self, browser_type: &BrowserType) -> Option<Browser> {
        #[cfg(target_os = "macos")]
        {
            self.find_on_macos(browser_type)
        }
        #[cfg(target_os = "windows")]
        {
            self.find_on_windows(browser_type)
        }
        #[cfg(target_os = "linux")]
        {
            self.find_on_linux(browser_type)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            None
        }
    }

    /// Find all available browsers
    pub fn find_all(&self) -> Vec<Browser> {
        let mut browsers = Vec::new();

        for browser_type in &self.preferred_browsers {
            if let Some(browser) = self.find_browser(browser_type) {
                browsers.push(browser);
            }
        }

        browsers
    }

    /// Find browser on macOS
    #[cfg(target_os = "macos")]
    fn find_on_macos(&self, browser_type: &BrowserType) -> Option<Browser> {
        // Check environment variables first
        if let Some(path) = self.check_env_vars() {
            return Some(Browser::new(browser_type.clone(), path));
        }

        // Check macOS application paths
        for app_path in browser_type.macos_app_paths() {
            if Path::new(app_path).exists() {
                return Some(Browser::new(browser_type.clone(), app_path.to_string()));
            }
        }

        // Fallback to system search using lsregister
        self.find_via_lsregister(browser_type)
    }

    /// Find browser on Windows
    #[cfg(target_os = "windows")]
    fn find_on_windows(&self, browser_type: &BrowserType) -> Option<Browser> {
        // Check environment variables first
        if let Some(path) = self.check_env_vars() {
            return Some(Browser::new(browser_type.clone(), path));
        }

        // Check Windows-specific installation paths
        let install_paths = self.get_windows_install_paths(browser_type);
        for path in install_paths {
            if Path::new(&path).exists() {
                return Some(Browser::new(browser_type.clone(), path));
            }
        }

        // Check registry-based paths
        self.find_via_registry(browser_type)
    }

    /// Find browser on Linux
    #[cfg(target_os = "linux")]
    fn find_on_linux(&self, browser_type: &BrowserType) -> Option<Browser> {
        // Check environment variables first
        if let Some(path) = self.check_env_vars() {
            return Some(Browser::new(browser_type.clone(), path));
        }

        // Check which command for each executable
        for executable in browser_type.executables() {
            if let Ok(output) = Command::new("which").arg(executable).output() {
                if output.status.success() {
                    if let Ok(path) = String::from_utf8(output.stdout) {
                        let path = path.trim().to_string();
                        if !path.is_empty() && Path::new(&path).exists() {
                            return Some(Browser::new(browser_type.clone(), path));
                        }
                    }
                }
            }
        }

        // Check common Linux installation paths
        self.find_in_linux_paths(browser_type)
    }

    fn check_env_vars(&self) -> Option<String> {
        // Check common environment variables
        let env_vars = vec!["CHROME_PATH", "LIGHTHOUSE_CHROMIUM_PATH", "BROWSER_PATH"];

        for var in env_vars {
            if let Ok(path) = env::var(var) {
                if Path::new(&path).exists() {
                    if var == "LIGHTHOUSE_CHROMIUM_PATH" {
                        eprintln!("Warning: LIGHTHOUSE_CHROMIUM_PATH is deprecated, use CHROME_PATH or BROWSER_PATH instead.");
                    }
                    return Some(path);
                }
            }
        }
        None
    }

    #[cfg(target_os = "macos")]
    fn find_via_lsregister(&self, browser_type: &BrowserType) -> Option<Browser> {
        let lsregister = "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister";

        if !Path::new(lsregister).exists() {
            return None;
        }

        let search_pattern = match browser_type {
            BrowserType::Chrome => "Google Chrome",
            BrowserType::Edge => "Microsoft Edge",
            BrowserType::Brave => "Brave Browser",
            BrowserType::Opera => "Opera",
            BrowserType::Vivaldi => "Vivaldi",
            _ => return None,
        };

        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("{} -dump | grep -i '{}'", lsregister, search_pattern))
            .output()
            .ok()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains(".app") {
                let app_path = line.split(".app").next()?.trim();
                let full_app_path = format!("{}.app", app_path);

                // Construct executable path
                let exec_path = match browser_type {
                    BrowserType::Chrome => format!("{}/Contents/MacOS/Google Chrome", full_app_path),
                    BrowserType::Edge => format!("{}/Contents/MacOS/Microsoft Edge", full_app_path),
                    BrowserType::Brave => format!("{}/Contents/MacOS/Brave Browser", full_app_path),
                    BrowserType::Opera => format!("{}/Contents/MacOS/Opera", full_app_path),
                    BrowserType::Vivaldi => format!("{}/Contents/MacOS/Vivaldi", full_app_path),
                    _ => continue,
                };

                if Path::new(&exec_path).exists() {
                    return Some(Browser::new(browser_type.clone(), exec_path));
                }
            }
        }
        None
    }

    #[cfg(target_os = "windows")]
    fn get_windows_install_paths(&self, browser_type: &BrowserType) -> Vec<String> {
        let mut paths = Vec::new();

        let program_files = vec![
            env::var("PROGRAMFILES").unwrap_or_else(|_| "C:\\Program Files".to_string()),
            env::var("PROGRAMFILES(X86)").unwrap_or_else(|_| "C:\\Program Files (x86)".to_string()),
            env::var("LOCALAPPDATA").unwrap_or_else(|_| "".to_string()),
        ];

        let suffixes = match browser_type {
            BrowserType::Chrome => vec![
                "Google\\Chrome\\Application\\chrome.exe",
                "Google\\Chrome SxS\\Application\\chrome.exe",
            ],
            BrowserType::Edge => vec![
                "Microsoft\\Edge\\Application\\msedge.exe",
            ],
            BrowserType::Brave => vec![
                "BraveSoftware\\Brave-Browser\\Application\\brave.exe",
            ],
            BrowserType::Opera => vec![
                "Opera\\launcher.exe",
            ],
            BrowserType::Vivaldi => vec![
                "Vivaldi\\Application\\vivaldi.exe",
            ],
            _ => vec![],
        };

        for program_dir in program_files {
            if program_dir.is_empty() {
                continue;
            }
            for suffix in &suffixes {
                paths.push(format!("{}\\{}", program_dir, suffix));
            }
        }

        paths
    }

    #[cfg(target_os = "windows")]
    fn find_via_registry(&self, _browser_type: &BrowserType) -> Option<Browser> {
        // Note: Windows registry access would require additional dependencies
        // For now, we rely on known installation paths
        None
    }

    #[cfg(target_os = "linux")]
    fn find_in_linux_paths(&self, browser_type: &BrowserType) -> Option<Browser> {
        let common_paths = vec![
            "/usr/bin",
            "/usr/local/bin",
            "/opt/google/chrome",
            "/opt/microsoft/msedge",
            "/opt/brave.com/brave",
            "/opt/opera",
            "/opt/vivaldi",
        ];

        for base_path in common_paths {
            for executable in browser_type.executables() {
                let full_path = format!("{}/{}", base_path, executable);
                if Path::new(&full_path).exists() {
                    return Some(Browser::new(browser_type.clone(), full_path));
                }
            }
        }

        // Check .desktop files for additional paths
        self.find_via_desktop_files(browser_type)
    }

    #[cfg(target_os = "linux")]
    fn find_via_desktop_files(&self, browser_type: &BrowserType) -> Option<Browser> {
        let mut desktop_dirs = vec![
            "/usr/share/applications".to_string(),
            "/usr/local/share/applications".to_string(),
        ];

        if let Some(home) = home_dir() {
            desktop_dirs.push(home.join(".local/share/applications").to_string_lossy().into_owned());
        }

        let exec_names = browser_type.executables();

        for desktop_dir in desktop_dirs {
            if let Ok(entries) = std::fs::read_dir(desktop_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.ends_with(".desktop") {
                            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                                for line in content.lines() {
                                    if line.starts_with("Exec=") {
                                        let exec_line = line.strip_prefix("Exec=")?;
                                        // Extract the executable path (first part before spaces)
                                        let exec_path = exec_line.split_whitespace().next()?;
                                        let path = Path::new(exec_path);

                                        // Check if this matches our browser executables
                                        if let Some(file_stem) = path.file_stem() {
                                            if let Some(file_stem_str) = file_stem.to_str() {
                                                if exec_names.contains(&file_stem_str) && path.exists() {
                                                    return Some(Browser::new(browser_type.clone(), exec_path.to_string()));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

/// Legacy functions for backward compatibility
pub fn darwin_fast() -> Option<String> {
    BrowserFinder::default().find_first().map(|b| b.executable_path)
}

pub fn linux() -> Option<String> {
    BrowserFinder::default().find_first().map(|b| b.executable_path)
}

pub fn win32() -> Option<String> {
    BrowserFinder::default().find_first().map(|b| b.executable_path)
}
