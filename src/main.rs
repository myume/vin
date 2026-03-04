use std::{
    error::Error,
    io::{self},
    path::{Path, PathBuf},
};

use clap::Parser;
use vin_interp::interpreter::Interpreter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let interpreter = match Interpreter::new() {
        Ok(interpreter) => interpreter,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(-1);
        }
    };

    let e = match args.file {
        Some(file) => run(interpreter, &file),
        None => repl(interpreter),
    };

    if let Err(e) = e {
        eprintln!("{e}");
        std::process::exit(-1);
    }
}

fn run(interpreter: Interpreter, file: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn repl(mut interpreter: Interpreter) -> Result<(), Box<dyn Error>> {
    eprintln!("Virtual INput Repl");

    let mut line = String::new();
    loop {
        eprint!("\r> ");
        io::stdin().read_line(&mut line)?;
        if let Err(e) = interpreter.execute(line.trim_end()) {
            eprintln!("{e}");
        };

        line.clear();
    }
}
