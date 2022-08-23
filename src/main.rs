use macroquad::{audio, prelude::*, rand::ChooseRandom};
use snake::{XY, *};
use std::{collections::VecDeque, process::exit};

/// Time to wait between each tick, in seconds.
/// Lower = faster.
const TICK_TIME: f32 = 0.2;

/// Game grid size, measured in "squares".
pub const GRID_SIZE: XY = XY { x: 10, y: 10 };

// Colours.
const SNAKE_COLOUR: Color = BLUE;
const SNAKE_HEAD_COLOUR: Color = DARKBLUE;
const BACKGROUND: Color = DARKGREEN;
const GRID_BACKGROUND: Color = GREEN;
const APPLE_COLOUR: Color = RED;

#[macroquad::main("Snake")]
async fn main() {
    rand::srand(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    set_pc_assets_folder("assets");
    let eat_apple_sound = audio::load_sound("eat_apple.wav")
        .await
        .expect("find sound assets/eat_apple.wav");

    // Keeps track of how much time is left till the next tick (when the snake moves).
    let mut tick_timer: f32 = 0.0;

    let mut score: u16 = 0;
    const START_POS: XY = XY {
        x: GRID_SIZE.x / 2,
        y: GRID_SIZE.y / 2,
    };
    let mut snake = Snake::new(START_POS);

    // The apples position.
    let mut apple_pos = XY {
        x: START_POS.x.saturating_add(3).min(GRID_SIZE.x - 1),
        ..START_POS
    };

    let mut input_queue = VecDeque::new();

    loop {
        get_last_key_pressed()
            .and_then(|input| {
                use KeyCode::*;
                Some(match input {
                    W | K | Up => Direction::Up,
                    A | H | Left => Direction::Left,
                    S | J | Down => Direction::Down,
                    D | L | Right => Direction::Right,
                    _ => return None,
                })
                // We avoid stacking repeated inputs because holding down a key for longer than usual would fill up
                // the input queue and other keypresses would be ignored until the repeated inputs are consumed.
                .filter(|dir| input_queue.back() != Some(dir))
            })
            .map(|new_dir| input_queue.push_back(new_dir));

        tick_timer += get_frame_time();
        // Advance the game if a tick has passed.
        if tick_timer > TICK_TIME {
            tick_timer = 0.0;

            // We only change direction on tick because changing direction twice between ticks could turn the snake into itself.
            if let Some(new_dir) = input_queue.pop_front() {
                snake.change_direction(new_dir);
            }

            // Advance the snake, or die if it collided with anything.
            if snake.advance_or_collide_in(GRID_SIZE) {
                info!("You died!");
                return;
            };

            if *snake.head() == apple_pos {
                snake.grow();
                score += 1;
                // FIXME: audio isn't working for me??
                audio::play_sound_once(eat_apple_sound);

                // Calculate grid positions not occupied by the snake.
                let empty_positions: Vec<XY> = (0..GRID_SIZE.x)
                    .map(|x| (0..GRID_SIZE.y).map(move |y| XY { x, y }))
                    .flatten()
                    .filter(|pos| snake.blocks().iter().all(|snake| snake != pos))
                    .collect();

                apple_pos = *empty_positions.choose().unwrap_or_else(|| {
                    info!("You win!");
                    exit(0);
                });
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

        draw_rectangle(gridx, gridy, grid_width, grid_height, GRID_BACKGROUND);

        let draw_block = |block: &XY, colour| {
            let x = gridx + block.x as f32 * block_size;
            let y = gridy + block.y as f32 * block_size;
            draw_rectangle(x, y, block_size, block_size, colour);
        };

        // Draw chequered grid
        // for (pos, colour) in (0..GRID_SIZE.x)
        //     .map(move |x| (0..GRID_SIZE.y).map(move |y| XY { x, y }))
        //     .flatten()
        //     .zip([GREEN, LIME].into_iter().cycle())
        // {
        //     draw_block(&pos, colour);
        // }

        draw_block(&apple_pos, APPLE_COLOUR);

        draw_block(snake.head(), SNAKE_HEAD_COLOUR); // The head is drawn in a different colour to the body.

        // We skip(1) to not draw over the head.
        for block in snake.blocks().iter().skip(1) {
            draw_block(block, SNAKE_COLOUR)
        }

        // Draw centered score in faded white.
        draw_text(
            &score.to_string(),
            scrw / 2.0,
            60.0,
            60.0,
            Color::new(1.0, 1.0, 1.0, 0.9),
        );

        next_frame().await
    }
}
