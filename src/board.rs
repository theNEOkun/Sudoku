use crate::position::Position;

pub struct Board {
    positions: Vec<Position>,
}

impl Board {
    pub fn new_empty() -> Self {
        let mut positions = Vec::new();
        for y in 0..=9 {
            for x in 0..=9 {
                positions[(y * 9) + x] = Position::new(x, y);
            }
        }
        Self {
            positions
        }
    }
}

impl std::ops::Index<Position> for Board {
    type Output = Position;

    /// Indexes the underlying structure with an index
    fn index(&self, index: Position) -> &Self::Output {
        &self.positions[index.index]
    }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = Position;

    /// Indexes the underlying structure with a tuple of (x, y)
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = (index.1 * 9) + index.0;
        &self.positions[index]
    }
}

impl std::ops::Index<usize> for Board {
    type Output = Position;

    /// Indexes the underlying structure with an index
    fn index(&self, index: usize) -> &Self::Output {
        &self.positions[index]
    }
}
