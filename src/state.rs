use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct PomodoroState {
    pub start_time: u64,
    pub work_duration: u64,
    pub break_duration: u64,
    pub is_working: bool,
    pub filepath: String,
}

impl PomodoroState {
    pub fn new(config: &Config) -> Self {
        Self {
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            filepath: config.filepath.clone(),
            work_duration: config.work_duration,
            break_duration: config.break_duration,
            is_working: true,
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let serialized =
            serde_json::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if let Some(parent) = Path::new(&self.filepath).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.filepath, serialized)
    }

    pub fn load(&self) -> io::Result<Option<Self>> {
        if !Path::new(&self.filepath).exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&self.filepath)?;
        let state: PomodoroState =
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Some(state))
    }

    pub fn is_active(&self) -> io::Result<bool> {
        Ok(self.load()?.is_some())
    }

    pub fn delete(&self) -> io::Result<()> {
        if Path::new(&self.filepath).exists() {
            fs::remove_file(&self.filepath)?;
        }
        Ok(())
    }

    pub fn get_formatted_status(&self) -> io::Result<String> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let elapsed = current_time - self.start_time;
        let remaining = if self.is_working {
            self.work_duration.saturating_sub(elapsed)
        } else {
            self.break_duration.saturating_sub(elapsed)
        };

        Ok(format!(
            "{}: {:02}:{:02} remaining",
            if self.is_working { "Work" } else { "Break" },
            remaining / 60,
            remaining % 60
        ))
    }
}
