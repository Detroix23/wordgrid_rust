// src/maths.rs
use std::ops;

pub type CharGrid = Vec<Vec<char>>;

/// A set of 2 integer coordinates. Similar to an `integer vector`.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Size> for Size {
    type Output = Size;

    /// Add the self and the Right Hand Side (rhs).
    fn add(self, rhs: Size) -> Size {
        Size {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}