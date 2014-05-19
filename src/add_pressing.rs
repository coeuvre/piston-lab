
use piston::keyboard;

pub trait AddPressing<'a, T> {
    fn pressing(&'a self, key: keyboard::Key) -> T;
}

