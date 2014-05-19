
use graphics::Value;
use piston::keyboard;
use {
    AddPress,
    AddPressing,
    KeyboardPressEvent,
    KeyboardPressingEvent,
};

pub struct KeyboardEvent<'a>;

impl<'a> AddPress<'a, KeyboardPressEvent<'a>> for KeyboardEvent<'a> {
    #[inline(always)]
    fn press(&self, key: keyboard::Key) -> KeyboardPressEvent<'a> {
        KeyboardPressEvent {
            key: Value(key),
        }
    }
}

impl<'a> AddPressing<'a, KeyboardPressingEvent<'a>> for KeyboardEvent<'a> {
    #[inline(always)]
    fn pressing(&self, key: keyboard::Key) -> KeyboardPressingEvent<'a> {
        KeyboardPressingEvent {
            key: Value(key),
        }
    }
}

