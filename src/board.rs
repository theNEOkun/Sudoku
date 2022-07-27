use crate::position::Position;

pub struct Board {
    positions: Box<[Position; 9 * 9]>,
}

impl Board {
    /// Create a mew empty board, with all positions filled with no value
    pub fn new_empty() -> Self {
        let mut positions = [Position::default(); 9 * 9];
        for y in 0..9 {
            for x in 0..9 {
                positions[(y * 9) + x] = Position::new(x, y);
            }
        }
        Self {
            positions: Box::new(positions)
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

#[cfg(test)]
mod board_test {
    use super::*;

    #[test]
    fn test_creation() {
        let board = Board::new_empty();

        let position = Position::new(2, 2);
        assert_eq!(board[position].get_value(), None);
    }
}
