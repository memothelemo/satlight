#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn from_offset(offset: usize, text: &str) -> Position {
        if offset == 0 {
            return Position { line: 1, column: 1 };
        }

        let mut line = 1;
        let mut column = 1;

        for (iter_offset, c) in text.char_indices() {
            let iter_offset = iter_offset + 1;
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            if iter_offset == offset as usize {
                return Position { line, column };
            }
        }

        Position { line, column }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location({},{})", self.line, self.column)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_from_offset() {
        static SAMPLE: &str = "\nabc";
        assert_eq!(Position::from_offset(0, SAMPLE), Position::new(1, 1));
        assert_eq!(Position::from_offset(1, SAMPLE), Position::new(2, 1));
        assert_eq!(Position::from_offset(2, SAMPLE), Position::new(2, 2));
    }
}
