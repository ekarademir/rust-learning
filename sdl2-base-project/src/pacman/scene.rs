use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use crate::rotatingwave::paint::Drawable;

pub struct Container<T> {
    pub width: u32,
    pub height: u32,
    pub contents: Vec<T>,
}

impl <T: Drawable> Container <T> {
    pub fn new(width: u32, height: u32) -> Container<T> {
        Container {
            width, height, contents: Vec::new()
        }
    }

    pub fn push(&mut self, item: T) -> &Self {
        self.contents.push(item);
        self
    }
}

impl<T: Drawable> Drawable for Container<T> {
    fn draw(&self) -> Surface {
        let mut surface = Surface::new(self.width, self.height, PixelFormatEnum::RGBA8888).unwrap();
        for child in self.contents.iter() {
            let child_surface: Surface = child.draw();
            let r1 = child_surface.rect();
            let r2 = Rect::new(child.get_x(), child.get_y(), r1.width(), r1.height());
            child_surface.blit(r1, &mut surface, r2);
        }
        surface
    }
}
