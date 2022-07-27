use crate::position::Position;

pub struct Board {
    positions: Vec<Position>,
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
