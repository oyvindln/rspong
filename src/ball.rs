use graphics::math::Vec2d;
use graphics::types::Rectangle;

//use boundingbox;
use paddle::*;

/*
fn normalize(vector: Vec2d) -> Vec2d {
    // TODO: use function from piston if possible.
    use graphics::math::*;
    let length = square_len(vector).sqrt();
    mul_scalar(vector, 1.0 / length)
}
 */

fn get_center(rectangle: Rectangle) -> Vec2d {
    [
        rectangle[0] + 0.5 * (rectangle[2] - rectangle[0]),
        rectangle[1] + 0.5 * (rectangle[3] - rectangle[1]),
    ]
}

fn paddle_ball_intersect(new_pos: Vec2d, radius: f64, paddle: &Paddle) -> bool {
    new_pos[1] + radius > paddle.position[1] &&
        new_pos[1] < paddle.position[1] + paddle.height
}

#[derive(Clone, Default)]
pub struct Ball {
    pub position: Vec2d,
    pub velocity: Vec2d,
    pub radius: f64,
}

pub enum UpdateData {
    PointLeft,
    PointRight,
    None
}

pub fn update_ball(
    ball: &mut Ball, left_paddle: &Paddle, right_paddle: &Paddle, rectangle: Rectangle, delta: f64)
    -> UpdateData {
    let vel = [ball.velocity[0] * delta, ball.velocity[1] * delta];

    let new_pos = [ball.position[0] + vel[0],
                   ball.position[1] + vel[1]];

    let diameter = ball.radius * 2.0;

    //left
    if new_pos[0] < rectangle[0] {
        if paddle_ball_intersect(new_pos, ball.radius, left_paddle) {
            ball.position[0] = -new_pos[0];
            ball.velocity[0] = -ball.velocity[0];
            return UpdateData::None;
           }
        ball.position = get_center(rectangle);
        return UpdateData::PointRight
        //right
    } else if diameter + new_pos[0] > rectangle[2] {
        if paddle_ball_intersect(new_pos, ball.radius, right_paddle) {
            ball.position[0] = rectangle[2] - vel[0] - diameter;
            ball.velocity[0] = -ball.velocity[0];
            return UpdateData::None;
        }
        ball.position = get_center(rectangle);
        return UpdateData::PointLeft
    } else {
        ball.position[0] = new_pos[0]
    }

    //top
    if new_pos[1] < rectangle[1] {
        ball.position[1] = -new_pos[1];
        ball.velocity[1] = -ball.velocity[1];
        //bottom
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
    assert!(paddle_ball_intersect([1.0, 5.0], &paddle));
    assert!(paddle_ball_intersect([1.0, 10.0], &paddle));
    assert!(paddle_ball_intersect([1.0, 30.0], &paddle));
    assert!(!paddle_ball_intersect([1.0, 50.0], &paddle));

}
