#![warn(missing_docs)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::new_without_default)]
#![allow(dead_code)]

//! # Lunar
//!
//! An experimental language derived from Lua.

mod prelude;

/// Compiler environment for the language.
pub mod env;
pub use lunarscript::*;
