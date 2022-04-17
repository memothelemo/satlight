#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod utils;
pub use utils::*;

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    pub fn is_valid(&self) -> bool {
        self.start <= self.end
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn between(&self, offset: usize) -> bool {
        offset >= self.start && offset <= self.end
    }

    pub fn standard_span(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    #[inline]
    pub fn outside(&self, offset: usize) -> bool {
        !self.between(offset)
    }

    #[inline]
    pub fn from_two_spans(one: Span, two: Span) -> Span {
        Span::new(one.start, two.end)
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({},{})", self.start, self.end)
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({},{})", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_between() {
        let span = Span::new(2, 10);
        assert_eq!(span.between(2), true);
        assert_eq!(span.between(10), true);
        assert_eq!(span.between(0), false);
        assert_eq!(span.between(11), false);
    }

    #[test]
    fn span_valid() {
        assert_eq!(Span::new(2, 10).is_valid(), true);
        assert_eq!(Span::new(5, 5).is_valid(), true);
        assert_eq!(Span::new(10, 5).is_valid(), false);
    }

    #[test]
    fn span_outside() {
        let span = Span::new(2, 10);
        assert_eq!(span.outside(2), false);
        assert_eq!(span.outside(10), false);
        assert_eq!(span.outside(0), true);
        assert_eq!(span.outside(11), true);
    }

    #[test]
    fn get_text_range() {
        let sample: &str = "This is me, your only one fan.";
        assert_eq!(get_text_ranged(&sample, Span::new(0, 3)), Ok("Thi"));
        assert_eq!(
            get_text_ranged(&sample, Span::new(10, 5)),
            Err(RangeOutOfBounds(Span::new(10, 5)))
        );
    }

    #[test]
    fn position_from_offset() {
        let sample: &str = "\nh";
        assert_eq!(Position::from_offset(0, sample), Position::new(1, 1));
        assert_eq!(Position::from_offset(1, sample), Position::new(2, 1));
        assert_eq!(Position::from_offset(2, sample), Position::new(2, 2));
    }
}
