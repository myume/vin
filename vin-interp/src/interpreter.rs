use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    thread::sleep,
    time::Duration,
};

use anyhow::Context;
use uinput::Device;
use vin_parser::parser::{ParseError, Parser};

use crate::executable::{Executable, ExecuteError};

pub struct Interpreter {
    parser: Parser,
    pub(crate) device: Device,
}

#[derive(Debug, thiserror::Error)]
pub enum InterpreterError {
    #[error("Failed to parse statement")]
    ParseError(#[from] ParseError),

    #[error("Failed to execute statement")]
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

    pub fn run(&mut self, file: &Path) -> anyhow::Result<()> {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);

        let mut line = String::new();
        let mut i = 0;
        while reader.read_line(&mut line)? > 0 {
            i += 1;

            if line.trim().is_empty() {
                line.clear();
                continue;
            }

            self.execute(line.trim_end())
                .context(format!("Failed to execute line {i}"))?;
            line.clear();
        }

        Ok(())
    }

    pub fn repl(&mut self) -> anyhow::Result<()> {
        eprintln!("Virtual INput Repl");

        let mut line = String::new();
        loop {
            eprint!("> ");
            if io::stdin().read_line(&mut line)? == 0 {
                eprintln!();
                break;
            };

            if let Err(e) = self
                .execute(line.trim_end())
                .context("Failed to execute statement")
            {
                eprintln!("{e:?}");
            };

            line.clear();
        }
        Ok(())
    }
}
