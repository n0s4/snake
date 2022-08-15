use macroquad::prelude::*;

const BACKGROUND: Color = DARKGREEN;
const SNAKE_COLOUR: Color = GREEN;

/// Size of a square on the game "grid".
const SQUARE_SIZE: f32 = 50.0;

/// Time to wait between each tick, in seconds.
const TICK_TIME: f32 = 0.3;

#[macroquad::main("Snake")]
async fn main() {
    // Move direction starts at 1 on the X axis, i.e going right.
    let mut move_direction = Vec2::X;

    // Initialise position to the center of the screen.
    let mut pos = Vec2::new(
        (screen_width() / 2.0) - (SQUARE_SIZE / 2.0),
        (screen_height() / 2.0) - (SQUARE_SIZE / 2.0),
    );

    // Keeps track of how much time is left till the next tick (when the snake moves).
    let mut timer: f32 = 0.0;

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
            pos += move_direction * SQUARE_SIZE;
        }

        draw_rectangle(pos.x, pos.y, SQUARE_SIZE, SQUARE_SIZE, SNAKE_COLOUR);

        next_frame().await
    }
}

/// Convert input into a direction as a Vec2.
/// ```
/// 'h' => [-1, 0] // left
/// 'j' => [0, -1] // down
/// 'k' => [0, 1] // up
/// 'l' => [1, 0] // right
/// ```
fn key_to_direction(key: KeyCode) -> Option<Vec2> {
    // Supports hjkl, arrow keys, and wasd.
    match key {
        KeyCode::H | KeyCode::Left | KeyCode::A => Some(-Vec2::X), // left
        KeyCode::J | KeyCode::Down | KeyCode::S => Some(Vec2::Y),  // down
        KeyCode::K | KeyCode::Up | KeyCode::W => Some(-Vec2::Y),   // up
        KeyCode::L | KeyCode::Right | KeyCode::D => Some(Vec2::X), // right
        _ => None,
    }
}
