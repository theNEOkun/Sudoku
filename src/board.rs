use crate::position::Position;

pub struct Board {
    positions: Box<[[Position; 9]; 9]>,
}

fn get_square(num: usize) -> usize {
    if num < 3 {
        0
    } else if num < 6 {
        1
    } else {
        2
    }
}

/// Method used to get the indexes for the two arrays
///
/// @param x is the global x position (0 - 8)
/// @param y is the global y position (0 - 8)
///
/// @return a tuple of (first, second)
pub fn get_index(x: usize, y: usize) -> (usize, usize) {
    let x = x % 3;
    let y = y % 3;
    let second_array = (y * 3) + x;
    (get_square(x) + get_square(y), second_array)
}

impl Board {
    /// Create a mew empty board, with all positions filled with no value
    pub fn new_empty() -> Self {
        let mut positions = [[Position::default(); 9]; 9];
        for y in 0..9 {
            for x in 0..9 {
                let (first, second) = get_index(x, y);
                positions[first][second] = Position::new(x, y);
            }
        }
        Self {
            positions: Box::new(positions)
        }
    }

    /// Method to test a given row for if it is correct
    ///
    /// @param row is the row to test
    pub fn test_row(&self, row: usize) -> bool {
        let mut tests = [false; 9];
        for column in 0..9 as usize {
            let pos = self[(row, column)];
            if let Some(value) = pos.get_value() {
                if !tests[value as usize] {
                    tests[value as usize] = true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Method to test a given column for if it is correct
    ///
    /// @param column is the column to test
    pub fn test_column(&self, column: usize) -> bool {
        let mut tests = [false; 9];
        for row in 0..9 as usize {
            let pos = self[(row, column)];
            if let Some(value) = pos.get_value() {
                if !tests[value as usize] {
                    tests[value as usize] = true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn test_square(&self, square: usize) -> bool {
        let mut tests = [false; 9];
        for position in 0..9 as usize {
            let pos = self.positions[square][position];
            if let Some(value) = pos.get_value() {
                if !tests[value as usize] {
                    tests[value as usize] = true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl std::ops::Index<Position> for Board {
    type Output = Position;

    /// Indexes the underlying structure with an index
    fn index(&self, index: Position) -> &Self::Output {
        &self[index.index]
    }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = Position;

    /// Indexes the underlying structure with a tuple of (x, y)
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (first, second) = get_index(index.1, index.0);
        &self.positions[first][second]
    }
}

#[cfg(test)]
mod board_test {
    use super::*;

    fn get_board() -> Board {
        Board::new_empty()
    }

    #[test]
    fn test_creation() {
        let board = Board::new_empty();

        let position = Position::new(2, 2);
        assert_eq!(board[position].get_value(), None);
    }

    #[test]
    fn test_row() {
        let board = get_board();

        assert!(board.test_row(1));
    }

    #[test]
    fn test_column() {
        let board = get_board();

        assert!(board.test_column(1));
    }

    #[test]
    fn test_get_indexes() {
        let square = 4;
        let index = 6;

        let (first, second) = get_index(3, 5);

        assert_eq!(first, square);
        assert_eq!(second, index);
    }
}
