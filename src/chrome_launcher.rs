use crate::chrome_finder::{darwin_fast, linux, win32};
use crate::flags::DEFAULT_FLAGS;
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
    connection_poll_interval:u64,
    max_connection_retries:u32,
    user_data_dir: String,
    chrome_flags: Vec<String>,
    starting_url: String,
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
            starting_url: get_default(opts.starting_url, || "about:blank".to_owned())
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

    fn get_chrome_path(&self) -> Result<String, String> {
        let chrome_path = if cfg!(target_os = "windows") {
            win32()
        } else if cfg!(target_os = "linux") {
            linux()
        } else if cfg!(target_os = "macos") {
            darwin_fast()
        } else {
            return Err("Unknown operating system".to_string());
        };

        chrome_path.ok_or_else(|| "Chrome path not found".to_string())
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

      
            flags.push(format!("--remote-debugging-port={}", self.port));
        

        if !self.ignore_default_flags && cfg!(target_os = "linux") {
            flags.push("--disable-setuid-sandbox".to_string());
        }
        flags.push(format!("--user-data-dir={}", self.user_data_dir));
        
        if env::var("HEADLESS").is_ok() {
            flags.push("--headless".to_string());
        }
        flags.extend(self.chrome_flags.clone());
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
