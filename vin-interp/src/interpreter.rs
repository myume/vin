use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    thread::sleep,
    time::Duration,
};

use uinput::Device;
use vin_parser::parser::{ParseError, Parser};

use crate::executable::{Executable, ExecuteError};

pub struct Interpreter {
    parser: Parser,
    pub(crate) device: Device,
}

#[derive(Debug, thiserror::Error)]
pub enum InterpreterError {
    #[error("Failed to parse statement: {0}")]
    ParseError(#[from] ParseError),

    #[error("Failed to execute statement: {0}")]
    ExecuteError(#[from] ExecuteError),

    #[error("Could create virtual device: {0}")]
    DeviceInit(uinput::Error),
}

impl Interpreter {
    pub fn new() -> Result<Self, InterpreterError> {
        let device = uinput::default()
            .map_err(InterpreterError::DeviceInit)?
            .name("vin_device")
            .map_err(InterpreterError::DeviceInit)?
            .event(uinput::event::Keyboard::All)
            .map_err(InterpreterError::DeviceInit)?
            .create()
            .map_err(InterpreterError::DeviceInit)?;

        // unsure why i need to sleep. probably has something to do with setting up the device
        sleep(Duration::from_secs(1));

        Ok(Interpreter {
            parser: Parser::default(),
            device,
        })
    }

    pub fn execute(&mut self, line: &str) -> Result<(), InterpreterError> {
        let statement = self.parser.parse_statement(line)?;
        statement.execute(self)?;
        Ok(())
    }

    pub fn run(&mut self, file: &Path) -> Result<(), Box<dyn Error>> {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);

        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            self.execute(line.trim_end())?;
            line.clear();
        }

        Ok(())
    }

    pub fn repl(&mut self) -> Result<(), Box<dyn Error>> {
        eprintln!("Virtual INput Repl");

        let mut line = String::new();
        loop {
            eprint!("> ");
            io::stdin().read_line(&mut line)?;
            if let Err(e) = self.execute(line.trim_end()) {
                eprintln!("{e}");
            };

            line.clear();
        }
    }
}
