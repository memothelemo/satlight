use std::env;

use anyhow::{Context, Result};
use log::SetLoggerError;

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
    let files = match salite::env::parse_project(&project) {
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

    #[allow(unused)]
    // let env = project.check(&files);
    // println!("{:#?}", env);
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
