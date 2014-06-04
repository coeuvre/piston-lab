
#![feature(globs)]

extern crate graphics;
extern crate piston;

use piston::*;

mod test_image_app;
mod test_sub_image_app;
mod test_mouse_move_event_app;

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings {
            title: "Piston-Lab".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 0.0]
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");

    //let mut app = test_mouse_move_event_app::App::new();
    //let mut app = test_image_app::App::new();
    let mut app = test_sub_image_app::App::new();

    app.run(&mut game_window, &mut asset_store);
}

