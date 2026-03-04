use thiserror::Error;
use vin_parser::grammar::{KeyboardEvent, Statement};

use crate::interpreter::Interpreter;

pub trait Executable {
    fn execute(&self, interp: &mut Interpreter) -> Result<(), ExecuteError>;
}

#[derive(Debug, Error)]
pub enum ExecuteError {
    #[error("Failed to send event to device: {0:?}")]
    DeviceError(uinput::Error),
}

impl Executable for Statement {
    fn execute(&self, interp: &mut Interpreter) -> Result<(), ExecuteError> {
        match self {
            Statement::KeyboardEvent(keyboard_event) => keyboard_event.execute(interp)?,
        }

        Ok(())
    }
}

impl Executable for KeyboardEvent {
    fn execute(&self, interp: &mut Interpreter) -> Result<(), ExecuteError> {
        match self {
            KeyboardEvent::KeyPress { key } => interp
                .device
                .click(key)
                .map_err(ExecuteError::DeviceError)?,
        }

        Ok(())
    }
}
