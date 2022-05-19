#![allow(clippy::or_fun_call)]
#![feature(backtrace)]

use clap::Parser;

mod command;
mod logger;
mod preqs;

fn run_process() -> anyhow::Result<()> {
    let cmd = command::Command::parse();
    preqs::set_max_level(cmd.is_verbose_enabled());
    match cmd {
        command::Command::Build { path, .. } => {
            // maybe set the current directory as the default
            let is_current_dir = path.is_none();
            let path = path
                .unwrap_or(std::env::current_dir().expect("Failed to get the current directory"));

            command::command_build(path, is_current_dir)
        }
    }
}

fn main() -> anyhow::Result<()> {
    preqs::init_logger().expect("Failed to run logger");
    preqs::set_panic_hook();

    run_process()
}

// #![feature(result_option_inspect)]
// #![feature(ptr_const_cast)]

// use std::env;

// use anyhow::{Context, Result};
// use log::SetLoggerError;
// use salite::{
//     ast::Position,
//     checker::{Analyzer, EnvContext, Resolver},
//     common::memory::SafePtr,
// };

// use clap::{Args, Parser, SubCommand};

// mod logger;

// #[derive(Parser, Debug)]
// pub enum Command {}

// fn compile() -> Result<()> {
//     log::info!("Initializing project");

//     let now = std::time::Instant::now();
//     let mut project = salite::env::project::from_dir(".").with_context(|| {
//         format!(
//             "Failed to load project from current directory: {}",
//             std::env::current_dir().unwrap().to_string_lossy()
//         )
//     })?;

//     project
//         .reload()
//         .with_context(|| "Failed to reload project")?;

//     #[allow(unused)]
//     let mut files = match salite::env::parse_project(&project) {
//         Ok(files) => files,
//         Err(errors) => {
//             let elapsed = now.elapsed();
//             log::info!("Took to initialize project: {:.2?}", elapsed);

//             for err in errors.iter() {
//                 eprintln!("{}", err);
//             }

//             std::process::exit(1);
//         }
//     };

//     let elapsed = now.elapsed();
//     log::info!("Took to initialize project: {:.2?}", elapsed);

//     let mut env = project.check(&files);
//     let env_ptr = SafePtr::from_ptr(&mut env as *mut EnvContext);

//     let now = std::time::Instant::now();
//     #[allow(unused_must_use)]
//     for (file_path, module) in env.modules_mut().iter_mut() {
//         Resolver::from_result(module, env_ptr.clone()).inspect_err(|e| {
//             println!(
//                 "{}:{}: {}",
//                 file_path.to_string_lossy(),
//                 Position::from_offset(e.span().start, &project.get_source_code(file_path).unwrap()),
//                 e
//             );
//         });
//     }

//     let elapsed = now.elapsed();
//     log::info!("Took to resolve entire project source: {:.2?}", elapsed);

//     let now = std::time::Instant::now();
//     #[allow(unused_must_use)]
//     for (file_path, module) in env.modules().iter() {
//         Analyzer::analyze(module.ctx.clone(), &module.file).inspect_err(|e| {
//             println!(
//                 "{}:{}: {}",
//                 file_path.to_string_lossy(),
//                 Position::from_offset(e.span().start, &project.get_source_code(file_path).unwrap()),
//                 e
//             );
//         });
//     }
//     let elapsed = now.elapsed();

//     log::info!("Took to resolve analyze project source: {:.2?}", elapsed);

//     Ok(())
// }

// fn main() -> Result<()> {
//     init_logger().with_context(|| "Failed to initialize logger")?;
//     compile()
// }

use std::panic;
