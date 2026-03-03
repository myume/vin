use std::str::FromStr;

use uinput::event::keyboard::Key;

use crate::grammar::{KEYPRESS, KeyboardEvent, Statement};

#[derive(Default)]
pub struct Parser {}

#[derive(Debug)]
pub enum ParseError {
    InvalidKey,
    InvalidKeyboardCommand,
    MissingKeyboardCommand,
    InvalidStatement,
}

impl Parser {
    fn parse_keypress(&self, parts: &[&str]) -> Result<KeyboardEvent, ParseError> {
        if parts.len() != 2 {
            return Err(ParseError::InvalidKeyboardCommand);
        }

        Ok(KeyboardEvent::KeyPress {
            key: Key::from_str(parts[1]).map_err(|_| ParseError::InvalidKey)?,
        })
    }

    fn parse_keyboard_event(&self, s: &str) -> Result<KeyboardEvent, ParseError> {
        let parts: Vec<&str> = s.split(" ").collect();
        let Some(&command) = parts.first() else {
            return Err(ParseError::MissingKeyboardCommand);
        };

        match command {
            KEYPRESS => self.parse_keypress(&parts),
            _ => Err(ParseError::InvalidKeyboardCommand),
        }
    }

    pub fn parse_statement(&self, s: &str) -> Result<Statement, ParseError> {
        if let Ok(event) = self.parse_keyboard_event(s) {
            return Ok(Statement::KeyboardEvent(event));
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
