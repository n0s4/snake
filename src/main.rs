use macroquad::{audio, prelude::*, rand::ChooseRandom};
mod snake;
use snake::*;
mod xy;
use std::{collections::VecDeque, process::exit};
use xy::{Direction, XY};

/// Time to wait between each tick, in seconds.
/// Lower = faster.
const TICK_TIME: f32 = 0.2;

const GRID_SIZE: XY = XY { x: 15, y: 15 };

// Colours.
const BACKGROUND: Color = DARKGREEN;
const GRID_COLOUR: Color = GREEN;
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

    loop {
        let score = play_game(eat_apple_sound).await;

        match death_screen(score).await {
            DeathScreenResult::PlayAgain => continue,
            DeathScreenResult::Quit => break,
        };
    }
}

async fn play_game(eat_apple_sound: audio::Sound) -> u16 {
    // How much time til the next tick.
    let mut tick_timer: f32 = 0.0;

    let mut score: u16 = 0;
    const MIDDLE: XY = XY {
        x: GRID_SIZE.x / 2,
        y: GRID_SIZE.y / 2,
    };

    let mut snake = Snake::new(XY {
        x: MIDDLE.x.saturating_sub(2).min(GRID_SIZE.x - 1),
        ..MIDDLE
    });

    let mut apple_pos = XY {
        x: MIDDLE.x.saturating_add(1).min(GRID_SIZE.x - 1),
        ..MIDDLE
    };

    let mut input_queue = VecDeque::new();

    loop {
        // Add inputs to the input queue.
        if let Some(input) = get_last_key_pressed() {
            // We only store up to three non-repeating inputs
            if input_queue.back() != Some(&input) && input_queue.len() < 3 {
                input_queue.push_back(input);
            }
        }

        tick_timer += get_frame_time();
        if tick_timer > TICK_TIME {
            tick_timer = 0.0;

            // We only change direction on tick because changing direction twice between ticks could turn the snake into itself.
            // This consumes the input queue until it finds a directional key, then breaks.
            while let Some(input) = input_queue.pop_front() {
                use KeyCode::*;
                let new_dir = match input {
                    W | P | Up => Direction::Up,
                    A | Z | Left => Direction::Left,
                    S | L | Down => Direction::Down,
                    D | X | Right => Direction::Right,
                    _ => continue,
                };
                snake.change_direction(new_dir);
                break;
            }

            if snake.advance_or_collide_in(GRID_SIZE) {
                return score;
            };

            if *snake.head() == apple_pos {
                snake.grow();
                score += 1;
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
        // Rendering

        clear_background(BACKGROUND);

        let (scrw, scrh) = (screen_width(), screen_height());

        // The actual size of each grid block, calculated to be as big as possible while fitting in the screen.
        let block_size = (scrw / GRID_SIZE.x as f32).min(scrh / GRID_SIZE.y as f32);

        let grid_width = block_size * GRID_SIZE.x as f32;
        let grid_height = block_size * GRID_SIZE.y as f32;

        // accounts padding on each side of the screen
        let gridx = (scrw - grid_width) / 2.0;
        let gridy = (scrh - grid_height) / 2.0;

        draw_rectangle(gridx, gridy, grid_width, grid_height, GRID_COLOUR);

        let draw_block = |block: &XY, colour| {
            let x = gridx + block.x as f32 * block_size;
            let y = gridy + block.y as f32 * block_size;
            draw_rectangle(x, y, block_size, block_size, colour);
        };

        // // Draw chequered grid
        // for (pos, colour) in (0..GRID_SIZE.x)
        //     .map(move |x| (0..GRID_SIZE.y).map(move |y| XY { x, y }))
        //     .flatten()
        //     .zip([GREEN, LIME].into_iter().cycle())
        // {
        //     draw_block(&pos, colour);
        // }

        draw_block(&apple_pos, APPLE_COLOUR);

        let gradient = 1.0 / snake.len() as f32 / 2.0;
        let mut colour = 1.0;
        for block in snake.blocks() {
            draw_block(block, Color::new(0.0, 0.43, colour, 1.0));
            colour -= gradient;
        }

        draw_text(
            &score.to_string(),
            scrw / 2.0,
            60.0,
            60.0,
            Color::new(1.0, 1.0, 1.0, 0.7),
        );

        next_frame().await;
    }
}

enum DeathScreenResult {
    PlayAgain,
    Quit,
}

async fn death_screen(score: u16) -> DeathScreenResult {
    loop {
        clear_background(BACKGROUND);

        let (scrw, scrh) = (screen_width(), screen_height());
        let (midx, midy) = (scrw / 2.0, scrh / 2.0);

        draw_text("Game over!", midx - 100.0, midy - 50.0, 50.0, WHITE);
        draw_text(&format!("Score: {score}"), midx - 50.0, midy, 40.0, WHITE);
        draw_text(
            "Press space to play again or q to quit.",
            midx - 220.0,
            midy + 50.0,
            30.0,
            WHITE,
        );

        match get_last_key_pressed() {
            Some(KeyCode::Q) => return DeathScreenResult::Quit,
            Some(KeyCode::Space) => return DeathScreenResult::PlayAgain,
            _ => (),
        }

        next_frame().await
    }
}
