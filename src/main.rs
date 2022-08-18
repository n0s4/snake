use ::rand::{thread_rng, Rng};
use macroquad::{audio, prelude::*};
use snake::*;

/// Time to wait between each tick, in seconds.
const TICK_TIME: f32 = 0.1;

#[macroquad::main("Snake")]
async fn main() {
    set_pc_assets_folder("assets");

    let eat_apple_sound = audio::load_sound("eat_apple.wav")
        .await
        .expect("find sound assets/eat_apple.wav");

    // Move direction starts at 1 on the X axis, i.e going right.
    let mut move_direction = Vec2::X;

    // Keeps track of how much time is left till the next tick (when the snake moves).
    let mut timer: f32 = 0.0;

    let mut score: u16 = 0;

    // The window manager may resize after the first frame, so we skip a frame before capturing the size.
    next_frame().await;
    info!("Window size is {}x{}", screen_width(), screen_height());

    let middle = Vec2::new(
        ((screen_width() / SQUARE_SIZE) * 0.5).floor() * SQUARE_SIZE,
        ((screen_height() / SQUARE_SIZE) * 0.5).floor() * SQUARE_SIZE,
    );

    let mut snake = Snake::new(Vec2::new(middle.x - (SQUARE_SIZE * 3.), middle.y));

    // The apples position.
    let mut apple_pos = Vec2::new(middle.x + (SQUARE_SIZE * 3.), middle.y);

    loop {
        clear_background(BACKGROUND);

        // Set the move direction if a new input occured.
        if let Some(input) = get_char_pressed() {
            move_direction = match input.to_ascii_lowercase() {
                'w' | 'k' => -Vec2::Y, // up
                'a' | 'h' => -Vec2::X, // left
                's' | 'j' => Vec2::Y,  // down
                'd' | 'l' => Vec2::X,  // right
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
                snake.grow();
                score += 1;
                // FIXME: audio isn't working for me??
                audio::play_sound_once(eat_apple_sound);

                // move the apple
                let mut rng = thread_rng();

                let gridx = rng.gen_range(0..(screen_width() / SQUARE_SIZE) as u16);
                let gridy = rng.gen_range(0..(screen_height() / SQUARE_SIZE) as u16);

                apple_pos = Vec2::new(gridx as f32 * SQUARE_SIZE, gridy as f32 * SQUARE_SIZE);
            }
        }

        draw_square_at(&apple_pos, APPLE_COLOUR);
        snake.draw();
        draw_text(&format!("Score: {score}"), 10.0, 30.0, 40.0, WHITE);

        next_frame().await
    }
}
