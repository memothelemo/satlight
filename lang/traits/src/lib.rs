use std::fmt::Debug;

pub trait Node: Debug {
    fn span(&self) -> lunar_location::Span;
}
