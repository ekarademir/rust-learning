//! # `core`
//! Core facilities for the engine. It contains context initiation, window creation
//! canvas creation etc. It also tries to use OpenGL context.
extern crate gl;

use std::time::Duration;
use std::path::PathBuf;

use sdl2::{
    Sdl,
    VideoSubsystem
};
use sdl2::render::{
    CanvasBuilder,
    WindowCanvas
};
use sdl2::video::WindowBuilder;
use sdl2::ttf::Sdl2TtfContext;

#[allow(unused_imports)]
use sdl2::gfx::primitives::DrawRenderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::rotatingwave::color::NamedColours;
use crate::rotatingwave::paint::Painter;

/// Finds the driver index of the `opengl` driver. The index is then injected into
/// the canvas if there is one.
pub fn find_sdl_opengl_driver_index() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

/// Function that is called at each cycle of the application loop.
pub type UpdateFn<M> = fn(&mut M, &mut Painter);

/// State container for the application. It contains references to various aspects
/// of SDL.
pub struct Application<M>{
    pub model: M,
    pub painter: Option<Painter>,
    pub sdl_context: Sdl,
    pub video_subsystem: Option<VideoSubsystem>,
}

/// Holds options to use while creating the application window.
pub struct ApplicationOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fullscreen: Option<bool>,
    pub resource_folder: Option<PathBuf>,
}

impl<M> Application<M> {
    /// Initiates SDL2 context and also creates a new `Application` instance.
    /// `model` holds a state that is passed to the updating function at each
    /// application loop iteration.
    pub fn new(model: M) -> Application<M> {
        Application {
            sdl_context: sdl2::init().expect("Could not initiate sdl2 context"),
            video_subsystem: None,
            painter: None,
            model,
        }
    }

    /// Creates WindowCanvas and tries to switch to OpenGL context
    /// if there is an opengl driver available.
    pub fn init(&mut self, title: &str, options: ApplicationOptions) -> &mut Self {
        self.video_subsystem = self.sdl_context.video().ok();
        let mut window_builder: WindowBuilder = match (options.width, options.height) {
            (Some(width), Some(height)) => {
                self.video_subsystem.as_ref().unwrap().window(title, width, height)
            },
            _ => panic!("Both width and height should be provided")
        };

        match options.fullscreen {
            Some(true) => {
                window_builder.fullscreen();
            },
            _ => ()
        }

        let window = match window_builder.opengl().build() {
            Ok(w) => {
                info!("Built window");
                w
            },
            Err(msg) => panic!(format!("Could not create the window, {}", msg)),
        };

        let mut canvas_builder: CanvasBuilder = window.into_canvas();
        match find_sdl_opengl_driver_index() {
            Some(driver_index) => {
                canvas_builder = canvas_builder.index(driver_index);
                self.painter = Some(Painter::new(
                    canvas_builder.build().ok().unwrap(),
                    sdl2::ttf::init().ok().unwrap(),
                    options.resource_folder.unwrap()
                ));

                gl::load_with(
                    |name| self.video_subsystem.as_ref().unwrap()
                                .gl_get_proc_address(name) as *const _
                );

                match self.painter.as_mut().unwrap().canvas.window()
                    .gl_set_context_to_current() {
                        Ok(_) => info!("Switched to OpenGL context"),
                        Err(msg) => panic!(format!("Could not switch to OpenGL context, {}", msg)),
                }
            },
            _ => {
                self.painter = Some(Painter::new(
                    canvas_builder.build().ok().unwrap(),
                    sdl2::ttf::init().ok().unwrap(),
                    options.resource_folder.unwrap()
                ));
            }
        }
        self
    }

    /// Inititates application loop, until user presses escape or quits the app.
    /// At each iteration it calls `maybe_update` function with the provided `model`
    /// at while creating the app.
    pub fn start(&mut self, maybe_update: Option<UpdateFn<M>>) -> &mut Self {
        // Init event pump
        let mut event_pump = match self.sdl_context.event_pump() {
            Ok(p) => p,
            Err(msg) => panic!(format!("Could not inititate event pump, {}", msg)),
        };

        // let mut canvas = self.canvas.as_mut().unwrap();
        let mut p = self.painter.as_mut().unwrap();
        info!("Initiating application loop");
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => ()
                }
            }

            match maybe_update {
                Some(update) => update(&mut self.model, &mut p),
                None => ()
            }

            p.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
        }

        self
    }
}
