
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
        self.image = Some(Texture::from_path(&asset_store.path("char01.png").unwrap()).unwrap());
    }

    fn render(&self, _ext_dt: f64, c: &Context, gl: &mut Gl) {
        c.image(self.image.get_ref()).draw(gl);
    }
}

