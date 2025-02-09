use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config::Config;
use crate::notifications;
use crate::state::PomodoroState;

pub fn run_pomodoro(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = PomodoroState::new(&config);
    state.save()?;

    println!("Pomodoro timer started. Press Ctrl+C to exit.");

    if config.watch_timer {
        watch_pomodoro(&mut state)?;
    }
    Ok(())
}

pub fn watch_pomodoro(state: &mut PomodoroState) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - state.start_time;

        let remaining = if state.is_working {
            state.work_duration - elapsed
        } else {
            state.break_duration - elapsed
        };

        print!(
            "\r{} remaining: {:02}:{:02}    ",
            if state.is_working { "Work" } else { "Break" },
            remaining / 60,
            remaining % 60
        );
        std::io::Write::flush(&mut std::io::stdout())?;

        if remaining == 0 {
            if state.is_working {
                notifications::notify("Pomodoro", "Time for a break!");
                state.is_working = false;
            } else {
                notifications::notify("Pomodoro", "Back to work!");
                state.is_working = true;
            }
            state.start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            state.save()?;
            println!();
        }

        thread::sleep(Duration::from_secs(1));
    }
}
