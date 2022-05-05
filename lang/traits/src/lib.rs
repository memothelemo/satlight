use std::fmt::Debug;

pub trait SpannedNode: Debug {
    fn span(&self) -> salite_location::Span;
}
