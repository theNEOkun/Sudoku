use std::fmt::Display;

/// Used to change the difficulty of a given puzzle
#[derive(Debug)]
pub enum Difficulties {
    Easy,
    Medium,
    Hard
}

impl Difficulties {

    /// Used to get from number to difficulty
    ///
    /// 0 = easy
    /// 1 = medium
    /// 2 = hard
    /// defaults to medium
    pub fn from_num(num: u8) -> Self {
        match num {
            0 => Difficulties::Easy,
            1 => Difficulties::Medium,
            2 => Difficulties::Hard,
            _ => Difficulties::Medium,
        }
    }

    /// Used to calculate number of empty squares
    ///
    /// Easy = 15 empty
    /// Medium = 30 empty
    /// Hard = 60 empty
    pub fn value(&self) -> usize {
        match self {
            Difficulties::Easy => 16,
            Difficulties::Medium => 8,
            Difficulties::Hard => 4
        }
    }
}

impl Display for Difficulties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Difficulties::Easy => "easy",
            Difficulties::Medium => "medium",
            Difficulties::Hard => "hard"
        };
        write!(f, "{}", name)
    }
}
