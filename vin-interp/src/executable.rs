use thiserror::Error;
use vin_parser::grammar::{KeyboardEvent, Repeat, Statement};

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
            Statement::Repeat(repeat) => repeat.execute(interp)?,
        }
        interp
            .device
            .synchronize()
            .map_err(ExecuteError::DeviceError)?;

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
            KeyboardEvent::Send { keys } => {
                for key in keys {
                    interp
                        .device
                        .click(key)
                        .map_err(ExecuteError::DeviceError)?
                }
            }
            KeyboardEvent::Hold { key } => interp
                .device
                .press(key)
                .map_err(ExecuteError::DeviceError)?,
            KeyboardEvent::Release { key } => interp
                .device
                .release(key)
                .map_err(ExecuteError::DeviceError)?,
        }

        Ok(())
    }
}

impl Executable for Repeat {
    fn execute(&self, interp: &mut Interpreter) -> Result<(), ExecuteError> {
        match self.times {
            Some(times) => {
                for _ in 0..times {
                    for statement in self.body.iter() {
                        statement.execute(interp)?
                    }
                }
            }
            None => loop {
                for statement in self.body.iter() {
                    statement.execute(interp)?
                }
            },
        }

        Ok(())
    }
}
