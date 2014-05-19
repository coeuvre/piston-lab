
use graphics::{
    Borrowed,
    Field,
    Value,
};
use piston::keyboard;
use piston::event::{
    Event,
    KeyPressed,
    KeyReleased,
};

use {
    AddLasting,
    KeyboardHoldingLastingEvent,

    BackEnd,
    Map,
    Observer,
};

pub struct KeyboardHoldingEvent<'a> {
    pub key: Field<'a, keyboard::Key>,
}

impl<'a> Map<'a> for KeyboardHoldingEvent<'a> {
    fn map<'a, B: BackEnd>(&self, back_end: &mut B, command: ||: 'a) -> uint {
        back_end.add_observer(box KeyboardHoldingEventObserver::new(command, *self.key.get()))
    }
}

impl<'a> AddLasting<'a, KeyboardHoldingLastingEvent<'a>> for KeyboardHoldingEvent<
'a> {
    fn lasting(&'a self, time: f64) -> KeyboardHoldingLastingEvent<'a> {
        KeyboardHoldingLastingEvent {
            key: Borrowed(self.key.get()),
            lasting: Value(time),
        }
    }
}

struct KeyboardHoldingEventObserver<'a> {
    command: ||: 'a,
    key: keyboard::Key,
    is_holding: bool,
}

impl<'a> KeyboardHoldingEventObserver<'a> {
    pub fn new<'a>(command: ||: 'a, key: keyboard::Key) -> KeyboardHoldingEventObserver<'a> {
        KeyboardHoldingEventObserver {
            command: command,
            key: key,
            is_holding: false,
        }
    }
}

impl<'a> Observer for KeyboardHoldingEventObserver<'a> {
    fn can_trigger(&self) -> bool {
        self.is_holding
    }

    fn trigger(&mut self) {
        (self.command)();
    }

    fn on_event(&mut self, e: Event) {
        match e {
            KeyPressed(key) if key == self.key => {
                self.is_holding = true;
            },
            KeyReleased(key) if key == self.key => {
                self.is_holding = false;
            }
            _ => {}
        }
    }
}
