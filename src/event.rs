
use {
    AddKeyboard,
    KeyboardEvent,
};

pub struct Event<'a>;

impl<'a> Event<'a> {
    pub fn new() -> Event {
        Event
    }
}

impl<'a> AddKeyboard<'a, KeyboardEvent<'a>> for Event<'a> {
    fn keyboard(&self) -> KeyboardEvent<'a> {
        KeyboardEvent
    }
}

