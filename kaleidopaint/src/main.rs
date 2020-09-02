use std::env;
use std::path::PathBuf;

use nannou::prelude::*;

use crate::kaleidopaint::Model;

// nannou related stuff
fn model(app: &App) -> Model {
    let current_dir = env::current_dir()
            .expect("Could not determine current folder");
    println!("Running from {:?}", current_dir);
    let default_font = current_dir.join("assets").join("fonts").join("leaguemono4.ttf");

    app.set_loop_mode(LoopMode::wait(3));
    app.new_window()
        .with_maximized(true)
        .event(event)
        .view(view)
        .build()
        .expect("Could not create the window");
    Model::new(&app, default_font)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.draw_ui();
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    // println!("{:?}", event);
    match event {
        MouseMoved(coords) => model.cursor_pos = coords,
        _ => ()
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let mut draw = app.draw();

    draw.background().color(WHITE);

    draw.to_frame(&app, &frame).expect("Could not paint the frame");

    model.ui.draw_to_frame(&app, &frame).expect("Could not paint UI");

    frame
}

fn exit(app: &App, model: Model) {
    println!("Bye!");
}

fn main() {
    nannou::app(model)
        .update(update)
        .exit(exit)
        .run();
}


// kaleidopaint
mod kaleidopaint {
    use std::path::PathBuf;
    // use std::f64::consts::PI;
    use std::f32::consts::PI;

    use nannou::app::App;
    use nannou::geom;
    use nannou::color::Rgb;
    use nannou::draw::Draw;
    use nannou::ui::prelude::*;
    use nannou::math::prelude::*;

    use cgmath::{Vector2, Matrix2};

    pub struct AppDefaults;
    impl AppDefaults {
        pub const CURSOR_SIZE: f32 = 15.0;
        pub const CURSOR_MIN: f32 = 5.0;
        pub const CURSOR_MAX: f32 = 70.0;

        pub const REPETITION: f32 = 6.0;
        pub const REPETITION_MIN: f32 = 1.0;
        pub const REPETITION_MAX: f32 = 20.0;

        // UI related no need to be public
        const FONT_SIZE: u32 = 15;
        const SLIDER_WIDTH: f64 = 200.0;
        const SLIDER_HEIGHT: f64 = 30.0;
        const WIDGET_BG_COLOR: Colour = ColourPalette::LIGHT_WALNUT;
        const WIDGET_FG_COLOR: Colour = ColourPalette::LIGHT_PISTACIO;

        const MARGIN: f64 = 5.0;
    }

    pub struct Colour {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl Colour {
        pub fn to_rgb(&self) -> Rgb {
            Rgb::new_u8(self.red, self.green, self.blue)
        }

        pub fn to_conrod_color(&self) -> color::Color {
            // Rgb::new_u8(self.red, self.green, self.blue)
            color::Color::Rgba(
                self.red_as_f32(),
                self.green_as_f32(),
                self.blue_as_f32(),
                1.0,
            )
        }

        fn as_f32(val: u8) -> f32 { (val as f32) / 255.0 }
        pub fn red_as_f32(&self) -> f32 { Colour::as_f32(self.red) }
        pub fn green_as_f32(&self) -> f32 { Colour::as_f32(self.green) }
        pub fn blue_as_f32(&self) -> f32 { Colour::as_f32(self.blue) }

    }


    pub struct ColourPalette;
    impl ColourPalette {
        pub const DARK_FUCHSIA: Colour = Colour {red: 74, green: 65, blue: 95};
        pub const MATTE_FUCHSIA: Colour = Colour {red: 163, green: 103, blue: 135};
        pub const MATTE_PISTACIO: Colour = Colour {red: 154, green: 202, blue: 152};
        pub const LIGHT_WALNUT: Colour = Colour {red: 177, green: 171, blue: 118};
        pub const LIGHT_PISTACIO: Colour = Colour {red: 222, green: 240, blue: 177};
    }

    pub struct Ids {
        cursor_size_id: widget::Id,
        repetition_id: widget::Id,
        cursor_ids: Vec<widget::Id>,
    }

    pub struct Model {
        pub cursor_pos: geom::Vector2,
        pub cursor_size: f32,
        pub repetition: usize,
        pub ui: Ui,
        pub ids: Ids,
    }

    impl Model {
        pub fn new(app: &App, default_font: PathBuf) -> Model {
            let mut ui = app.new_ui().default_font_path(default_font).build().expect("Could not build UI");

            // Only generate enough ids so that it can fill max number of cursors
            // These will be addressed by index. Id generation is only done once.
            let mut cursor_ids = Vec::new();
            for i in 0..AppDefaults::REPETITION_MAX as usize {
                cursor_ids.push(ui.generate_widget_id());
            }

            let ids = Ids {
                cursor_size_id: ui.generate_widget_id(),
                repetition_id: ui.generate_widget_id(),
                cursor_ids: cursor_ids,
            };

            Model {
                cursor_pos: geom::Vector2::new(0.0, 0.0),
                cursor_size: AppDefaults::CURSOR_SIZE,
                repetition: AppDefaults::REPETITION as usize,
                ids,
                ui,
            }
        }

        pub fn draw_ui(&mut self) {
            let ui = &mut self.ui.set_widgets();

            // Apparently we redraw the widgets at every update
            fn make_slider(value: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
                widget::Slider::new(value, min, max)
                    .w_h(AppDefaults::SLIDER_WIDTH, AppDefaults::SLIDER_HEIGHT)
                    .label_font_size(AppDefaults::FONT_SIZE)
                    .label_rgb(
                        AppDefaults::WIDGET_FG_COLOR.red_as_f32(),
                        AppDefaults::WIDGET_FG_COLOR.green_as_f32(),
                        AppDefaults::WIDGET_FG_COLOR.blue_as_f32()
                    )
                    .rgb(
                        AppDefaults::WIDGET_BG_COLOR.red_as_f32(),
                        AppDefaults::WIDGET_BG_COLOR.green_as_f32(),
                        AppDefaults::WIDGET_BG_COLOR.blue_as_f32()
                    )
                    .border(0.0)
            }

            // cursors
            // draw cursors first so that they are below other controls
            // otherwise events are not captured by other controls.
            // i.e. no interaction.
            let cursor_angle = self.cursor_pos.angle();
            let dtheta = 2.0 * PI / (self.repetition as f32);
            let sin_dtheta = dtheta.sin();
            let cos_dtheta = dtheta.cos();

            let rot = Matrix2::new(
                cos_dtheta,
                sin_dtheta,
                -1.0 * sin_dtheta,
                cos_dtheta
            );

            let mut current_cursor = Vector2::new(self.cursor_pos.x, self.cursor_pos.y);
            for i in 0..self.repetition {
                widget::Circle::fill_with(self.cursor_size as f64, ColourPalette::DARK_FUCHSIA.to_conrod_color())
                    .x_y(current_cursor.x as f64, current_cursor.y as f64)
                    .set(self.ids.cursor_ids[i], ui);

                current_cursor = rot * current_cursor;
            }

            // cursor size
            for value in make_slider(self.cursor_size, AppDefaults::CURSOR_MIN, AppDefaults::CURSOR_MAX)
                .top_left_with_margin(AppDefaults::MARGIN)
                .label("size")
                .set(self.ids.cursor_size_id, ui)
            {
                self.cursor_size = value;
            }

            // repetition
            for value in make_slider(self.repetition as f32, AppDefaults::REPETITION_MIN, AppDefaults::REPETITION_MAX)
                .right(AppDefaults::MARGIN)
                .label("repetition")
                .set(self.ids.repetition_id, ui)
            {
                self.repetition = value as usize;
            }
        }

        pub fn draw_cursors(&self) {
            let cursor_angle = self.cursor_pos.angle();
            let dtheta = 2.0 * PI / (self.repetition as f32);
            let sin_dtheta = dtheta.sin();
            let cos_dtheta = dtheta.cos();

            let rot = Matrix2::new(
                cos_dtheta,
                sin_dtheta,
                -1.0 * sin_dtheta,
                cos_dtheta
            );

            let mut current_cursor = Vector2::new(self.cursor_pos.x, self.cursor_pos.y);
            for i in 0..self.repetition {
                // self.draw_cursor(&draw, geom::Vector2::new(current_cursor.x, current_cursor.y));

                widget::Circle::fill_with(self.cursor_size as f64, ColourPalette::DARK_FUCHSIA.to_conrod_color())
                    .x_y(current_cursor.x as f64, current_cursor.y as f64);

                current_cursor = rot * current_cursor;
            }

        }
    }
}
