use crate::position::Position;

pub struct Board {
    positions: Box<[[Position; 9]; 9]>,
}

/// Gets the square based on the x and y
///
/// @params x is the coordinate on the x-axis,
/// @params y is the coordinate on the y-axis
fn get_square(x: usize, y: usize) -> usize {
    if y < 3 {
        if x < 3 {
            0
        } else if x < 6 {
            1
        } else {
            2
        }
    } else if y < 6 {
        if x < 3 {
            3
        } else if x < 6 {
            4
        } else {
            5
        }
    } else {
        if x < 3 {
            6
        } else if x < 6 {
            7
        } else {
            8
        }
    }
}

/// Method used to get the indexes for the two arrays
///
/// @param x is the global x position (0 - 8)
/// @param y is the global y position (0 - 8)
///
/// @return a tuple of (first, second)
pub fn get_index(x: usize, y: usize) -> (usize, usize) {
    let first_array = get_square(x, y);
    let x = x % 3;
    let y = y % 3;
    let second_array = (y * 3) + x;
    (first_array, second_array)
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

    /// Used to create and fill a board with values
    ///
    /// @param info is a vec of vec with u8s to fill each position in the board. 
    ///         inner vecs represent a square
    pub fn new_with_info(info: Vec<Vec<u8>>) -> Self {
        let mut positions = [[Position::default();9 ]; 9];
        
        for (o_index, outer_each) in info.iter().enumerate() {
            for (i_index, inner_each) in outer_each.iter().enumerate() {
                    positions[o_index][i_index] = Position::with_value(i_index, o_index, *inner_each);
            }
        }

        for (o_index, outer_each) in positions.clone().iter().enumerate() {
            for (i_index, inner_each) in outer_each.iter().enumerate() {
                if let None = inner_each.get_value() {
                    positions[o_index][i_index].set_value(10);
                }
            }
        }

        Self {
            positions: Box::new(positions)
        }
    }

    /// Method to test the whole board
    ///
    /// @return true if the board is correct, else false
    pub fn test_board(&self) -> bool {
        let mut returnval = true;
        for each in 0..9 {
            returnval &= self.test_row(each);
            returnval &= self.test_column(each);
            returnval &= self.test_square(each);
        }
        returnval
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
        for row in 0..9 {
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

    /// Method to test a square for if it is correct
    ///
    /// @param square is  the square to test [
    /// 0, 1, 2
    /// 3, 4, 5
    /// 6, 7, 8
    /// ]
    pub fn test_square(&self, square: usize) -> bool {
        let mut tests = [false; 9];
        for position in 0..9 {
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

    fn get_board_with_values() -> Board {
        let inner_info_1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let inner_info_2 = vec![3, 4, 5, 6, 7, 8, 0, 1, 2];
        let inner_info_3 = vec![6, 7, 8, 0, 1, 2, 3, 4, 5];
        let inner_info_4 = vec![1, 2, 0, 4, 5, 3, 7, 8, 6];
        let inner_info_5 = vec![4, 5, 3, 7, 8, 6, 1, 2, 0];
        let inner_info_6 = vec![7, 8, 6, 1, 2, 0, 4, 5, 3];
        let inner_info_7 = vec![2, 0, 1, 5, 3, 4, 8, 6, 7];
        let inner_info_8 = vec![5, 3, 4, 8, 6, 7, 2, 0, 1];
        let inner_info_9 = vec![8, 6, 7, 2, 0, 1, 5, 3, 4];

        let info = vec![
            inner_info_1, inner_info_2, inner_info_3,
            inner_info_4, inner_info_5, inner_info_6,
            inner_info_7, inner_info_8, inner_info_9
        ];
        Board::new_with_info(info)
    }

    fn get_board_with_false_values() -> Board {
        let inner_info_1 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_2 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_3 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_4 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_5 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_6 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_7 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_8 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_9 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];

        let info = vec![
            inner_info_1, inner_info_2, inner_info_3,
            inner_info_4, inner_info_5, inner_info_6,
            inner_info_7, inner_info_8, inner_info_9
        ];
        Board::new_with_info(info)
    }

    #[test]
    fn test_creation() {
        let board = Board::new_empty();

        let position = Position::new(2, 2);
        assert_eq!(board[position].get_value(), None);
    }

    #[test]
    fn test_row_manual() {
        let board = get_board_with_values();

        assert_eq!(Some(0), board[(0, 0)].get_value());
        assert_eq!(Some(1), board[(0, 1)].get_value());
        assert_eq!(Some(2), board[(0, 2)].get_value());
        assert_eq!(Some(3), board[(0, 3)].get_value());
        assert_eq!(Some(4), board[(0, 4)].get_value());
        assert_eq!(Some(5), board[(0, 5)].get_value());
        assert_eq!(Some(6), board[(0, 6)].get_value());
        assert_eq!(Some(7), board[(0, 7)].get_value());
        assert_eq!(Some(8), board[(0, 8)].get_value());

        assert_eq!(Some(3), board[(1, 0)].get_value());
        assert_eq!(Some(4), board[(1, 1)].get_value());
        assert_eq!(Some(5), board[(1, 2)].get_value());
        assert_eq!(Some(6), board[(1, 3)].get_value());
        assert_eq!(Some(7), board[(1, 4)].get_value());
        assert_eq!(Some(8), board[(1, 5)].get_value());
        assert_eq!(Some(0), board[(1, 6)].get_value());
        assert_eq!(Some(1), board[(1, 7)].get_value());
        assert_eq!(Some(2), board[(1, 8)].get_value());
    }

    #[test]
    fn test_board() {
        let board = get_board_with_values();

        assert!(board.test_board());

        let board = get_board_with_false_values();

        assert!(!board.test_board());
    }

    #[test]
    fn test_row() {
        let board = get_board_with_values();

        assert!(board.test_row(0));

        let board = get_board_with_false_values();

        assert!(!board.test_row(0));
    }

    #[test]
    fn test_column() {
        let board = get_board_with_values();

        assert!(board.test_column(0));

        let board = get_board_with_false_values();

        assert!(!board.test_column(0));
    }

    #[test]
    fn test_get_indexes() {
        let square = 4;
        let index = 6;

        let (first, second) = get_index(3, 5);

        assert_eq!(first, square);
        assert_eq!(second, index);

        let square = 8;
        let index = 6;

        let (first, second) = get_index(6, 8);

        assert_eq!(first, square);
        assert_eq!(second, index);
    }
}
