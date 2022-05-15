#![allow(clippy::large_enum_variant)]
#![allow(clippy::new_without_default)]
#![allow(clippy::or_fun_call)]
#![allow(dead_code)]

mod context;
mod diagnostics;
mod resolver;
mod transformer;
mod typeckr;

pub mod hir;
pub use context::*;
pub use diagnostics::*;
pub use resolver::*;
pub use transformer::*;
pub use typeckr::*;
pub mod types;
