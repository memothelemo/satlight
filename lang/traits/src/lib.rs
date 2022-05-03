use std::fmt::Debug;

pub trait SpannedNode: Debug {
    fn span(&self) -> lunar_location::Span;
}
