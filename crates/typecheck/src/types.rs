use super::*;
use id_arena::Id;
use lunar_ast as ast;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Invalid,
    Nil,
    Number,
    Unknown,
    Variable(Id<Variable>),
    Void,
}

impl Type {
    pub fn skip_downwards(&self, checker: &Typechecker) -> Type {
        match self {
            Type::Variable(id) => checker
                .variables
                .get(*id)
                .unwrap()
                .ty
                .clone()
                .skip_downwards(checker),
            _ => self.clone(),
        }
    }

    pub fn cast(&self, other: &Type, checker: &Typechecker) -> Option<Type> {
        // we cannot skip downwards though
        match (self, other) {
            (_, Type::Unknown) => Some(Type::Unknown),
            (Type::Unknown, _) => Some(other.clone()),
            (Type::Nil, Type::Void) => Some(Type::Void),
            (Type::Void, Type::Nil) => Some(Type::Nil),
            _ => {
                if other.matches(self, checker) {
                    Some(other.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn matches(&self, other: &Type, checker: &Typechecker) -> bool {
        let main = self.skip_downwards(checker);
        let other = other.skip_downwards(checker);
        match (main, other) {
            (Type::Nil, Type::Void) => true,
            (Type::Number, Type::Number) => true,
            (Type::Variable(a), Type::Variable(b)) => a == b,
            (Type::Void, Type::Nil) => true,
            _ => false,
        }
    }

    pub fn description(&self, checker: &Typechecker) -> String {
        match self {
            Type::Invalid => "invalid".to_string(),
            Type::Nil => "nil".to_string(),
            Type::Number => "number".to_string(),
            Type::Variable(id) => checker.variables.get(*id).unwrap().name.to_string(),
            Type::Void => "void".to_string(),
            Type::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableKind {
    Variable,
    Type(),
}

impl VariableKind {
    pub fn is_type(&self) -> bool {
        matches!(self, VariableKind::Type(..))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub definitions: Vec<ast::Span>,
    pub kind: VariableKind,
    pub name: String,
    pub shadowed: Option<Id<Variable>>,
    pub ty: Type,
}
