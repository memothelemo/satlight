#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[inline]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    pub const fn invalid() -> Self {
        Self { start: 1, end: 0 }
    }

    pub fn is_valid(&self) -> bool {
        self.start <= self.end
    }

    pub fn head(self) -> Self {
        Self {
            start: self.start,
            end: self.start,
        }
    }

    pub fn tail(self) -> Self {
        Self {
            start: self.end,
            end: self.end,
        }
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            start: usize::min(self.start, other.start),
            end: usize::max(self.end, other.end),
        }
    }

    pub fn range(self) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.start,
            end: self.end,
        }
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({},{})", self.start, self.end)
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::Span;

    #[test]
    fn span_merge() {
        let start = Span { start: 2, end: 8 };
        let other = Span { start: 5, end: 17 };
        assert_eq!(start.merge(other), Span { start: 2, end: 17 });
    }
}
