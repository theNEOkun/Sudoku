pub struct Position {
    pub x: usize,
    pub y: usize,
    pub index: usize,
    value: Option<u8>,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            index: (y * 9) + x,
            value: None,
        }
    }

    pub fn with_value(x: usize, y: usize, value: u8) -> Self {
        Self {
            x,
            y,
            index: (y * 9) + x,
            value: Some(value),
        }
    }

    pub fn get_value(&self) -> Option<u8> {
        self.value
    }
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
