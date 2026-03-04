use std::str::FromStr;

use thiserror::Error;
use uinput::event::keyboard::Key;

use crate::grammar::{KeyboardCommands, KeyboardEvent, Statement};

#[derive(Default)]
pub struct Parser {}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid send command: {0}")]
    InvalidSend(String),

    #[error("Invalid Key \"{0}\"")]
    InvalidKey(String),

    #[error("Invalid Command \"{0}\"")]
    InvalidKeyboardCommand(String),

    #[error("Missing Keyboard Command")]
    MissingKeyboardCommand,

    #[error("Invalid Statement")]
    InvalidStatement,
}

impl Parser {
    fn parse_key(&self, parts: &[&str]) -> Result<Key, ParseError> {
        if parts.len() < 2 {
            return Err(ParseError::MissingKeyboardCommand);
        }

        if parts.len() > 2 {
            return Err(ParseError::InvalidKey(parts[1..].join(" ")));
        }

        Key::from_str(parts[1]).map_err(|_| ParseError::InvalidKey(parts[1].to_owned()))
    }

    fn parse_send(&self, message: &str) -> Result<KeyboardEvent, ParseError> {
        let mut keys = Vec::new();
        for c in message.chars() {
            let key =
                Key::from_str(&c.to_string()).map_err(|_| ParseError::InvalidKey(c.to_string()))?;
            keys.push(key);
        }

        Ok(KeyboardEvent::Send { keys })
    }

    fn parse_keyboard_event(&self, s: &str) -> Result<KeyboardEvent, ParseError> {
        let parts: Vec<&str> = s.split(" ").collect();
        let Some(&command) = parts.first() else {
            return Err(ParseError::MissingKeyboardCommand);
        };

        match KeyboardCommands::from_str(command.to_uppercase().as_str()) {
            Ok(command) => match command {
                KeyboardCommands::KeyPress => Ok(KeyboardEvent::KeyPress {
                    key: self.parse_key(&parts)?,
                }),
                KeyboardCommands::Send => self.parse_send(&parts[1..].join(" ")),
                KeyboardCommands::Hold => Ok(KeyboardEvent::Hold {
                    key: self.parse_key(&parts)?,
                }),
                KeyboardCommands::Release => Ok(KeyboardEvent::Release {
                    key: self.parse_key(&parts)?,
                }),
            },
            Err(_) => Err(ParseError::InvalidKeyboardCommand(command.to_owned())),
        }
    }

    pub fn parse_statement(&self, s: &str) -> Result<Statement, ParseError> {
        match self.parse_keyboard_event(s) {
            Ok(event) => Ok(Statement::KeyboardEvent(event)),
            Err(e) => Err(e),
        }

        // Err(ParseError::InvalidStatement)
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_parse_keypress() {
        for key in Key::iter() {
            let statement = format!("{} {}", KeyboardCommands::KeyPress, key);
            let res = Parser::default().parse_statement(&statement).unwrap();
            assert_eq!(
                res,
                Statement::KeyboardEvent(KeyboardEvent::KeyPress { key })
            )
        }
    }

    #[test]
    fn test_parse_hold() {
        for key in Key::iter() {
            let statement = format!("{} {}", KeyboardCommands::Hold, key);
            let res = Parser::default().parse_statement(&statement).unwrap();
            assert_eq!(res, Statement::KeyboardEvent(KeyboardEvent::Hold { key }))
        }
    }

    #[test]
    fn test_parse_release() {
        for key in Key::iter() {
            let statement = format!("{} {}", KeyboardCommands::Release, key);
            let res = Parser::default().parse_statement(&statement).unwrap();
            assert_eq!(
                res,
                Statement::KeyboardEvent(KeyboardEvent::Release { key })
            )
        }
    }

    #[test]
    fn test_send_message() {
        let statement = "SEND HELLO WORLD";
        let res = Parser::default().parse_statement(statement).unwrap();
        let keys = vec![
            Key::H,
            Key::E,
            Key::L,
            Key::L,
            Key::O,
            Key::Space,
            Key::W,
            Key::O,
            Key::R,
            Key::L,
            Key::D,
        ];
        assert_eq!(res, Statement::KeyboardEvent(KeyboardEvent::Send { keys }))
    }
}
