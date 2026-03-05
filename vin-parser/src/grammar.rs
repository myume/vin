use strum::{Display, EnumString};
use uinput::event::keyboard::Key;

pub const REPEAT_OPEN: &str = "{";
pub const REPEAT_TERMINATOR: &str = "}";
pub const REPEAT_COMMAND: &str = "REPEAT";

#[derive(Debug, PartialEq)]
pub enum Statement {
    KeyboardEvent(KeyboardEvent),
    Repeat(Repeat),
    NOOP,
}

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum KeyboardCommands {
    #[strum(serialize = "PRESS")]
    KeyPress,
    #[strum(serialize = "SEND")]
    Send,
    #[strum(serialize = "HOLD")]
    Hold,
    #[strum(serialize = "RELEASE")]
    Release,
}

#[derive(Debug, PartialEq)]
pub enum KeyboardEvent {
    KeyPress { key: Key },
    Hold { key: Key },
    Release { key: Key },
    Send { keys: Vec<Key> },
}

#[derive(Debug, PartialEq)]
pub struct Repeat {
    pub times: Option<u32>,
    pub body: Vec<Statement>,
}
