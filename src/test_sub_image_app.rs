
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
        self.image = Some(asset_store.load_image("digits.png").unwrap());
    }

    fn render(&self, c: &Context, gl: &mut Gl) {
        let mut image = self.image.unwrap();
        image.source_rect[2] = 20;
        image.source_rect[3] = 26;
        c.image(image).draw(gl);
    }
}

