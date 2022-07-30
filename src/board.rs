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

/// Used to remove values from the board
fn removal(board: &mut Board) {
    let squares = SIDE * SIDE;
    let empties = (squares * 3) / 4;
    let mut vec = (0..squares).collect::<Vec<usize>>();
    vec.shuffle(&mut thread_rng());

    for each in vec[0..empties].iter() {
        board[(each/SIDE, each%SIDE)].set_dir(None);
    }
}

fn create_empty_array() -> [[Position; SIDE]; SIDE] {
    let mut vec = [[Position::default(); SIDE]; SIDE];
    for first in 0..SIDE {
        for second in 0..SIDE {
            vec[first][second].reset_none(first, second);
        }
    }
    vec
}

pub struct Board {
    /// is the matrix of which the sudoku-square is
    /// [position](../position/struct.Position.html)
    pub filled: Box<[[Position; SIDE]; SIDE]>,
    empty: Box<[[Position; SIDE]; SIDE]>,
    tries: Box<[[Position; SIDE]; SIDE]>,
}

const NUMBERS: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

impl Board {

    /// Creates a new board, with all positions filled
    ///
    /// ## Return
    ///
    /// a board with all positions uniquely filled
    pub fn new() -> Self {
        let mut positions = [[Position::default(); SIDE]; SIDE];

        let mut rng = thread_rng();

        let mut rows = NUMBERS;
        rows.shuffle(&mut rng);
        let mut cols = NUMBERS;
        cols.shuffle(&mut rng);

        let mut nums = NUMBERS;
        nums.shuffle(&mut rng);

        for r in rows.iter() {
            for c in cols.iter() {
                positions[*r][*c].reset(*c, *r, nums[pattern(*r, *c)]);
            }
        }

        let mut board = Self {
            filled: Box::new(positions),
            empty: Box::new(positions),
            tries: Box::new(create_empty_array()),
        };
        removal(&mut board);
        board
    }

    /// Create a mew empty board, with all positions filled with no value
    pub fn new_empty() -> Self {
        let positions = create_empty_array();
        Self {
            filled: Box::new(positions),
            empty: Box::new(positions),
            tries: Box::new(create_empty_array()),
        }
    }

    /// Used to create and fill a board with values
    ///
    /// ## Arguments
    ///
    /// * info -is a vec of vec with usizes to fill each position in the board.
    ///         inner vecs represent a square
    pub fn with_squares(info: [[usize; SIDE]; SIDE]) -> Self {
        let mut filled = [[Position::default(); SIDE]; SIDE];

        for (first, outer_each) in info.iter().enumerate() {
            for (second, inner_each) in outer_each.iter().enumerate() {
                let (first, second) = get_index(second, first);
                if *inner_each < 9 {
                    filled[first][second].reset(first, second, *inner_each);
                } else {
                    filled[first][second].reset_none(first, second);
                }
            }
        }

        Self {
            filled: Box::new(filled),
            empty: Box::new(filled),
            tries: Box::new(create_empty_array()),
        }
    }

    pub fn add_number(&mut self, x: usize, y: usize, num: usize) -> bool {
        if let None = self[(x, y)].get_value() {
            let square = get_square(x, y);
            if self.num_in_row(y, num) && self.num_in_column(x, num) && self.num_in_square(square, num) {
                self.tries[y][x].set_value(num);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Used to create and fill a board with values
    ///
    /// ## Arguments
    ///
    /// * info -is a vec of vec with usizes to fill each position in the board.
    ///         inner vecs represent a square
    pub fn with_rows(info: [[usize; SIDE]; SIDE]) -> Self {
        let mut filled = [[Position::default(); SIDE]; SIDE];

        for (first, outer_each) in info.iter().enumerate() {
            for (second, inner_each) in outer_each.iter().enumerate() {
                if *inner_each < 9 {
                    filled[first][second].reset(first, second, *inner_each);
                } else {
                    filled[first][second].reset_none(first, second);
                }
            }
        }

        Self {
            filled: Box::new(filled),
            empty: Box::new(filled),
            tries: Box::new(create_empty_array()),
        }
    }

    /// Method to test the whole board
    ///
    /// ## Returns
    /// true if the board is correct, else false
    pub fn test_board(&self) -> bool {
        let mut returnval = true;
        for each in 0..SIDE {
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

    /// Tests if a number is already in the row
    ///
    /// ## Arguments
    ///
    /// * row - the row to test
    /// * num - the number to test in the row
    ///
    /// ## Returns
    ///
    /// Return true if the number is NOT in the row, else false
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

    /// Tests if a number is already in the column
    ///
    /// ## Arguments
    ///
    /// * column - the column to test
    /// * num - the number to test in the row
    ///
    /// ## Returns
    ///
    /// Return true if the number is NOT in the column, else false
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
            let pos = self.filled[first][second];
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

    /// Tests if a number is already in the square
    ///
    /// ## Arguments
    ///
    /// * square - the square to test
    /// * num - the number to test in the row
    ///
    /// ## Returns
    ///
    /// Return true if the number is NOT in the square, else false
    pub fn num_in_square(&self, square: usize, num: usize) -> bool {
        for position in 0..SIDE {
            let (first, second) = get_index(square, position);
            let pos = self.filled[first][second];
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
        for each in self.filled.iter() {
            for each in each {
                if let Some(val) = each.get_value() {
                    write!(f, "|{}|", val + 1)?;
                } else {
                    write!(f, "| |")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        for each in self.empty.iter() {
            for each in each {
                if let Some(val) = each.get_value() {
                    write!(f, "|{}|", val + 1)?;
                } else {
                    write!(f, "| |")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for each in self.empty.iter() {
            for each in each {
                if let Some(val) = each.get_value() {
                    write!(f, "|{}|", val + 1)?;
                } else {
                    write!(f, "| |")?;
                }
            }
            write!(f, "\n")?;
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
        &self.empty[first][second]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Board {

    /// Indexes the underlying structure with a tuple of (x, y)
    fn index_mut(&mut self, (first, second): (usize, usize)) -> &mut Self::Output {
        //let (first, second) = get_index(second, first);
        &mut self.empty[first][second]
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

    fn get_empty_board() -> Board {
        let inner_info_1 = [7,  3,  5,  9,  10, 10,  8, 10, 10];
        let inner_info_2 = [9, 10, 10, 10,  6, 10, 10,  10, 10];
        let inner_info_3 = [8, 10, 10, 10, 10, 10, 10,   1, 10];
        let inner_info_4 = [10,10,  9, 10, 10, 10, 10,   4, 10];
        let inner_info_5 = [10,10,  8, 10, 10, 10, 10,  10, 10];
        let inner_info_6 = [10,10, 10, 10, 10,  9, 10,  10, 10];
        let inner_info_7 = [10, 9, 10, 10, 10,  6, 10,  10,  3];
        let inner_info_8 = [10, 8, 10, 10, 10,  3, 10,  10, 10];
        let inner_info_9 = [4,  7,  3, 10, 10, 10,  2,  10,  6];

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
    fn test_add_number() {
        let mut board = get_empty_board();

        assert!(!board.add_number(0, 0, 1));
        assert!(board.add_number(0, 5, 1));
        assert!(board.add_number(4, 4, 1));
    }

    #[test]
    fn test_number_rows() {
        let board = get_empty_board();

        assert!(board.num_in_row(0, 1));

        assert!(board.num_in_row(1, 1));

        assert!(board.num_in_row(8, 1));
    }

    #[test]
    fn test_number_columns() {
        let board = get_empty_board();

        assert!(board.num_in_column(0, 1));

        assert!(board.num_in_column(1, 1));

        assert!(board.num_in_column(8, 1));
    }

    #[test]
    fn test_number_squares() {
        let board = get_empty_board();

        assert!(board.num_in_square(0, 1));

        assert!(board.num_in_square(1, 1));

        assert!(board.num_in_square(8, 1));
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

        assert!(board.filled[2][2].get_value().is_some());
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
        
        for each in 0..SIDE {
            assert!(board.test_row(each));
        }

        let board = Board::new();
        
        for each in 0..SIDE {
            assert!(board.test_row(each));
        }

        let board = get_board_with_false_values();
        
        for each in 0..SIDE {
            // assert NOT
            assert!(!board.test_row(each));
        }
    }

    #[test]
    fn test_column() {
        let board = get_board_with_values();

        for each in 0..SIDE {
            assert!(board.test_column(each));
        }

        let board = Board::new();

        for each in 0..SIDE {
            assert!(board.test_column(each));
        }

        let board = get_board_with_false_values();

        for each in 0..SIDE {
            // assert NOT
            assert!(!board.test_column(each));
        }
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
