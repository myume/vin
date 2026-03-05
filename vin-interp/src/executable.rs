use std::{thread::sleep, time::Duration};

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
            Statement::NOOP => return Ok(()),
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
        for _ in 0..self.times {
            for statement in self.body.iter() {
                statement.execute(interp)?;

                // for some reason writing rapidly to uinput causes incomplete outputs
                // sleep somehow fixes it, seems like some sort of weird race condition
                sleep(Duration::from_millis(1));
            }
        }

        Ok(())
    }
}
