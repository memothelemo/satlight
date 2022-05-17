#![warn(missing_docs)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::new_without_default)]
#![allow(clippy::or_fun_call)]
#![allow(dead_code)]
#![feature(ptr_const_cast)]

//! # Salite
//!
//! An experimental language derived from Lua.

mod prelude;

/// Compiler environment for the language.
pub mod env;
pub use salitescript::*;
