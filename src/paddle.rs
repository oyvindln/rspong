use app::Direction;
use piston_window::Key;
use graphics::math::Vec2d;

pub struct Paddle {
    pub position: Vec2d,
    pub height: f64,
}

/// Updates paddle position according to the requested direction.
pub fn update_paddle(paddle: &mut Paddle, direction_pressed: Direction, delta: f64) {
    paddle.position[1] += paddle_translate(direction_pressed, delta)
}

/// Calculate new paddle position according to direction and delta.
fn paddle_translate(direction: Direction, delta: f64) -> f64 {
    const PADDLE_VELOCITY: f64 = 10.0;
    match direction {
        Direction::Up => -delta * PADDLE_VELOCITY,
        Direction::Down => delta * PADDLE_VELOCITY,
        _ => 0.0,
    }
}

/// Returns what direction the paddle should move according to a key press event.
pub fn paddle_direction_from_key_press(current_dir: Direction, key: Key) -> Direction {
    match current_dir {
        Direction::None => {
            match key {
                Key::Up => Direction::Up,
                Key::Down => Direction::Down,
                _ => Direction::None,
            }
        }
        Direction::Up => {
            match key {
                Key::Down => Direction::None,
                _ => Direction::Up,
            }
        }
        Direction::Down => {
            match key {
                Key::Up => Direction::None,
                _ => Direction::Down,
            }
        }
    }
}
/// Returns what direction the paddle should move according to a key release event.
pub fn paddle_direction_from_key_release(current_dir: Direction, key: Key) -> Direction {
    match current_dir {
        Direction::None => Direction::None,
        Direction::Up => {
            match key {
                Key::Up => Direction::None,
                _ => Direction::Up,
            }
        }
        Direction::Down => {
            match key {
                Key::Down => Direction::None,
                _ => Direction::Down,
            }
        }
    }
}
