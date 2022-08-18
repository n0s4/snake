use macroquad::prelude::*;
use std::collections::VecDeque;

/// Size of a square on the game "grid".
// TODO: Square size shouldn't be constant due to window resizing.
pub const SQUARE_SIZE: f32 = 30.0;

// Colours.
const SNAKE_COLOUR: Color = BLUE;
const SNAKE_HEAD_COLOUR: Color = DARKBLUE;
pub const BACKGROUND: Color = GREEN;
pub const APPLE_COLOUR: Color = RED;

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
        let mut squares = self.squares.iter();
        let head = squares.next().unwrap();

        // We draw the head in a different colour
        draw_square_at(head, SNAKE_HEAD_COLOUR);

        for square in squares {
            draw_square_at(square, SNAKE_COLOUR)
        }
    }

    /// The position of the head of the snake.
    pub fn head(&self) -> &Vec2 {
        self.squares.front().unwrap()
    }
}

pub fn draw_square_at(pos: &Vec2, colour: Color) {
    draw_rectangle(pos.x, pos.y, SQUARE_SIZE, SQUARE_SIZE, colour)
}
