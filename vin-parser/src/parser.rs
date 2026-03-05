use std::str::FromStr;

use thiserror::Error;
use uinput::event::keyboard::Key;

use crate::grammar::{
    KeyboardCommands, KeyboardEvent, REPEAT_COMMAND, REPEAT_OPEN, REPEAT_TERMINATOR, Repeat,
    Statement,
};

#[derive(Default)]
pub struct Parser {}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid send command: {0}")]
    InvalidSend(String),

    #[error("Invalid repeat command")]
    InvalidRepeat,

    #[error("Bad repeat instruction: {0}")]
    BadRepeat(String),

    #[error("Invalid Key \"{0}\"")]
    InvalidKey(String),

    #[error("Invalid Command \"{0}\"")]
    InvalidKeyboardCommand(String),

    #[error("Missing Keyboard Command")]
    MissingKeyboardCommand,

    #[error("Incomplete repeat statement")]
    IncompleteStatement,

    #[error("Invalid Statement \"{0}\"")]
    InvalidStatement(String),
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

    fn parse_repeat(&self, s: &str) -> Result<Repeat, ParseError> {
        let mut lines = s.lines();
        let Some(repeat_start) = lines.next() else {
            return Err(ParseError::InvalidRepeat);
        };

        let parts: Vec<&str> = repeat_start.split(" ").collect();
        if parts.len() < 2 || parts.len() > 3 || parts[0].to_uppercase() != REPEAT_COMMAND {
            return Err(ParseError::InvalidRepeat);
        }

        if parts.last() != Some(&REPEAT_OPEN) {
            return Err(ParseError::BadRepeat(
                "Missing open brace on repeat".to_string(),
            ));
        }

        let times: Option<u32> = if parts.len() == 3 {
            Some(parts[1].parse().map_err(|_| {
                ParseError::BadRepeat(format!("Invalid number of repetitions: \"{}\"", parts[1]))
            })?)
        } else {
            None
        };

        let mut repeat = Repeat {
            times,
            body: Vec::new(),
        };

        for line in lines {
            let line = line.trim();
            if line == REPEAT_TERMINATOR {
                return Ok(repeat);
            }

            let statement = self.parse_statement(line)?;
            repeat.body.push(statement);
        }

        Err(ParseError::IncompleteStatement)
    }

    pub fn parse_statement(&self, s: &str) -> Result<Statement, ParseError> {
        match self.parse_repeat(s) {
            Ok(repeat) => return Ok(Statement::Repeat(repeat)),
            Err(e) => {
                if !matches!(e, ParseError::InvalidRepeat) {
                    return Err(e);
                }
            }
        }

        match self.parse_keyboard_event(s) {
            Ok(event) => return Ok(Statement::KeyboardEvent(event)),
            Err(e) => {
                if !matches!(e, ParseError::MissingKeyboardCommand)
                    || !matches!(e, ParseError::InvalidKeyboardCommand(_))
                {
                    return Err(e);
                }
            }
        };

        Err(ParseError::InvalidStatement(s.to_owned()))
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

    #[test]
    fn test_parse_repeat() {
        let statement = "REPEAT 10 {\n\tSEND hello world\n}";
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
        let send = Statement::KeyboardEvent(KeyboardEvent::Send { keys });
        assert_eq!(
            res,
            Statement::Repeat(Repeat {
                times: Some(10),
                body: vec![send]
            })
        )
    }

    #[test]
    fn test_parse_repeat_forever() {
        let statement = "REPEAT {\n\tSEND hello world\n}";
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
        let send = Statement::KeyboardEvent(KeyboardEvent::Send { keys });
        assert_eq!(
            res,
            Statement::Repeat(Repeat {
                times: None,
                body: vec![send]
            })
        )
    }
}
