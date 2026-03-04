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
        Ok(Interpreter {
            parser: Parser::default(),
            device: uinput::default()
                .map_err(InterpreterError::DeviceInit)?
                .name("vin_device")
                .map_err(InterpreterError::DeviceInit)?
                .event(uinput::event::Keyboard::All)
                .map_err(InterpreterError::DeviceInit)?
                .create()
                .map_err(InterpreterError::DeviceInit)?,
        })
    }

    pub fn execute(&mut self, line: &str) -> Result<(), InterpreterError> {
        let statement = self.parser.parse_statement(line)?;
        statement.execute(self)?;
        Ok(())
    }
}
