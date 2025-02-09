use crate::config::Config;
use crate::state::PomodoroState;

pub fn run_pomodoro(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let state = PomodoroState::new(&config);
    state.save()?;

    println!("Pomodoro timer started. Press Ctrl+C to exit.");

    if config.watch_timer {
        state.watch()?;
    }
    Ok(())
}
