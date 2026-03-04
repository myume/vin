use strum::{Display, EnumString};
use uinput::event::keyboard::Key;

#[derive(Debug, PartialEq)]
pub enum Statement {
    KeyboardEvent(KeyboardEvent),
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
