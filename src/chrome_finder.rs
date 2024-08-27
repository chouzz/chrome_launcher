use home::home_dir;
use std::env;
use std::path::Path;
use std::process::Command;

pub fn darwin_fast() -> Option<String> {
    let priority_options = vec![
        env::var("CHROME_PATH").ok(),
        env::var("LIGHTHOUSE_CHROMIUM_PATH").ok(),
        Some(
            "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary"
                .to_string(),
        ),
        Some("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".to_string()),
    ];

    for chrome_path in priority_options {
        if let Some(actual_path) = chrome_path {
            let path = Path::new(&actual_path);
            if path.exists() {
                return Some(actual_path);
            }
        }
    }

    darwin().get(0).cloned()
}

fn darwin() -> Vec<String> {
    let suffixes = vec![
        "/Contents/MacOS/Google Chrome Canary",
        "/Contents/MacOS/Google Chrome",
    ];

    let lsregister = "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister";
    let mut installations = vec![];

    if let Some(custom_chrome_path) = resolve_chrome_path() {
        installations.push(custom_chrome_path);
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "{} -dump | grep -i 'google chrome\\( canary\\)?\\.app'",
            lsregister
        ))
        .output()
        .expect("Failed to execute command");

    let exec_paths = String::from_utf8_lossy(&output.stdout);
    for inst in exec_paths.lines() {
        for suffix in &suffixes {
            let exec_path = format!(
                "{}{}",
                &inst[..inst.find(".app").unwrap() + 4].trim(),
                suffix
            );
            let path = Path::new(&exec_path);
            if path.exists() && !installations.contains(&exec_path) {
                installations.push(exec_path);
            }
        }
    }

    return installations;
}

fn resolve_chrome_path() -> Option<String> {
    if let Ok(chrome_path) = env::var("CHROME_PATH") {
        let path = Path::new(&chrome_path);
        if path.exists() {
            return Some(chrome_path);
        }
    }
    if let Ok(lighthouse_path) = env::var("LIGHTHOUSE_CHROMIUM_PATH") {
        eprintln!("ChromeLauncher: LIGHTHOUSE_CHROMIUM_PATH is deprecated, use CHROME_PATH env variable instead.");
        let path = Path::new(&lighthouse_path);
        if path.exists() {
            return Some(lighthouse_path);
        }
    }
    None
}

pub fn linux() -> Option<String> {
    let mut installations = vec![];

    // 1. Look into CHROME_PATH env variable
    if let Some(custom_chrome_path) = resolve_chrome_path() {
        installations.push(custom_chrome_path);
    }

    // 2. Look into the directories where .desktop are saved on gnome based distro's
    let mut desktop_installation_folders: Vec<String> = Vec::new();
    if let Some(homedir) = home_dir() {
        let path = homedir.join(".local/share/applications/");
        desktop_installation_folders.push(path.to_string_lossy().into_owned());
    }
    desktop_installation_folders.push("/usr/share/applications/".to_owned());

    installations.extend(desktop_installation_folders);

    // Look for google-chrome(-stable) & chromium(-browser) executables by using the which command
    let executables = vec![
        "google-chrome-stable",
        "google-chrome",
        "chromium-browser",
        "chromium",
    ];
    for executable in executables {
        match Command::new("which").arg(executable).output() {
            Ok(output) => {
                if let Ok(path) = String::from_utf8(output.stdout) {
                    let chrome_path = path.lines().next().unwrap_or("").to_string();
                    let path = Path::new(&chrome_path);
                    if path.exists() {
                        installations.push(chrome_path);
                    }
                }
            }
            Err(_) => {
                // ignore errors
            }
        }
    }

    installations.into_iter().next()
}

pub fn win32() -> Option<String> {
    let mut installations: Vec<String> = vec![];

    let suffixes = vec![
        "Google/Chrome SxS/Application/chrome.exe",
        "Google/Chrome/Application/chrome.exe",
    ];

    let prefixes = vec![
        env::var("LOCALAPPDATA").ok(),
        env::var("PROGRAMFILES").ok(),
        env::var("PROGRAMFILES(X86)").ok(),
    ];

    for prefix in prefixes.into_iter().filter_map(|p| p) {
        for suffix in &suffixes {
            let path = Path::new(&prefix).join(suffix);
            if path.exists() {
                installations.push(path.to_string_lossy().into_owned());
            }
        }
    }

    if let Some(custom_chrome_path) = resolve_chrome_path() {
        installations.push(custom_chrome_path);
    }

    installations.into_iter().next()
}
