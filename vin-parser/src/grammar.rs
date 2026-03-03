use uinput::event::keyboard::Key;

pub const KEYPRESS: &str = "PRESS";

#[derive(Debug, PartialEq)]
pub enum Statement {
    KeyboardEvent(KeyboardEvent),
}

#[derive(Debug, PartialEq)]
pub enum KeyboardEvent {
    KeyPress { key: Key },
}
