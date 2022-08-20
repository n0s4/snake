use macroquad::prelude::*;
use std::collections::VecDeque;

pub mod xy;
pub use xy::{Direction, XY};

pub struct Snake {
    /// The main data of the snake. [`Snake`] is essentially just a queue of the position of each block.
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

    /// If the [`Snake`] will collide with itself or the walls of `grid` when [`advance`]d, returns `true`,
    /// otherwise advances the [`Snake`], returning `false`.
    pub fn advance_or_collide_in(&mut self, grid: XY) -> bool {
        self.will_collide_in(grid) || {
            self.advance();
            false
        }
    }

    /// Advances (moves) the [`Snake`] one block in it's current `direction`, without checking for collisions.
    fn advance(&mut self) {
        let new_head = self.head().shift(self.direction);
        self.blocks.push_front(new_head);

        // We only remove the tail if adding the new head exceeds the length, if we collect an apple
        // then the `length` increments and we don't remove the old head, effectively growing the snake.
        if self.blocks.len() > self.length {
            self.blocks.pop_back();
        }
    }

    /// Whether the [`Snake`] *will* collide with the walls of `grid` or itself the next time it is [`advance`]d.
    fn will_collide_in(&self, grid: XY) -> bool {
        let hd = self.head();

        use Direction::*;
        let collided_wall = match self.direction {
            Left => hd.x == 0,
            Up => hd.y == 0,
            Right => hd.x + 1 == grid.x,
            Down => hd.y + 1 == grid.y,
        };

        collided_wall || {
            // If we won't hit a wall then check for self-collisions.
            let future_head = hd.shift(self.direction);
            self.blocks.iter().any(|&block| block == future_head)
        }
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
