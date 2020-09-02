//! # `paint`
//! Higher level stuff related to drawing on canvas.

use std::path::PathBuf;
use std::collections::HashMap;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{
    Canvas,
    Texture,
    TextureQuery,
    WindowCanvas,
};
use sdl2::surface::Surface;
use sdl2::ttf::Sdl2TtfContext;

use crate::rotatingwave::color::NamedColours;

/// Wrapper for Canvas created from the application window.
/// It extends the canvas functionality by adding ttf rendering.
pub struct Painter {
    pub canvas: WindowCanvas,
    pub ttf_context: Sdl2TtfContext,
    resource_folder: PathBuf,
}

impl Painter {
    fn get_resource(&self, resource: &str) -> PathBuf {
        let fonts_base = self.resource_folder.join("fonts");
        let mut resource_lookup: HashMap<String, PathBuf> = HashMap::new();
        resource_lookup.insert("font_thinnest".to_string(), fonts_base.join("leaguemono0.ttf"));
        resource_lookup.insert("font_thinner".to_string(), fonts_base.join("leaguemono1.ttf"));
        resource_lookup.insert("font_thin".to_string(), fonts_base.join("leaguemono2.ttf"));
        resource_lookup.insert("font_normal".to_string(), fonts_base.join("leaguemono3.ttf"));
        resource_lookup.insert("font_bold".to_string(), fonts_base.join("leaguemono4.ttf"));
        resource_lookup.insert("font_bolder".to_string(), fonts_base.join("leaguemono5.ttf"));
        resource_lookup.insert("font_boldest".to_string(), fonts_base.join("leaguemono6.ttf"));
        resource_lookup.insert("font_black".to_string(), fonts_base.join("leaguemono7.ttf"));

        match resource_lookup.get(&resource.to_string()) {
            Some(p) => p.to_path_buf(),
            None => panic!(format!("Can't find resource {}", resource))
        }
    }

    pub fn new(
        canvas: WindowCanvas,
        ttf_context: Sdl2TtfContext,
        resource_folder: PathBuf,
    ) -> Painter {
        Painter {
            canvas,
            ttf_context,
            resource_folder
        }
    }

    /// Write a text to the application window.
    pub fn text(&mut self, content: &str, size: u16, x: u32, y: u32, typeface: Option<&str>) {
        let font_path = match typeface {
            Some(name) => self.get_resource(name),
            None => self.get_resource("font_normal")
        };
        let texture_creator = self.canvas.texture_creator();
        let mut font = self.ttf_context.load_font(
            font_path,
            size
        ).unwrap();

        let surface = font.render(content)
            .blended(NamedColours::BLACK).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        let TextureQuery {width, height, ..} = texture.query();
        let target = Rect::new(x as i32, y as i32, width, height);

        self.canvas.copy(&texture, None, Some(target));
    }

    pub fn copy_to_window(&mut self, surface: &Surface, x: i32, y: i32) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery {width, height, ..} = texture.query();
        let target = Rect::new(x, y, width, height);

        self.canvas.copy(&texture, None, Some(target));
    }
}

pub trait Drawable {
    fn draw(&self) -> Surface;
    fn get_x(&self) -> i32 {
        0
    }
    fn get_y(&self) -> i32 {
        0
    }
}

///////////////////////////////////
/// Example code below
pub struct Drawing{}

impl Drawing {
    pub fn draw(targetCanvas: &mut WindowCanvas) {
        let surface = Surface::new(200, 200, PixelFormatEnum::RGBA8888).unwrap();
        let mut canvas = surface.into_canvas().unwrap();

        canvas.set_draw_color(NamedColours::CRIMSON);
        canvas.draw_rect(Rect::new(10, 10, 180, 180));

        let surface = canvas.into_surface();
        let texture_creator = targetCanvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery {width, height, ..} = texture.query();
        let target = Rect::new(50, 500, width, height);

        targetCanvas.copy(&texture, None, Some(target));
    }
}

pub struct MyWindow {
    width: u32,
    height: u32,
    margin: u32,
}

impl MyWindow {
    pub fn new(width: u32, height: u32) -> MyWindow {
        MyWindow {
            width,
            height,
            margin: 10,
        }
    }
}

impl Drawable for MyWindow {
    fn draw(&self) -> Surface {
        let surface = Surface::new(self.width, self.height, PixelFormatEnum::RGBA8888).unwrap();
        let mut canvas = surface.into_canvas().unwrap();
        canvas.set_draw_color(NamedColours::CRIMSON);
        canvas.draw_rect(Rect::new(
            self.margin as i32,
            self.margin as i32,
            self.width - self.margin * 2,
            self.height - self.margin * 2
        ));

        canvas.into_surface()
    }
}
