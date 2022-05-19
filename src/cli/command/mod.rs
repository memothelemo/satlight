//use super::preqs;
use clap::Parser;
use std::path::PathBuf;

mod build;

pub use build::*;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[clap(version)]
pub enum Command {
    Build {
        path: Option<PathBuf>,
        #[clap(long, short)]
        verbose: bool,
    },
}

impl Command {
    pub fn is_verbose_enabled(&self) -> bool {
        match self {
            Command::Build { verbose, .. } => *verbose,
        }
    }
}
