mod cli;
mod config;
mod notifications;
mod state;
mod timer;

use cli::{parse_args, CliCommand};
use config::Config;
use state::PomodoroState;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let command = parse_args();

    let config = Config::load_or_default()?;
    println!("Stopping the timer...");
    let state = PomodoroState::new(&config);

    match command {
        CliCommand::Start {
            work_duration,
            break_duration,
            watch_timer,
        } => {
            println!("Loading state from \"{}\"", config.filepath);
            if state.is_active()? {
                return Ok(());
            }

            let config = Config::new(
                work_duration.unwrap_or(config.work_duration),
                break_duration.unwrap_or(config.break_duration),
                config.filepath,
                watch_timer,
            );

            let state = PomodoroState::new(&config);
            state.save()?;

            timer::run_pomodoro(config)?;
            println!("Pomodoro session started.");
        }
        CliCommand::Status => match state.load()? {
            Some(state) => {
                let status = state.get_formatted_status()?;
                println!("{}", status);
            }
            None => println!("No active Pomodoro session."),
        },
        CliCommand::Stop => {
            if state.is_active()? {
                state.delete()?;
                println!("Pomodoro session stopped.");
            } else {
                println!("No active Pomodoro session to stop.");
            }
        }
        CliCommand::EnableWatch => {
            if state.is_active()? {
                let config = Config::new(
                    config.work_duration,
                    config.break_duration,
                    config.filepath,
                    true,
                );
                let mut state = PomodoroState::new(&config);
                let res = timer::watch_pomodoro(&mut state);
                if let Err(e) = res {
                    println!("Error: {}", e);
                }
                println!("Watching the timer is now {}.", config.watch_timer);
            } else {
                println!("No active Pomodoro session to watch.");
            }
        }
    }

    Ok(())
}
