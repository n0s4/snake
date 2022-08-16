use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use snake::*;

/// Time to wait between each tick, in seconds.
const TICK_TIME: f32 = 0.1;

#[macroquad::main("Snake")]
async fn main() {
    // Move direction starts at 1 on the X axis, i.e going right.
    let mut move_direction = Vec2::X;

    // Keeps track of how much time is left till the next tick (when the snake moves).
    let mut timer: f32 = 0.0;

    // The window manager may resize after the first frame, so we skip a frame before capturing the size.
    next_frame().await;
    info!("Window size is {}x{}", screen_width(), screen_height());

    let start_pos = Vec2::new(
        ((screen_width() / SQUARE_SIZE) * 0.5).floor() * SQUARE_SIZE - 3.0 * SQUARE_SIZE,
        ((screen_height() / SQUARE_SIZE) * 0.5).floor() * SQUARE_SIZE,
    );

    info!("Starting position is {}", start_pos);

    let mut snake = Snake::new(start_pos);

    // The apples position.
    let mut apple_pos = Vec2::new(start_pos.x + SQUARE_SIZE * 3.0, start_pos.y);

    loop {
        clear_background(BACKGROUND);

        // Set the move direction if a new input occured.
        if let Some(input) = get_char_pressed() {
            move_direction = match input {
                'w' | 'k' => -Vec2::Y,
                'a' | 'h' => -Vec2::X,
                's' | 'j' => Vec2::Y,
                'd' | 'l' => Vec2::X,
                // If it was any other key, don't change the direction.
                _ => move_direction,
            }
        }

        timer += get_frame_time();
        // Move the snake if a tick has passed.
        if timer > TICK_TIME {
            timer = 0.0;
            snake.advance(move_direction);

            if *snake.head() == apple_pos {
                info!("Apple eaten!");
                snake.grow();

                // Scary maths ahead.
                let mut rng = thread_rng();

                let gridx = rng.gen_range(0..(screen_width() / SQUARE_SIZE) as u32);
                let gridy = rng.gen_range(0..(screen_height() / SQUARE_SIZE) as u32);

                info!("New grid pos: {}, {}", gridx, gridy);

                apple_pos = Vec2::new(gridx as f32 * SQUARE_SIZE, gridy as f32 * SQUARE_SIZE);

                info!("New real pos: {}, {}\n", apple_pos.x, apple_pos.y);
            }
        }

        draw_square_at(&apple_pos, APPLE_COLOUR);
        snake.draw();

        next_frame().await
    }
}
