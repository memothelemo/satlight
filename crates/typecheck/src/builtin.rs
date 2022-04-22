use crate::*;

pub trait LanguageBuiltin {
    fn add_builtin_types(&mut self, scope: &mut Scope, storage: &mut SymbolStorage);
}

#[derive(Debug)]
pub struct LunarBuiltin;

impl LanguageBuiltin for LunarBuiltin {
    fn add_builtin_types(&mut self, scope: &mut Scope, storage: &mut SymbolStorage) {
        macro_rules! declare_lazy {
            { $( $ty_name:expr => $ty:expr, )* } => {
                $( scope.try_declare($ty_name, $ty, storage); )*
            };
        }
        declare_lazy! {
            "boolean" => SymbolTyp::Type(Typ::Bool),
            "number" => SymbolTyp::Type(Typ::Number),
            "nil" => SymbolTyp::Type(Typ::Nil),
            "string" => SymbolTyp::Type(Typ::String),
            "unknown" => SymbolTyp::Type(Typ::Unknown),
        };
    }
}
