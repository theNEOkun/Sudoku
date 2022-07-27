use crate::board::get_index;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub index: (usize, usize),
    value: Option<u8>,
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
            index: get_index(x, y),
            value: None,
        }
    }

    /// Create a new Position with x, y and a value
    ///
    /// @param x is the position in the x-axis
    /// @param y is the position in the y-axis
    /// @param value is the value the position should contain
    pub fn with_value(x: usize, y: usize, value: u8) -> Self {
        Self {
            x,
            y,
            index: get_index(x, y),
            value: Some(value),
        }
    }

    /// Gets the value
    ///
    /// @return the value of the position
    pub fn get_value(&self) -> Option<u8> {
        self.value
    }

    /// Sets the value
    ///
    /// @param value is the value to set the position to
    pub fn set_value(&mut self, value: u8) {
        self.value = Some(value);
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
