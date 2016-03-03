extern crate rand;

use graphics::math::Vec2d;
use graphics::types::Rectangle;
use rand::distributions::{IndependentSample, Range};

use paddle::*;

/// Finds the center of a rectangle.
fn get_center(rectangle: Rectangle) -> Vec2d {
    [rectangle[0] + 0.5 * (rectangle[2] - rectangle[0]),
     rectangle[1] + 0.5 * (rectangle[3] - rectangle[1])]
}

/// Test if the ball is at an y value that is the same as the paddle covers.
fn paddle_ball_intersect(new_pos: Vec2d, radius: f64, paddle: &Paddle) -> bool {
    new_pos[1] + radius > paddle.position[1] && new_pos[1] < paddle.position[1] + paddle.height
}

/// Generate a new random number for the ball's respawn velocity
fn get_random_velocity() -> Vec2d {
    // Create random generator
    // We should probably store somewhere this eventually
    let mut rng = rand::thread_rng();
    let between = rand::distributions::Range::new(-50.0, 50.0);
    [between.ind_sample(&mut rng), between.ind_sample(&mut rng)]
}

#[derive(Clone, Default)]
pub struct Ball {
    pub position: Vec2d,
    pub velocity: Vec2d,
    pub radius: f64,
}

/// Enum to signal to the application the response to ball movement.
pub enum UpdateData {
    PointLeft,
    PointRight,
    None,
}

impl Ball {
    /// Reset the ball in the middle of `rectangle` with a random velocity
    fn reset(&mut self, rectangle: Rectangle) {
        self.position = get_center(rectangle);
        self.velocity = get_random_velocity();
    }
}

/// Move the ball and reset it if it has left the play area.
///
/// Returns UpdateData::PointLeft or UpdateData::PointRight if the ball exits
/// the play area, UpdateData::None otherwise.
pub fn update_ball(ball: &mut Ball,
                   left_paddle: &Paddle,
                   right_paddle: &Paddle,
                   rectangle: Rectangle,
                   delta: f64)
                   -> UpdateData {
    let vel = [ball.velocity[0] * delta, ball.velocity[1] * delta];

    let new_pos = [ball.position[0] + vel[0], ball.position[1] + vel[1]];

    let diameter = ball.radius * 2.0;

    // Check collision with the left edge
    if new_pos[0] < rectangle[0] {
        if paddle_ball_intersect(new_pos, ball.radius, left_paddle) {
            // The Ball is outside
            ball.position[0] = -new_pos[0];
            ball.velocity[0] = -ball.velocity[0];
            return UpdateData::None;
        }
        ball.reset(rectangle);
        return UpdateData::PointRight;
        // Right edge
    } else if diameter + new_pos[0] > rectangle[2] {
        if paddle_ball_intersect(new_pos, ball.radius, right_paddle) {
            ball.position[0] = rectangle[2] - vel[0] - diameter;
            ball.velocity[0] = -ball.velocity[0];
            return UpdateData::None;
        }
        ball.reset(rectangle);
        return UpdateData::PointLeft;
    } else {
        ball.position[0] = new_pos[0]
    }

    // Check collision with top edge
    if new_pos[1] < rectangle[1] {
        ball.position[1] = -new_pos[1];
        ball.velocity[1] = -ball.velocity[1];
        // Bottom edge
    } else if diameter + new_pos[1] > rectangle[3] {
        ball.position[1] = rectangle[3] - vel[1] - diameter;
        ball.velocity[1] = -ball.velocity[1];
    } else {
        ball.position[1] = new_pos[1];
    }
    UpdateData::None

}

#[test]
fn test_paddle_ball_intersect() {
    let paddle = Paddle {
        position: [1.0, 0.0],
        height: 40.0,
    };
    assert!(paddle_ball_intersect([1.0, 5.0], 10.0, &paddle));
    assert!(paddle_ball_intersect([1.0, 10.0], 10.0, &paddle));
    assert!(paddle_ball_intersect([1.0, 30.0], 10.0, &paddle));
    assert!(!paddle_ball_intersect([1.0, 50.0], 10.0, &paddle));

}
