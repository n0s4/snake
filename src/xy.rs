#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Used for coordinates and grid size. (0, 0) is top left.
pub struct XY {
    pub x: u8,
    pub y: u8,
}

impl XY {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Shifts a position by 1 in `direction`. This saturates instead of overflowing.
    pub fn shift(&self, direction: Direction) -> Self {
        use Direction::*;
        let xy = XY::new;

        match direction {
            Up => xy(self.x, self.y.saturating_sub(1)),
            Down => xy(self.x, self.y.saturating_add(1)),
            Left => xy(self.x.saturating_sub(1), self.y),
            Right => xy(self.x.saturating_add(1), self.y),
        }
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
        use Direction::*;
        other
            == match self {
                Up => Down,
                Down => Up,
                Left => Right,
                Right => Left,
            }
    }
}
