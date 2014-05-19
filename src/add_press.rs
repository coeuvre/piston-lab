
use piston::keyboard;

pub trait AddPress<'a, T> {
    fn press(&'a self, key: keyboard::Key) -> T;
}
