use std::str::FromStr;

use thiserror::Error;
use uinput::event::keyboard::Key;

use crate::grammar::{KEYPRESS, KeyboardEvent, Statement};

#[derive(Default)]
pub struct Parser {}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid Key {0}")]
    InvalidKey(String),

    #[error("Invalid Command {0}")]
    InvalidKeyboardCommand(String),

    #[error("Missing Keyboard Command")]
    MissingKeyboardCommand,

    #[error("Invalid Statement")]
    InvalidStatement,
}

impl Parser {
    fn parse_keypress(&self, parts: &[&str]) -> Result<KeyboardEvent, ParseError> {
        if parts.len() != 2 {
            return Err(ParseError::MissingKeyboardCommand);
        }

        Ok(KeyboardEvent::KeyPress {
            key: Key::from_str(parts[1])
                .map_err(|_| ParseError::InvalidKey(parts[1].to_owned()))?,
        })
    }

    fn parse_keyboard_event(&self, s: &str) -> Result<KeyboardEvent, ParseError> {
        let parts: Vec<&str> = s.split(" ").collect();
        let Some(&command) = parts.first() else {
            return Err(ParseError::MissingKeyboardCommand);
        };

        match command.to_uppercase().as_str() {
            KEYPRESS => self.parse_keypress(&parts),
            _ => Err(ParseError::InvalidKeyboardCommand(command.to_owned())),
        }
    }

    pub fn parse_statement(&self, s: &str) -> Result<Statement, ParseError> {
        match self.parse_keyboard_event(s) {
            Ok(event) => return Ok(Statement::KeyboardEvent(event)),
            Err(e) => {
                if !matches!(e, ParseError::InvalidKeyboardCommand(_)) {
                    return Err(e);
                }
            }
        }

        Err(ParseError::InvalidStatement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keypress() {
        for (key_name, key) in Key::iter_variant_names().zip(Key::iter_variants()) {
            let statement = format!("PRESS {key_name}");
            let res = Parser::default().parse_statement(&statement).unwrap();
            assert_eq!(
                res,
                Statement::KeyboardEvent(KeyboardEvent::KeyPress { key })
            )
        }
    }
}
