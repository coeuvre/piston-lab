
use piston::keyboard;

pub trait AddHolding<'a, T> {
    fn holding(&'a self, key: keyboard::Key) -> T;
}

