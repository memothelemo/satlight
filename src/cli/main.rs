use std::env;

use anyhow::{anyhow, Context, Result};
use log::SetLoggerError;
use salite::ast::Position;

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

    for (file_path, file) in files.into_iter() {
        use salite::checker::{analyzer, binder};
        log::info!("Checking file: {}", file_path.to_string_lossy());

        let now = std::time::Instant::now();
        let (binder, block) = binder::Binder::new(&file);
        let elapsed = now.elapsed();
        log::debug!("Took to bind the source file: {:.2?}", elapsed);

        let now = std::time::Instant::now();
        analyzer::Analyzer::analyze(&binder, project.config(), &block).map_err(|err| {
            anyhow!(format!(
                "{}:{}: {}",
                file_path.to_string_lossy(),
                Position::from_offset(
                    err.span().start,
                    &project.get_source_code(&file_path).unwrap()
                ),
                err
            ))
        })?;
        let elapsed = now.elapsed();
        log::debug!("Took to typecheck file: {:.2?}", elapsed);
    }

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
