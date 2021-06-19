extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston::{mouse::MouseButton, Button::Mouse, ButtonState};

const BACKGROUND: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
const HEIGHT: f64 = 500.0;
const WIDTH: f64 = 500.0;

struct Board {
    gl: GlGraphics,
    brush_pixels: Vec<Pixel>,
}

impl Board {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BACKGROUND, gl);
        });

        for pix in self.brush_pixels.iter() {
            pix.render(&mut self.gl, args);
        }
    }
}

struct Pixel {
    color: [f32; 4],
    x: f64,
    y: f64,
}

impl Pixel {
    fn new(color: [f32; 4], x: f64, y: f64) -> Self {
        Pixel { color, x, y }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let size = 10.;

        let square = graphics::rectangle::square(self.x, self.y, size);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(self.color, square, transform, gl);
        });
    }
}

struct MouseLocation {
    x: f64,
    y: f64,
}

enum Mode {
    Draw,
    Erase,
}

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Drawing App", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let opengl = OpenGL::V2_1;

    let mut pixel_color = [0.0, 0.0, 0.0, 1.0];

    let pixels = vec![
        Pixel::new([0.0, 0.0, 0.0, 1.0], 0., 0.),
        Pixel::new([255.0, 0.0, 0.0, 1.0], 10., 0.),
        Pixel::new([0.0, 255.0, 0.0, 1.0], 0., 10.),
        Pixel::new([0.0, 0.0, 255.0, 1.0], 10., 10.),
    ];

    let mut board = Board {
        gl: GlGraphics::new(opengl),
        brush_pixels: pixels,
    };

    let mut events = Events::new(EventSettings::new()).ups(0);
    let mut mouse_location = MouseLocation { x: 0., y: 0. };
    let mut mouse_down = false;
    let mut mode = Mode::Draw;

    while let Some(e) = events.next(&mut window) {
        e.mouse_cursor(|pos| {
            mouse_location = MouseLocation {
                x: pos[0],
                y: pos[1],
            };
        });

        if let Some(args) = e.render_args() {
            board.render(&args);
        }

        if let Some(args) = e.button_args() {
            if let Mouse(btn) = args.button {
                match btn {
                    MouseButton::Left => {
                        mode = Mode::Draw;
                        match args.state {
                            ButtonState::Press => {
                                mouse_down = true;
                            }
                            ButtonState::Release => {
                                mouse_down = false;
                            }
                        }
                    }

                    MouseButton::Right => {
                        mode = Mode::Erase;
                        match args.state {
                            ButtonState::Press => {
                                mouse_down = true;
                            }
                            ButtonState::Release => {
                                mouse_down = false;
                            }
                        }
                    }

                    _ => {}
                }
            }
        }

        if mouse_down {
            match mode {
                Mode::Draw => pixel_color = [0.0, 0.0, 0.0, 1.0],
                Mode::Erase => pixel_color = [255.0, 255.0, 255.0, 1.0],
            }

            board
                .brush_pixels
                .push(Pixel::new(pixel_color, mouse_location.x, mouse_location.y));
        }
    }
}
