
use piston::*;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }
}

impl Game for App {
    fn mouse_move(&mut self, x: f64, y:f64, _asset_store: &mut AssetStore) {
        println!("Mouse point at ({}, {})", x, y);
    }
}

