use macroquad::prelude::*;

use std::{thread::sleep, time::Duration};

const SQUARE_SIZE: f32 = 50.0;
/// Time to wait between each frame.
const WAIT_TIME: Duration = Duration::from_millis(300);

#[macroquad::main("Snake")]
async fn main() {
    // Move direction starts at 1 on the X axis, i.e going right.
    let mut move_direction = Vec2::X;
    // Initialise position to the center of the screen.
    let mut pos = Vec2::new(
        (screen_width() / 2.0) - (SQUARE_SIZE / 2.0),
        (screen_height() / 2.0) - (SQUARE_SIZE / 2.0),
    );

    loop {
        clear_background(DARKGREEN);

        info!("position: {}", pos);
        info!("move direction: {}", move_direction);

        // Set the move direction if a new input occured.
        if let Some(new_dir) = get_last_key_pressed().and_then(key_to_direction) {
            move_direction = new_dir;
        }

        pos += move_direction * SQUARE_SIZE;

        draw_rectangle(
            pos.x,       // top left corner x
            pos.y,       // top left corner y
            SQUARE_SIZE, // width
            SQUARE_SIZE, // height
            GREEN,       // colour
        );

        // wait for a moment between each frame.
        sleep(WAIT_TIME);

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
    match key {
        KeyCode::H | KeyCode::Left | KeyCode::A => Some(-Vec2::X), // left
        KeyCode::J | KeyCode::Down | KeyCode::S => Some(Vec2::Y),  // down
        KeyCode::K | KeyCode::Up | KeyCode::W => Some(-Vec2::Y),   // up
        KeyCode::L | KeyCode::Right | KeyCode::D => Some(Vec2::X), // right
        _ => None,
    }
}
