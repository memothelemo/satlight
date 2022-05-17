#![allow(clippy::large_enum_variant)]
#![allow(clippy::new_without_default)]
#![allow(clippy::or_fun_call)]
#![allow(dead_code)]
#![feature(ptr_const_cast)]

mod analyzer;
mod context;
mod diagnostics;
mod resolver;
mod transformer;
mod utils;

pub mod hir;
pub use analyzer::*;
pub use context::*;
pub use diagnostics::*;
pub use resolver::*;
pub use transformer::*;
pub mod types;
pub use utils::*;
