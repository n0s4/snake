use macroquad::prelude::*;
use std::collections::VecDeque;

pub mod xy;
pub use xy::{Direction, XY};

/// Game grid size, measured in "squares".
pub const GRID_SIZE: XY = XY { x: 40, y: 30 };

pub struct Snake {
    /// The [`Snake`] is essentially just a queue of the position of each block.
    blocks: VecDeque<XY>,
    length: usize,
    direction: Direction,
}

impl Snake {
    /// A new instance of [`Snake`] with just two blocks, with the head's position at `head_pos`.
    pub fn new(head_pos: XY) -> Self {
        let tail = head_pos.shift(Direction::Left);
        Self {
            blocks: VecDeque::from([head_pos, tail]),
            length: 2,
            direction: Direction::Right,
        }
    }

    /// Advances (moves) the [`Snake`] one block in it's current `direction`, and checks for collision.
    /// If the [`Snake`] collides with the wall or itself, this returns `true`, otherwise `false`.
    pub fn advance_and_collide(&mut self) -> bool {
        let new_head = match self.head().checked_shift(self.direction) {
            Some(hd) => hd,
            // This will return none if the shifting attempted to bring the x or y below 0, i.e hitting the left or upper wall.
            None => return true,
        };

        if new_head.x >= GRID_SIZE.x || new_head.y >= GRID_SIZE.y {
            return true;
        }

        // Check for collisions with itself.
        if self
            .blocks()
            .iter()
            .any(|&existing_block| new_head == existing_block)
        {
            return true;
        }

        self.blocks.push_front(new_head);

        // We only remove the tail if adding the new head exceeds the length, if we collect an apple
        // then the `length` increments and we don't remove the old head, effectively growing the snake.
        if self.blocks.len() > self.length {
            self.blocks.pop_back();
        }

        false
    }

    /// Changes the direction of the `[Snake]`, unless the direction is the inverse of the current direction.
    pub fn change_direction(&mut self, new_direction: Direction) {
        // We don't want to be able to reverse the snake into itself.
        if new_direction.is_inverse_of(self.direction) {
            return;
        }
        self.direction = new_direction;
    }

    /// Increment the length of the [`Snake`].
    pub fn grow(&mut self) {
        self.length += 1;
    }

    /// The position of the head of this [`Snake`].
    pub fn head(&self) -> &XY {
        self.blocks.front().unwrap()
    }

    /// The direction of this [`Snake`].
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// A reference to the blocks of this [`Snake`].
    pub fn blocks(&self) -> &VecDeque<XY> {
        &self.blocks
    }
}
