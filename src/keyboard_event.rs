
use graphics::Value;
use piston::keyboard;
use {
    AddHolding,
    AddPress,
    KeyboardHoldingEvent,
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

impl<'a> AddHolding<'a, KeyboardHoldingEvent<'a>> for KeyboardEvent<'a> {
    fn holding(&self, key: keyboard::Key) -> KeyboardHoldingEvent<'a> {
        KeyboardHoldingEvent {
            key: Value(key),
        }
    }
}

