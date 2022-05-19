#![feature(result_option_inspect)]
#![feature(ptr_const_cast)]

use std::env;

use anyhow::{Context, Result};
use log::SetLoggerError;
use salite::{
    ast::Position,
    checker::{Analyzer, EnvContext, Resolver},
    common::memory::SafePtr,
};

mod logger;

fn compile() -> Result<()> {
    log::info!("Initializing project");

    let now = std::time::Instant::now();
    let mut project = salite::env::project::from_dir(".").with_context(|| {
        format!(
            "Failed to load project from current directory: {}",
            std::env::current_dir().unwrap().to_string_lossy()
        )
    })?;

    project
        .reload()
        .with_context(|| "Failed to reload project")?;

    #[allow(unused)]
    let mut files = match salite::env::parse_project(&project) {
        Ok(files) => files,
        Err(errors) => {
            let elapsed = now.elapsed();
            log::info!("Took to initialize project: {:.2?}", elapsed);

            for err in errors.iter() {
                eprintln!("{}", err);
            }

            std::process::exit(1);
        }
    };

    let elapsed = now.elapsed();
    log::info!("Took to initialize project: {:.2?}", elapsed);

    let mut env = project.check(&files);
    let env_ptr = SafePtr::from_ptr(&mut env as *mut EnvContext);

    let now = std::time::Instant::now();
    #[allow(unused_must_use)]
    for (file_path, module) in env.modules_mut().iter_mut() {
        Resolver::from_result(module, env_ptr.clone()).inspect_err(|e| {
            println!(
                "{}:{}: {}",
                file_path.to_string_lossy(),
                Position::from_offset(e.span().start, &project.get_source_code(file_path).unwrap()),
                e
            );
        });
    }

    let elapsed = now.elapsed();
    log::info!("Took to resolve entire project source: {:.2?}", elapsed);

    let now = std::time::Instant::now();
    #[allow(unused_must_use)]
    for (file_path, module) in env.modules().iter() {
        Analyzer::analyze(module.ctx.clone(), &module.file).inspect_err(|e| {
            println!(
                "{}:{}: {}",
                file_path.to_string_lossy(),
                Position::from_offset(e.span().start, &project.get_source_code(file_path).unwrap()),
                e
            );
        });
    }
    let elapsed = now.elapsed();

    log::info!("Took to resolve analyze project source: {:.2?}", elapsed);

    Ok(())
}

fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&logger::CLILogger)?;

    let debugger_mode = {
        let mut has_verbose = false;
        for arg in env::args() {
            if arg == "--verbose" {
                has_verbose = true;
                break;
            }
        }
        has_verbose
    };

    log::set_max_level({
        if cfg!(debug_assertions) || debugger_mode {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Warn
        }
    });
    Ok(())
}

fn main() -> Result<()> {
    init_logger().with_context(|| "Failed to initialize logger")?;
    compile()
}
