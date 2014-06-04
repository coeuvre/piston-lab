
use graphics::*;
use piston::*;

pub struct App {
    image: Option<Texture>,
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
        self.image = Some(Texture::from_path(&asset_store.path("digits.png").unwrap()).unwrap());
    }

    fn render(&self, _ext_dt: f64, c: &Context, gl: &mut Gl) {
        c.rect(0.0, 0.0, 20.0, 26.0).image(self.image.get_ref()).src_rect(0, 0, 20, 26).draw(gl);
    }
}

