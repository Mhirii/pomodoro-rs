use crate::config::Config;
use crate::notifications;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;
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

    pub fn watch(mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            if current_time < self.start_time {
                self.start_time = current_time;
            }

            let elapsed = current_time - self.start_time;

            let total_duration = if self.is_working {
                self.work_duration
            } else {
                self.break_duration
            };

            let remaining = if elapsed >= total_duration {
                0
            } else {
                total_duration - elapsed
            };

            print!("\r{}", self.get_formatted_status()?);
            std::io::Write::flush(&mut std::io::stdout())?;

            if remaining == 0 {
                if self.is_working {
                    notifications::notify("Pomodoro", "Time for a break!");
                    self.is_working = false;
                } else {
                    notifications::notify("Pomodoro", "Back to work!");
                    self.is_working = true;
                }
                self.start_time = current_time;
                self.save()?;
                println!();
            }

            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn check_update_interval(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let total_duration = if self.is_working {
            println!("WORK - Current time: {}", current_time);
            self.work_duration
        } else {
            println!("BREAK - Current time: {}", current_time);
            self.break_duration
        };

        let elapsed = current_time - self.start_time;

        if elapsed >= total_duration {
            println!(
                "Time for a {} session. elapsed: {}, total: {}",
                if self.is_working { "break" } else { "work" },
                elapsed,
                total_duration
            );
            self.is_working = !self.is_working;
            self.start_time = current_time;
        }
        Ok(())
    }
}
