use std::{thread::sleep, time::Duration};

use thiserror::Error;
use uinput::Device;
use vin_parser::parser::{ParseError, Parser};

use crate::executable::{Executable, ExecuteError};

pub struct Interpreter {
    parser: Parser,
    pub(crate) device: Device,
}

#[derive(Debug, Error)]
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
}
