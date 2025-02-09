use clap::{Arg, ArgMatches, Command};

#[derive(Debug)]
pub enum CliCommand {
    Start {
        work_duration: Option<u64>,
        break_duration: Option<u64>,
        watch_timer: bool,
    },
    Status,
    Stop,
    EnableWatch,
}

pub fn parse_args() -> CliCommand {
    let matches = Command::new("Pomodoro Timer")
        .version("1.0")
        .author("mhiri <ahmedmhiri218@gmail.com>")
        .about("A terminal-based Pomodoro timer")
        .subcommand(
            Command::new("start")
                .about("Start a Pomodoro session")
                .arg(
                    Arg::new("work")
                        .short('w')
                        .long("work")
                        .value_name("MINUTES")
                        .help("Sets the work duration in minutes"),
                )
                .arg(
                    Arg::new("break")
                        .short('b')
                        .long("break")
                        .value_name("MINUTES")
                        .help("Sets the break duration in minutes"),
                )
                .arg(
                    Arg::new("watch")
                        .short('W')
                        .long("watch")
                        .action(clap::ArgAction::SetTrue)
                        .required(false)
                        .help("Enables watching the timer"),
                ),
        )
        .subcommand(Command::new("status").about("Check the status of the Pomodoro timer"))
        .subcommand(Command::new("stop").about("Stop the Pomodoro timer"))
        .subcommand(Command::new("watch").about("Enable watching the timer"))
        .get_matches();

    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            let work_duration = parse_duration_arg(sub_matches, "work");
            let break_duration = parse_duration_arg(sub_matches, "break");
            let watch_timer = sub_matches.get_flag("watch");

            CliCommand::Start {
                work_duration,
                break_duration,
                watch_timer,
            }
        }
        Some(("status", _)) => CliCommand::Status,
        Some(("stop", _)) => CliCommand::Stop,
        Some(("watch", _)) => CliCommand::EnableWatch,
        _ => CliCommand::Status,
    }
}

fn parse_duration_arg(matches: &ArgMatches, arg_name: &str) -> Option<u64> {
    matches
        .get_one::<String>(arg_name)
        .and_then(|val| val.parse::<u64>().ok())
        .map(|minutes| minutes * 60)
}
