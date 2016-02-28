extern crate piston_window;
extern crate opengl_graphics;
extern crate graphics;
// extern crate piston2d-graphics;

use piston_window::*;

mod ball;
mod paddle;
mod app;
mod ai;

// enum ButtonStatus {
// Press,
// Release,
// }

fn main() {
    let opengl = OpenGL::V3_1;

    let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
                                   .exit_on_esc(true)
                                   .opengl(opengl)
                                   .build()
                                   .unwrap();

    let mut app = app::App::new(opengl,
                                window.size().width as f64,
                                window.size().height as f64);

    for e in window {

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(i) = e.press_args() {
            match i {
                Button::Keyboard(Key::Escape) => break,
                Button::Keyboard(key) => app.key_pressed(key),
                _ => (),
            }
        }

        if let Some(i) = e.release_args() {
            match i {
                Button::Keyboard(key) => app.key_released(key),
                _ => (),
            }
        }
    }
}
