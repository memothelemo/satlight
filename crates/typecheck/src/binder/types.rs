use super::*;
use id_arena::Id;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Typ {
    Bool,
    Error,
    Number,
    Nil,
    String,
    Unknown,
    Variable(Id<Symbol>),
    Void,
}
