#[derive(PartialEq, Eq, Clone, Copy)]
/// Used for co-ordinates. (0, 0) is top left.
pub struct XY {
    pub x: u8,
    pub y: u8,
}

impl XY {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn shift(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => XY::new(self.x, self.y.saturating_sub(1)),
            Direction::Down => XY::new(self.x, self.y + 1),
            Direction::Left => XY::new(self.x.saturating_sub(1), self.y),
            Direction::Right => XY::new(self.x + 1, self.y),
        }
    }

    pub fn checked_shift(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Down => XY::new(self.x, self.y + 1),
            Direction::Right => XY::new(self.x + 1, self.y),

            Direction::Up => XY::new(
                self.x,
                match self.y.checked_sub(1) {
                    Some(y) => y,
                    None => return None,
                },
            ),

            Direction::Left => XY::new(
                match self.x.checked_sub(1) {
                    Some(x) => x,
                    None => return None,
                },
                self.y,
            ),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn is_inverse_of(self, other: Self) -> bool {
        other
            == match self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
    }
}
