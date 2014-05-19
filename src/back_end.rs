
use piston::event::Event;

pub trait BackEnd {
    fn add_observer(&mut self, ob: Box<Observer>) -> uint;

    fn update(&mut self, _dt: f64) {}
    fn on_event(&mut self, _e: Event) {}
}

pub trait Observer {
    fn can_trigger(&self) -> bool { false }
    fn trigger(&mut self) {}
    fn after_triggered(&mut self) {}

    // one way to optimize is to support following method for back-end to
    // query, so it will not call `update` on the observer which don't need.
    //fn need_to_update(&self) -> bool { false; }
    fn update(&mut self, _dt: f64) {}

    fn on_event(&mut self, _e: Event) {}
}
