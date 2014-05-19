
use graphics::Value;
use piston::keyboard;
use {
    AddPressing,
    AddPress,
    KeyboardPressingEvent,
    KeyboardPressEvent,
};

pub struct KeyboardEvent<'a>;

impl<'a> AddPress<'a, KeyboardPressEvent<'a>> for KeyboardEvent<'a> {
    fn press(&self, key: keyboard::Key) -> KeyboardPressEvent<'a> {
        KeyboardPressEvent {
            key: Value(key),
        }
    }
}

impl<'a> AddPressing<'a, KeyboardPressingEvent<'a>> for KeyboardEvent<'a> {
    fn pressing(&self, key: keyboard::Key) -> KeyboardPressingEvent<'a> {
        KeyboardPressingEvent {
            key: Value(key),
        }
    }
}

