use super::*;
use log::SetLoggerError;

pub fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(message) => message.to_string(),
            None => match info.payload().downcast_ref::<String>() {
                Some(message) => message.clone(),
                None => "<no message>".to_string(),
            },
        };

        log::error!("Salite CLI crashed!");
        log::error!("This may be a bug.");
        log::error!("");
        log::error!("Please report a issue on GitHub at https://github.com/memothelemo/salight");
        log::error!("Details: {}", message);
        log::error!("");

        if let Some(location) = info.location() {
            log::error!(
                "in line {} on line {} column {}",
                location.file(),
                location.line(),
                location.column()
            );
        }

        // When using the backtrace crate, we need to check the RUST_BACKTRACE
        // environment variable ourselves. Once we switch to the (currently
        // unstable) std::backtrace module, we won't need to do this anymore.
        let should_backtrace = std::env::var("RUST_BACKTRACE")
            .map(|var| var == "1")
            .unwrap_or(false);

        if should_backtrace {
            eprintln!("{}", std::backtrace::Backtrace::capture());
        } else {
            eprintln!(
                "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace."
            );
        }
    }));
}

pub fn set_max_level(manual_verbose: bool) {
    log::set_max_level({
        if cfg!(debug_assertions) || manual_verbose {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Warn
        }
    });
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&logger::CLILogger)?;
    set_max_level(false);
    Ok(())
}
