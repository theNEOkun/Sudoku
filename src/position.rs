use crate::board::get_index;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub index: (usize, usize),
}

impl Position {
    /// Create a new position with an x and y
    ///
    /// @param x is the position in the x-axis
    /// @param y is the position in the y-axis
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            index: (x, y),
        }
    }

    /// Create a new Position with x, y and a value
    ///
    /// @param x is the position in the x-axis
    /// @param y is the position in the y-axis
    /// @param value is the value the position should contain
    pub fn with_value(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            index: get_index(x, y),
        }
    }

    /// Reset the value from default to with information
    ///
    /// @param x is the position in the x-axis
    /// @param y is the position in the y-axis
    /// @param value is the value the position should have
    pub fn reset(&mut self, x: usize, y: usize) -> &Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Reset with value to None
    ///
    /// ## Arguments
    ///
    /// * x - the position on the x-axis
    /// * y - the position on the y-axis
    pub fn reset_none(&mut self, x: usize, y: usize) -> &Self {
        self.x = x;
        self.y = y;
        self
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod position_test {
    use super::*;

    fn get_addr(addr: &Position) -> usize {
        let raw_prt = addr as *const Position;
        raw_prt as usize
    }
}
