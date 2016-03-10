use paddle::Paddle;
use ball::Ball;

const AI_MOVEMENT_PER_SECOND: f64 = 5.0;

/// Calculate the movement of the ai paddle
///
/// Currently only moves towards the ball
fn ai_paddle_translation(paddle: &Paddle, ball: &Ball, delta: f64) -> f64 {
    if ball.position[1] + ball.radius < paddle.position[1] {
        -AI_MOVEMENT_PER_SECOND * delta
    } else if ball.position[1] + ball.radius > paddle.position[1] + paddle.height {
        AI_MOVEMENT_PER_SECOND * delta
    } else {
        0.0
    }
}

pub fn update_ai_paddle(paddle: &mut Paddle, ball: &Ball, delta: f64) {
    paddle.position[1] += ai_paddle_translation(paddle, ball, delta);
}

#[test]
fn test_ai_translation() {
    let ball = Ball {
        position: [5.0, 0.0],
        velocity: [0.0, 0.0],
        radius: 10.0,
    };
    let paddle = Paddle {
        position: [1.0, 60.0],
        height: 40.0,
    };

    assert!(ai_paddle_translation(&paddle, &ball, 100.0) < 0.0);

    let ball = Ball {
        position: [5.0, 60.0],
        velocity: [0.0, 0.0],
        radius: 10.0,
    };

    let paddle = Paddle {
        position: [1.0, 0.0],
        height: 40.0,
    };

    println!("Movement: {}", ai_paddle_translation(&paddle, &ball, 100.0));

    assert!(ai_paddle_translation(&paddle, &ball, 100.0) > 0.0);
}
