use macroquad::prelude::*;
use std::collections::VecDeque;

/// Size of a square on the game "grid".
// FIXME: should the library really be interested in the square size?
pub const SQUARE_SIZE: f32 = 50.0;

// Colours.
pub const SNAKE_COLOUR: Color = GREEN;
pub const BACKGROUND: Color = DARKGREEN;

pub struct Snake {
    squares: VecDeque<Vec2>,
    length: usize,
}

impl Snake {
    /// A new instance of [`Snake`] with just two squares, with the head's position at `head_pos`.
    pub fn new(head_pos: Vec2) -> Self {
        Self {
            squares: VecDeque::from([head_pos, head_pos - Vec2::X * SQUARE_SIZE]),
            length: 2,
        }
    }

    /// This advances - or "moves" - the [`Snake`] one block in the given `direction`.
    // TODO: appropriate name for `move` that doesn't require raw ident.
    pub fn advance(&mut self, direction: Vec2) {
        // We add a new head relative to the old head in the direction.
        let new_head = *self.squares.front().unwrap() + direction * SQUARE_SIZE;
        self.squares.push_front(new_head);
        if self.squares.len() > self.length {
            // Then remove the tail (last element).
            self.squares.pop_back();
        }
    }

    pub fn grow(&mut self) {
        self.length += 1;
    }

    pub fn draw(&self) {
        for square in self.squares.iter() {
            draw_rectangle(square.x, square.y, SQUARE_SIZE, SQUARE_SIZE, SNAKE_COLOUR);
        }
    }
}

pub enum InputType {
    ChangeDirection(Vec2),
    Grow,
}

/// Convert input into a direction as a Vec2.
/// ```
/// 'h' => [-1, 0] // left
/// 'j' => [0, -1] // down
/// 'k' => [0, 1] // up
/// 'l' => [1, 0] // right
/// ```
pub fn parse_input(key: KeyCode) -> Option<InputType> {
    // Supports hjkl, arrow keys, and wasd.
    match key {
        KeyCode::H | KeyCode::Left | KeyCode::A => Some(InputType::ChangeDirection(-Vec2::X)), // left
        KeyCode::J | KeyCode::Down | KeyCode::S => Some(InputType::ChangeDirection(Vec2::Y)), // down
        KeyCode::K | KeyCode::Up | KeyCode::W => Some(InputType::ChangeDirection(-Vec2::Y)),  // up
        KeyCode::L | KeyCode::Right | KeyCode::D => Some(InputType::ChangeDirection(Vec2::X)), // right
        KeyCode::Space => Some(InputType::Grow),
        _ => None,
    }
}
