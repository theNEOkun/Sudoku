use crate::position::Position;
use rand::{seq::SliceRandom, thread_rng};

const BASE: usize = 3;
const SIDE: usize = BASE * BASE;

/// Gets the square based on the x and y
///
/// ## Arguments
///
/// *s x is the coordinate on the x-axis,
/// *s y is the coordinate on the y-axis
fn get_square(x: usize, y: usize) -> usize {
    if y < BASE {
        if x < BASE {
            0
        } else if x < 6 {
            1
        } else {
            2
        }
    } else if y < 6 {
        if x < BASE {
            3
        } else if x < 6 {
            4
        } else {
            5
        }
    } else {
        if x < BASE {
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
/// ## Arguments
///
/// * x - is the global x position (0 - 8)
/// * y - is the global y position (0 - 8)
///
/// ## Returns
/// a tuple of (first, second)
pub fn get_index(x: usize, y: usize) -> (usize, usize) {
    let first_array = get_square(x, y);
    let x = x % BASE;
    let y = y % BASE;
    let second_array = (y * BASE) + x;
    (first_array, second_array)
}

fn pattern(r: usize, c: usize) -> usize {
    (BASE * (r % BASE) + r / BASE + c) % SIDE
}

pub struct Board {
    /// is the matrix of which the sudoku-square is
    /// [position](../position/struct.Position.html)
    positions: Box<[[Position; SIDE]; SIDE]>,
}

impl Board {
    pub fn new() -> Self {
        let positions = [[Position::default(); SIDE]; SIDE];

        let mut board = Self {
            positions: Box::new(positions),
        };

        let mut rng = thread_rng();

        let mut rows = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        rows.shuffle(&mut rng);
        let mut cols = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        cols.shuffle(&mut rng);

        let mut nums = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        nums.shuffle(&mut rng);

        for r in rows.iter() {
            for c in cols.iter() {
                board[(*r, *c)].reset(*c, *r, nums[pattern(*r, *c)]);
            }
        }
        board
    }

    /// Create a mew empty board, with all positions filled with no value
    pub fn new_empty() -> Self {
        let mut positions = [[Position::default(); 9]; 9];
        for y in 0..9 {
            for x in 0..9 {
                //let (first, second) = get_index(x, y);
                positions[y][x].reset_none(x, y);
            }
        }
        Self {
            positions: Box::new(positions),
        }
    }

    /// Used to create and fill a board with values
    ///
    /// ## Arguments
    ///
    /// * info - is a vec of vec with usizes to fill each position in the board.
    ///         inner vecs represent a square
    pub fn with_squares(info: [[usize; 9]; 9]) -> Self {
        let mut positions = [[Position::default(); 9]; 9];

        for (first, outer_each) in info.iter().enumerate() {
            for (second, inner_each) in outer_each.iter().enumerate() {
                let (first, second) = get_index(second, first);
                positions[first][second].reset(second, first, *inner_each);
            }
        }

        Self {
            positions: Box::new(positions),
        }
    }

    /// Used to create and fill a board with values
    ///
    /// ## Arguments
    ///
    /// * info - is a vec of vec with usizes to fill each position in the board.
    ///         inner vecs represent a square
    pub fn with_rows(info: [[usize; 9]; 9]) -> Self {
        let mut positions = [[Position::default(); 9]; 9];

        for (first, outer_each) in info.iter().enumerate() {
            for (second, inner_each) in outer_each.iter().enumerate() {
                positions[first][second].reset(first, second, *inner_each);
            }
        }

        Self {
            positions: Box::new(positions),
        }
    }

    /// Method to test the whole board
    ///
    /// ## Returns
    /// true if the board is correct, else false
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
    /// ## Arguments
    ///
    /// * row - is the row to test
    /// ## Returns
    ///  true if no number is seen twice, and all < 9
    pub fn test_row(&self, row: usize) -> bool {
        let mut tests = 0b000000000;
        for column in 0..SIDE {
            let pos = self[(row, column)];
            if let Some(value) = pos.get_value() {
                let pos = 1 << value;
                if !(((tests & pos) >> value) == 1) {
                    tests |= pos;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        0b111111111 == tests
    }

    pub fn num_in_row(&self, row: usize, num: usize) -> bool {
        for column in 0..SIDE {
            let pos = self[(row, column)];
            if let Some(val) = pos.get_value() {
                if val == num {
                    return false
                }
            }
        }
        true
    }

    /// Method to test a given column for if it is correct
    ///
    /// ## Arguments
    ///
    /// * column - The column to test
    ///
    /// ## Return
    ///
    /// true if no number is seen twice, and all < 9
    pub fn test_column(&self, column: usize) -> bool {
        let mut tests = 0b000000000;
        for row in 0..SIDE {
            let pos = self[(row, column)];
            if let Some(value) = pos.get_value() {
                let pos = 1 << value;
                if !(((tests & pos) >> value) == 1) {
                    tests |= pos;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        0b111111111 == tests
    }

    pub fn num_in_column(&self, column: usize, num: usize) -> bool {
        for row in 0..SIDE {
            let pos = self[(row, column)];
            if let Some(val) = pos.get_value() {
                if val == num {
                    return false
                }
            }
        }
        true
    }

    /// Method to test a square for if it is correct
    ///
    /// ## Arguments
    ///
    /// * square - is  the square to test
    /// [
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8
    /// ]
    /// ## Returns
    /// true if no number is seen twice, and all < 9
    pub fn test_square(&self, square: usize) -> bool {
        let mut tests = 0b000000000;
        for position in 0..SIDE {
            let (first, second) = get_index(square, position);
            let pos = self.positions[first][second];
            if let Some(value) = pos.get_value() {
                let pos = 1 << value;
                if !(((tests & pos) >> value) == 1) {
                    tests |= pos;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        0b111111111 == tests
    }

    pub fn num_in_square(&self, square: usize, num: usize) -> bool {
        for position in 0..SIDE {
            let (first, second) = get_index(square, position);
            let pos = self.positions[first][second];
            if let Some(value) = pos.get_value() {
                if value == num {
                    return false;
                }
            }
        }
        true
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");
        for each in self.positions.iter() {
            output += &format!("{:?}\n", each);
        }
        write!(f, "")
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for each in self.positions.iter() {
            for each in each {
                if let Some(val) = each.get_value() {
                    write!(f, "|{}|", val + 1).unwrap();
                } else {
                    write!(f, "| |").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

impl std::ops::Index<Position> for Board {
    type Output = Position;

    /// Indexes the underlying structure with an index
    fn index(&self, index: Position) -> &Self::Output {
        &self[index.index]
    }
}

impl std::ops::IndexMut<Position> for Board {
    /// Indexes the underlying structure with an index
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.index]
    }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = Position;

    /// Indexes the underlying structure with a tuple of (x, y)
    fn index(&self, (first, second): (usize, usize)) -> &Self::Output {
        //let (first, second) = get_index(second, first);
        &self.positions[first][second]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Board {

    /// Indexes the underlying structure with a tuple of (x, y)
    fn index_mut(&mut self, (first, second): (usize, usize)) -> &mut Self::Output {
        //let (first, second) = get_index(second, first);
        &mut self.positions[first][second]
    }
}

#[cfg(test)]
mod board_test {
    use super::*;

    fn get_board() -> Board {
        Board::new_empty()
    }

    fn get_board_with_values() -> Board {
        let inner_info_1 = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let inner_info_2 = [3, 4, 5, 6, 7, 8, 0, 1, 2];
        let inner_info_3 = [6, 7, 8, 0, 1, 2, 3, 4, 5];
        let inner_info_4 = [1, 2, 0, 4, 5, 3, 7, 8, 6];
        let inner_info_5 = [4, 5, 3, 7, 8, 6, 1, 2, 0];
        let inner_info_6 = [7, 8, 6, 1, 2, 0, 4, 5, 3];
        let inner_info_7 = [2, 0, 1, 5, 3, 4, 8, 6, 7];
        let inner_info_8 = [5, 3, 4, 8, 6, 7, 2, 0, 1];
        let inner_info_9 = [8, 6, 7, 2, 0, 1, 5, 3, 4];

        let info = [
            inner_info_1,
            inner_info_2,
            inner_info_3,
            inner_info_4,
            inner_info_5,
            inner_info_6,
            inner_info_7,
            inner_info_8,
            inner_info_9,
        ];
        Board::with_rows(info)
    }

    fn get_board_with_false_values() -> Board {
        let inner_info_1 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_2 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_3 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_4 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_5 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_6 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_7 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_8 = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        let inner_info_9 = [1, 1, 1, 1, 1, 1, 1, 1, 1];

        let info = [
            inner_info_1,
            inner_info_2,
            inner_info_3,
            inner_info_4,
            inner_info_5,
            inner_info_6,
            inner_info_7,
            inner_info_8,
            inner_info_9,
        ];
        Board::with_squares(info)
    }

    #[test]
    fn test_creation() {
        let board = Board::new_empty();

        let position = Position::new(2, 2);
        assert_eq!(board[position].get_value(), None);
    }

    #[test]
    fn test_random() {
        let board = Board::new();

        let position = Position::new(2, 2);
        assert!(board[position].get_value().is_some());
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

        let board = get_board_with_false_values();

        assert_eq!(Some(1), board[(0, 0)].get_value());
        assert_eq!(Some(1), board[(0, 1)].get_value());
        assert_eq!(Some(1), board[(0, 2)].get_value());
        assert_eq!(Some(1), board[(0, 3)].get_value());
        assert_eq!(Some(1), board[(0, 4)].get_value());
        assert_eq!(Some(1), board[(0, 5)].get_value());
        assert_eq!(Some(1), board[(0, 6)].get_value());
        assert_eq!(Some(1), board[(0, 7)].get_value());
        assert_eq!(Some(1), board[(0, 8)].get_value());
    }

    #[test]
    fn test_board() {
        let board = get_board_with_values();

        assert!(board.test_board());

        let board = Board::new();

        assert!(board.test_board());

        let board = get_board_with_false_values();

        assert!(!board.test_board());
    }

    #[test]
    fn test_row() {
        let board = get_board_with_values();

        assert!(board.test_row(0));

        let board = Board::new();

        assert!(board.test_row(0));

        let board = get_board_with_false_values();

        assert!(!board.test_row(0));
    }

    #[test]
    fn test_column() {
        let board = get_board_with_values();

        assert!(board.test_column(0));

        let board = Board::new();

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
