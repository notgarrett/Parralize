use clap::Parser;
pub mod hardware_search;
pub mod listener;

pub enum Errors {
    MissingArguements(String),
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    listen: bool,

    #[arg(short, long)]
    port: String,

    #[arg(short, long)]
    send: bool,

    #[arg(short, long)]
    command: String,
}

pub fn parse_args() -> Result<(), Errors> {
    let args = Args::parse();

    let state = (
        args.listen,
        !args.port.is_empty(),
        !args.command.is_empty(),
        args.send,
    );

    // Listen, Port, Command, Send
    match state {
        // Successful listen command
        (true, true, _, _) => {}
        // Failed missing command
        (true, false, _, _) => {
            return Err(Errors::MissingArguements(
                "A port must be specified for the listener command.".to_string(),
            ))
        }
        // Successful send command
        (_, true, true, true) => {}
        // Failed send command
        (_, true, false, true) => {
            return Err(Errors::MissingArguements(
                "A command must be specified for the send command to function.".to_string(),
            ))
        }
        // Failed send command
        (_, _, _, false) => {}
        _ => {}
    };

    todo!()
}
