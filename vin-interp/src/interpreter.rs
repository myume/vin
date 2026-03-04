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
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            parser: Parser::default(),
            device: uinput::default()
                .unwrap()
                .name("vin_device")
                .unwrap()
                .event(uinput::event::Keyboard::All)
                .unwrap()
                .create()
                .unwrap(),
        }
    }

    pub fn execute(&mut self, line: &str) -> Result<(), InterpreterError> {
        let statement = self.parser.parse_statement(line)?;
        statement.execute(self)?;
        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
