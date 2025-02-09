use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

const DEFAULT_WORK_DURATION: u64 = 25 * 60;
const DEFAULT_BREAK_DURATION: u64 = 5 * 60;
const DEFAULT_CONFIG_PATH: &str = "/tmp/pomodoro.state";
const DEFAULT_WATCH_TIMER: bool = true;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub work_duration: u64,
    pub break_duration: u64,
    pub filepath: String,
    #[serde(skip_serializing, default)]
    pub watch_timer: bool,
}

impl Config {
    pub fn new(
        work_duration: u64,
        break_duration: u64,
        filepath: String,
        watch_timer: bool,
    ) -> Self {
        Self {
            work_duration,
            break_duration,
            filepath,
            watch_timer,
        }
    }

    pub fn load_or_default() -> io::Result<Self> {
        let config_path = Path::new(DEFAULT_CONFIG_PATH);

        if config_path.exists() {
            let contents = fs::read_to_string(config_path)?;
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_duration: DEFAULT_WORK_DURATION,
            break_duration: DEFAULT_BREAK_DURATION,
            filepath: DEFAULT_CONFIG_PATH.to_string(),
            watch_timer: DEFAULT_WATCH_TIMER,
        }
    }
}
