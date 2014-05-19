
#![feature(globs)]

extern crate graphics;
extern crate piston;

use piston::*;

// === For Event system =======================================================
pub use AddKeyboard = add_keyboard::AddKeyboard;
pub use AddLasting = add_lasting::AddLasting;
pub use AddPress = add_press::AddPress;
pub use AddPressing = add_pressing::AddPressing;

pub use Event = event::Event;
pub use KeyboardEvent = keyboard_event::KeyboardEvent;
pub use KeyboardPressingEvent = keyboard_pressing_event::KeyboardPressingEvent;
pub use KeyboardPressingLastingEvent = keyboard_pressing_lasting_event::KeyboardPressingLastingEvent;
pub use KeyboardPressEvent = keyboard_press_event::KeyboardPressEvent;

pub use Map = map::Map;

pub use BackEnd = back_end::BackEnd;
pub use Observer = back_end::Observer;
// ============================================================================

// === For Event system =======================================================
mod add_keyboard;
mod add_lasting;
mod add_press;
mod add_pressing;

mod event;
mod keyboard_event;
mod keyboard_pressing_event;
mod keyboard_pressing_lasting_event;
mod keyboard_press_event;

mod map;

mod back_end;
// ============================================================================

mod test_image_app;
mod test_mouse_move_event_app;
mod test_event_app;

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings::new (
            "Piston-Lab".to_owned(),
            [300, 300],
            false,
            true,
            [1.0, 1.0, 1.0, 1.0]
        )
    );

    let mut asset_store = AssetStore::from_folder("assets");

    //let mut app = test_mouse_move_event_app::App::new();
    //let mut app = test_image_app::App::new();
    let mut app = test_event_app::App::new();

    app.run(&mut game_window, &mut asset_store);
}

