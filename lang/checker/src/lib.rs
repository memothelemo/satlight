#![allow(clippy::new_without_default)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::large_enum_variant)]

pub mod analyzer;
pub mod binder;
pub mod hir;
pub mod types;

pub mod meta {
    #![allow(non_upper_case_globals)]

    #[allow(unused)]
    use crate::analyzer::Validate;

    use super::*;
    use analyzer::AnalyzeError;
    use bitflags::bitflags;
    use lazy_static::lazy_static;
    use salite_common::dictionary::Dictionary;

    bitflags! {
        pub struct AcceptedType: u32 {
            const Function = 0b0000001;
            const Table = 0b0000010;
        }
    }

    lazy_static! {
        static ref METAMETHODS_LIST: Vec<(&'static str, AcceptedType)> = vec![
            // arithmetic
            ("__add", AcceptedType::Function),
            ("__sub", AcceptedType::Function),
            ("__mul", AcceptedType::Function),
            ("__div", AcceptedType::Function),
            ("__mod", AcceptedType::Function),
            ("__unm", AcceptedType::Function),
            ("__concat", AcceptedType::Function),
            ("__eq", AcceptedType::Function),
            ("__lt", AcceptedType::Function),
            ("__le", AcceptedType::Function),

            // abstract object stuff
            ("__call", AcceptedType::Function),
            ("__tostring", AcceptedType::Function),

            // indexing
            ("__index", AcceptedType::Function | AcceptedType::Table),
            ("__newindex", AcceptedType::Function)
        ];

        static ref METAMETHODS: Dictionary<String, AcceptedType> = {
            let mut dictionary = Dictionary::new();
            for (name, accepted) in METAMETHODS.iter() {
                dictionary.insert(name.to_string(), *accepted);
            }
            dictionary
        };
    }

    /// Lua standard metatable checker with Checker trait implemented
    pub struct Standard;

    impl Checker for Standard {
        #[allow(unused)]
        fn check(
            analyzer: &mut analyzer::Analyzer<'_>,
            metatable: &types::Table,
        ) -> Result<(), AnalyzeError> {
            todo!()
        }
    }

    /// This trait allows to check the entire metatable of Lua.
    ///
    /// You can customize this and make your own set of rules
    /// of using the metatable.
    pub trait Checker {
        fn check(
            analyzer: &mut analyzer::Analyzer<'_>,
            metatable: &types::Table,
        ) -> Result<(), AnalyzeError>;
    }
}
