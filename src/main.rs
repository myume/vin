use clap::{Parser, Subcommand};
use vin_interp::interpreter::Interpreter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start a repl
    Repl,
}

fn main() {
    let args = Args::parse();

    let interpreter = match Interpreter::new() {
        Ok(interpreter) => interpreter,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(-1);
        }
    };

    match args.command {
        Some(command) => match command {
            Command::Repl => {
                // run in repl mode
            }
        },
        None => {
            // default interpreter
        }
    }
}
