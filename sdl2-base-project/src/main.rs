#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;

use std::env;
use log::Level;

use sdl2::render::{
    WindowCanvas
};
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

#[allow(unused_imports)]
use sdl2::gfx::primitives::DrawRenderer;

mod rotatingwave;
mod pacman;

use crate::rotatingwave::core::{
    Application,
    ApplicationOptions,
};

use crate::rotatingwave::color::NamedColours;
use crate::rotatingwave::paint::{
    Drawing,
    Painter,
    MyWindow,
    Drawable
};

use crate::pacman::scene::Container;

#[derive(Clone)]
pub struct Tile {
    pub x: i32, pub y: i32, pub width: u32, pub height: u32
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            x: 0, y: 0, width: 30, height: 30
        }
    }

    pub fn set_x(&mut self, x: i32) -> &Self {
        self.x = x;
        self
    }
}

impl Drawable for Tile {
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }
    fn draw(&self) -> Surface {
        let s = Surface::new(self.width, self.height, PixelFormatEnum::RGBA8888).unwrap();
        let mut c = s.into_canvas().unwrap();
        c.set_draw_color(NamedColours::CYAN);
        c.clear();
        // c.set_draw_color(NamedColours::CYAN);
        // c.fill_rect(rect: R)
        c.into_surface()
    }
}


struct MyModel;

fn main() {
    env_logger::init();

    let current_dir = match env::current_dir() {
        Ok(dir) => {
            info!("Running from {:?}", dir);
            dir
        },
        Err(msg) => panic!(format!("Could not find current directory. {}", msg))
    };
    let options = ApplicationOptions {
        width: Some(1000), height: Some(800),
        fullscreen: None,
        resource_folder: Some(current_dir.join("resources")),
    };

    let mut app = Application::new(MyModel{});
    app.init("Application", options);

    app.start(Some(update));
}


fn update(_model: &mut MyModel, painter: &mut Painter) {
    painter.canvas.set_draw_color(NamedColours::BLACK);
    painter.canvas.clear();

    let tile = Tile::new();
    let mut scene = Container::new(800, 200);

    for i in 0..10 {
        let mut a = tile.clone();
        a.set_x(i * (10 + a.width as i32));
        scene.push(a);
    }

    painter.copy_to_window(&scene.draw(), 10, 10);

    // painter.canvas.aa_circle(300, 300, 30, NamedColours::LIGHT_WALNUT).unwrap();
    // painter.canvas.circle(500, 300, 30, NamedColours::MATTE_FUCHSIA).unwrap();
    // painter.text("test TEST Thinnest", 24, 50, 50, Some("font_thinnest"));
    // painter.text("test TEST Thinner", 24, 50, 100, Some("font_thinner"));
    // painter.text("test TEST Thin", 24, 50, 150, Some("font_thin"));
    // painter.text("test TEST Normal", 24, 50, 200, None);
    // painter.text("test TEST Bold", 24, 50, 250, Some("font_bold"));
    // painter.text("test TEST Bolder", 24, 50, 300, Some("font_bolder"));
    // painter.text("test TEST Boldest", 24, 50, 350, Some("font_boldest"));
    // painter.text("test TEST Black", 24, 50, 400, Some("font_black"));

    // Drawing::draw(&mut painter.canvas);
    // let myw = MyWindow::new(200, 100);
    // myw.copy_to_window(&mut painter.canvas, 10, 10);
    // painter.copy_to_window(&myw.draw(), 10, 10);
}
