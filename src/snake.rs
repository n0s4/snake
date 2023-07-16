use macroquad::prelude::*;
use std::collections::VecDeque;

use crate::xy::{XY, *};

pub struct Snake {
    /// The main data of the snake. [`Snake`] is essentially just a queue of the position of each block.
    blocks: VecDeque<XY>,
    /// Movement direction of the snake's head.
    direction: Direction,
    /// Whether the snake will grow when it next [`advance`](Self::advance)s.
    /// Switched `true` by [`grow`](Self::grow)ing and "consumed" by [`advance`](Self::advance).
    /// [`advance`](Self::advance) advances the head but leaves the tail in place if this is `true`, switching it `false` again.
    growing: bool,
}

impl Snake {
    /// A new instance of [`Snake`] with just three blocks and he head's position at `head`.
    pub fn new(head: XY) -> Self {
        let middle = head.shift(Direction::Left);
        let tail = middle.shift(Direction::Left);
        Self {
            blocks: VecDeque::from([head, middle, tail]),
            growing: false,
            direction: Direction::Right,
        }
    }

    /// If the [`Snake`] will collide with itself or the walls of `grid` when [`advance`](Self::advance)d, returns `true`,
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

        // We don't remove the tail if we are growing. This means the head will move forward and the tail stays in place.
        if self.growing {
            self.growing = false;
        } else {
            self.blocks.pop_back();
        }
    }

    /// Whether the [`Snake`] *will* collide with the walls of `grid` or itself the next time it is [`advance`](Self::advance)d.
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
            let mut blocks = self.blocks.iter();
            if !self.growing {
                // We don't check for collisions with the tail block since it will move when advanced
                // *unless* we are going to grow (which means the tail block won't move), hence the conditional.
                blocks.next_back().unwrap();
            }
            blocks.any(|&block| future_head == block)
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

    /// Grow the snake. Calling repeatedly between `advance` calls is the same as calling it once.
    pub fn grow(&mut self) {
        self.growing = true;
    }

    pub fn head(&self) -> &XY {
        self.blocks.front().unwrap()
    }

    pub fn blocks(&self) -> &VecDeque<XY> {
        &self.blocks
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}
