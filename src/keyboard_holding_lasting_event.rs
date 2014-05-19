
use graphics::Field;
use piston::keyboard;
use piston::event::{
    Event,
    KeyPressed,
    KeyReleased,
};
use {
    BackEnd,
    Map,
    Observer,
};

pub struct KeyboardHoldingLastingEvent<'a> {
    pub key: Field<'a, keyboard::Key>,
    pub lasting: Field<'a, f64>,
}

impl<'a> Map<'a> for KeyboardHoldingLastingEvent<'a> {
    fn map<'a, B: BackEnd>(&self, back_end: &mut B, command: ||: 'a) -> uint {
        back_end.add_observer(box KeyboardHoldingLastingEventObserver::new(command, *self.key.get(), *self.lasting.get()))
    }
}

struct KeyboardHoldingLastingEventObserver<'a> {
    command: ||: 'a,
    key: keyboard::Key,
    is_holding: bool,
    cur_time: f64,
    lasting_time: f64,
}

impl<'a> KeyboardHoldingLastingEventObserver<'a> {
    pub fn new<'a>(command: ||: 'a, key: keyboard::Key, lasting: f64) -> KeyboardHoldingLastingEventObserver<'a> {
        KeyboardHoldingLastingEventObserver {
            command: command,
            key: key,
            is_holding: false,
            cur_time: 0.0,
            lasting_time: lasting,
        }
    }
}

impl<'a> Observer for KeyboardHoldingLastingEventObserver<'a> {
    fn can_trigger(&self) -> bool {
        self.is_holding && self.cur_time > self.lasting_time
    }

    fn trigger(&mut self) {
        (self.command)();
    }

    fn update(&mut self, dt: f64) {
        if self.is_holding {
            self.cur_time += dt;
        }
    }

    fn on_event(&mut self, e: Event) {
        match e {
            KeyPressed(key) if key == self.key => {
                self.is_holding = true;
            },
            KeyReleased(key) if key == self.key => {
                self.is_holding = false;
                self.cur_time = 0.0;
            }
            _ => {}
        }
    }
}
