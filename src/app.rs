extern crate opengl_graphics;
extern crate graphics;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{RenderArgs, UpdateArgs, Key};
use graphics::types::Rectangle;
use std::path::Path;

use ball::*;
use paddle::*;
use ai::update_ai_paddle;

// The height of the paddles
const PADDLE_HEIGHT: f64 = 40.0;

/// Enum to describe the direction the paddle is currently heading
#[derive(Clone,Copy)]
pub enum Direction {
    Up,
    Down,
    None,
}

pub struct App {
    gl: GlGraphics,
    ball: Ball,
    rectangle: Rectangle,
    paddle: Paddle,
    right_paddle: Paddle,
    direction_pressed: Direction,
    left_points: u32,
    right_points: u32,
    character_cache: GlyphCache<'static>,
}

impl App {
    pub fn new(opengl: opengl_graphics::OpenGL, width: f64, height: f64) -> App {
        App {
            gl: GlGraphics::new(opengl),
            ball: Ball {
                position: [30.0, 300.0],
                velocity: [-50.0, -50.0],
                radius: 10.0,
            },
            rectangle: [0.0, 0.0, width, height],
            paddle: Paddle {
                position: [0.5, 10.0],
                height: PADDLE_HEIGHT,
            },
            right_paddle: Paddle {
                position: [width - 10.5, 30.0],
                height: PADDLE_HEIGHT,
            },
            direction_pressed: {
                Direction::None
            },
            left_points: 0,
            right_points: 0,
            character_cache: GlyphCache::new(Path::new("./Ubuntu-R.ttf")).unwrap(),
        }

    }

    /// Draw the scene
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 25.0);
        let paddle_render = [0.0, 0.0, 10.0, PADDLE_HEIGHT];
        let text = text::Text::new(30);
        let left_string = self.left_points.to_string();
        let right_string = self.right_points.to_string();

        let ball_pos = self.ball.position;
        let paddle_pos = self.paddle.position;
        let right_paddle_pos = self.right_paddle.position;
        let left_text_pos = [5.0, 30.0];
        let right_text_pos = [590.0, 30.0];
        let cc = &mut self.character_cache;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            // Draw ball
            let transform = c.transform.trans(ball_pos[0], ball_pos[1]);
            ellipse(RED, square, transform, gl);

            // Draw left paddle
            let transform = c.transform.trans(paddle_pos[0], paddle_pos[1]);
            rectangle(RED, paddle_render, transform, gl);

            // Draw right paddle
            let transform = c.transform.trans(right_paddle_pos[0], right_paddle_pos[1]);
            rectangle(RED, paddle_render, transform, gl);

            // Draw text
            let transform = c.transform.trans(left_text_pos[0], left_text_pos[1]);
            text.draw(&left_string, cc, default_draw_state(), transform, gl);

            let transform = c.transform.trans(right_text_pos[0], right_text_pos[1]);
            text.draw(&right_string, cc, default_draw_state(), transform, gl)
        });
    }

    /// Updates the ball and paddle
    pub fn update(&mut self, args: &UpdateArgs) {
        let data = update_ball(&mut self.ball,
                               &self.paddle,
                               &self.right_paddle,
                               self.rectangle,
                               args.dt * 10.0);
        match data {
            UpdateData::PointLeft => self.left_points += 1,
            UpdateData::PointRight => self.right_points += 1,
            UpdateData::None => (),
        }
        update_paddle(&mut self.paddle, self.direction_pressed, args.dt * 50.0);
        update_ai_paddle(&mut self.right_paddle, &self.ball, args.dt * 50.0);
    }

    pub fn key_pressed(&mut self, key: Key) {
        self.direction_pressed = paddle_direction_from_key_press(self.direction_pressed, key);
    }

    pub fn key_released(&mut self, key: Key) {
        self.direction_pressed = paddle_direction_from_key_release(self.direction_pressed, key);
    }
}
