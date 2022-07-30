use crate::board::get_index;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub index: (usize, usize),
    value: Option<usize>,
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
            index: (x, y), //get_index(x, y),
            value: None,
        }
    }

    /// Create a new Position with x, y and a value
    ///
    /// @param x is the position in the x-axis
    /// @param y is the position in the y-axis
    /// @param value is the value the position should contain
    pub fn with_value(x: usize, y: usize, value: usize) -> Self {
        Self {
            x,
            y,
            index: get_index(x, y),
            value: Some(value),
        }
    }

    /// Reset the value from default to with information
    ///
    /// @param x is the position in the x-axis
    /// @param y is the position in the y-axis
    /// @param value is the value the position should have
    pub fn reset(&mut self, x: usize, y: usize, value: usize) -> &Self {
        self.x = x;
        self.y = y;
        self.value = Some(value);
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

    /// Gets the value
    ///
    /// @return the value of the position
    pub fn get_value(&self) -> Option<usize> {
        self.value
    }

    /// Sets the value
    ///
    /// @param value is the value to set the position to
    pub fn set_value(&mut self, value: usize) {
        self.value = Some(value);
    }

    pub fn set_dir(&mut self, value: Option<usize>) {
        self.value = value;
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = self.value {
            write!(f, "(x: {}, y:{}): {:?}", self.x, self.y, val)
        } else {
            write!(f, "(x: {}, y:{})", self.x, self.y)
        }
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

    #[test]
    fn test_value() {
        let position = Position::with_value(0, 0, 8);

        if let Some(value) = position.get_value() {
            assert_eq!(value, 8);
        }
    }

    #[test]
    fn test_equals() {
        let position_a = Position::new(0, 0);
        let position_b = Position::new(0, 1);
        let position_c = Position::new(0, 0);

        assert_ne!(position_a, position_b);
        assert_eq!(position_a, position_c);
    }
}
