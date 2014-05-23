
use graphics::*;
use piston::*;

pub struct App {
    image: Option<Image>,
}

impl App {
    pub fn new() -> App {
        App {
            image: None,
        }
    }
}

impl Game for App {
    fn load(&mut self, asset_store: &mut AssetStore) {
        self.image = Some(asset_store.load_image("char01.png").unwrap());
    }

    fn render(&self, c: &Context, gl: &mut Gl) {
        c.image(self.image.unwrap()).draw(gl);
    }
}

