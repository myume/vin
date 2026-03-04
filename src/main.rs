use std::path::PathBuf;

use clap::Parser;
use vin_interp::interpreter::Interpreter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut interpreter = match Interpreter::new() {
        Ok(interpreter) => interpreter,
        Err(e) => {
            eprintln!("{e:?}");
            std::process::exit(-1);
        }
    };

    let e = match args.file {
        Some(file) => interpreter.run(&file),
        None => interpreter.repl(),
    };

    if let Err(e) = e {
        eprintln!("{e:?}");
        std::process::exit(-1);
    }
}
