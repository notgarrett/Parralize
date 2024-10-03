use clap::Parser;

enum Errors {
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

fn main() {
    let args = Args::parse();

    let state = (
        args.listen,
        !args.port.is_empty(),
        !args.command.is_empty(),
        args.send,
    );

    match state {
        // Successful listen command
        (true, true, _, _) => {}
        // Failed missing command
        (true, false, _, _) => {}
        // Successful send command
        (_, true, true, true) => {}
        // Failed send command
        (_, true, false, true) => {}
        // Failed send command
        (_, _, _, false) => {}
        _ => {}
    }
}
