use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::Parser;
use vin_interp::interpreter::Interpreter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

fn run(mut interpreter: Interpreter, file: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        interpreter.execute(line.trim_end())?;
        line.clear();
    }

    Ok(())
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
