use macroquad::prelude::*;
use snake::*;

/// Time to wait between each tick, in seconds.
const TICK_TIME: f32 = 0.3;

#[macroquad::main("Snake")]
async fn main() {
    // Move direction starts at 1 on the X axis, i.e going right.
    let mut move_direction = Vec2::X;

    // Keeps track of how much time is left till the next tick (when the snake moves).
    let mut timer: f32 = 0.0;

    // Initialise position to the center of the screen.
    let middle = Vec2::new(
        (screen_width() / 2.0) - (SQUARE_SIZE / 2.0),
        (screen_height() / 2.0) - (SQUARE_SIZE / 2.0),
    );

    let mut snake = Snake::new(middle);

    loop {
        clear_background(BACKGROUND);

        // Set the move direction if a new input occured.
        if let Some(new_dir) = get_last_key_pressed().and_then(key_to_direction) {
            move_direction = new_dir;
        }

        timer += get_frame_time();
        // Move the snake if a tick has passed.
        if timer > TICK_TIME {
            timer = 0.0;
            snake.advance(move_direction);
        }

        snake.draw();

        next_frame().await
    }
}
