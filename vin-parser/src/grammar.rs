use strum::EnumString;
use uinput::event::keyboard::Key;

#[derive(Debug, PartialEq)]
pub enum Statement {
    KeyboardEvent(KeyboardEvent),
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum KeyboardCommands {
    #[strum(serialize = "PRESS")]
    KeyPress,
    #[strum(serialize = "SEND")]
    Send,
}

#[derive(Debug, PartialEq)]
pub enum KeyboardEvent {
    KeyPress { key: Key },
    Send { keys: Vec<Key> },
}
