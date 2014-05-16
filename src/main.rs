
#![feature(globs)]

extern crate graphics;
extern crate piston;

use piston::*;

mod test_mouse_move_event_app;

type GameWindowBackEnd = GameWindowGLFW;

fn main() {
    let mut game_window = GameWindow::new(
        GameWindowSettings::new (
            "Piston-Lab".to_owned(),
            [300, 300],
            false,
            true,
            [1.0, 1.0, 1.0, 1.0]
        )
    );

    let mut asset_store = AssetStore::empty();

    let mut app = test_mouse_move_event_app::App::new();

    app.run(&mut game_window, &mut asset_store);
}

