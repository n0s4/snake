use ::rand::{thread_rng, Rng};
use macroquad::{audio, prelude::*};
use snake::{XY, *};

/// Time to wait between each tick, in seconds.
/// Lower = faster.
const TICK_TIME: f32 = 0.1;

// Colours.
const SNAKE_COLOUR: Color = BLUE;
const SNAKE_HEAD_COLOUR: Color = DARKBLUE;
const BACKGROUND: Color = DARKGREEN;
const GRID_BACKGROUND: Color = GREEN;
const APPLE_COLOUR: Color = RED;

#[macroquad::main("Snake")]
async fn main() {
    set_pc_assets_folder("assets");
    let eat_apple_sound = audio::load_sound("eat_apple.wav")
        .await
        .expect("find sound assets/eat_apple.wav");

    // Keeps track of how much time is left till the next tick (when the snake moves).
    let mut tick_timer: f32 = 0.0;
    let mut score: u16 = 0;
    const START_POS: XY = XY {
        x: GRID_SIZE.x / 2 - 3,
        y: GRID_SIZE.y / 2 - 1,
    };
    let mut snake = Snake::new(START_POS);

    // The apples position.
    let mut apple_pos = XY {
        x: START_POS.x + 6,
        ..START_POS
    };

    loop {
        // Set the move direction if a new input occured.
        if let Some(input) = get_char_pressed() {
            snake.change_direction(match input.to_ascii_lowercase() {
                'w' | 'k' => Direction::Up,
                'a' | 'h' => Direction::Left,
                's' | 'j' => Direction::Down,
                'd' | 'l' => Direction::Right,
                // Otherwise don't change it (by setting it to itself).
                _ => snake.direction(),
            });
        }

        tick_timer += get_frame_time();
        // Move the snake if a tick has passed.
        if tick_timer > TICK_TIME {
            tick_timer = 0.0;
            if snake.advance_and_collide() {
                info!("You died!");
                return;
            };

            if *snake.head() == apple_pos {
                snake.grow();
                score += 1;
                // FIXME: audio isn't working for me??
                audio::play_sound_once(eat_apple_sound);

                // move the apple
                let mut rng = thread_rng();
                apple_pos = XY {
                    x: rng.gen_range(0..GRID_SIZE.x),
                    y: rng.gen_range(0..GRID_SIZE.y),
                };
            }
        }

        // Rendering:
        clear_background(BACKGROUND);

        let (scrw, scrh) = (screen_width(), screen_height());

        // The actual size of each grid block, calculated to be as big as possible while fitting in the screen.
        let block_size = (scrw / GRID_SIZE.x as f32).min(scrh / GRID_SIZE.y as f32);

        let grid_width = block_size * GRID_SIZE.x as f32;
        let grid_height = block_size * GRID_SIZE.y as f32;

        // accounts padding on each side of the screen
        let gridx = (scrw - grid_width) / 2.0;
        let gridy = (scrh - grid_height) / 2.0;

        // grid border
        draw_rectangle_lines(gridx, gridy, grid_width, grid_height, 02.0, BLACK);
        draw_rectangle(gridx, gridy, grid_width, grid_height, GRID_BACKGROUND);

        let draw_block = |block: &XY, colour| {
            let x = gridx + block.x as f32 * block_size;
            let y = gridy + block.y as f32 * block_size;
            draw_rectangle(x, y, block_size, block_size, colour);
        };

        draw_block(&apple_pos, APPLE_COLOUR);

        // draw snake
        let mut blocks = snake.blocks().iter();
        draw_block(blocks.next().unwrap(), SNAKE_HEAD_COLOUR); // the head is drawn in a different colour to the body.
        for block in blocks {
            draw_block(block, SNAKE_COLOUR)
        }

        // Print centered score in faded white.
        draw_text(
            &score.to_string(),
            scrw / 2.0,
            60.0,
            60.0,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );

        next_frame().await
    }
}
