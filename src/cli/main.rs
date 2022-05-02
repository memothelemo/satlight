use anyhow::{anyhow, Context, Result};
use lunar::ast::Position;

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let mut project = lunar::env::project::from_dir(".").with_context(|| {
        format!(
            "Failed to load project from current directory: {}",
            std::env::current_dir().unwrap().to_string_lossy()
        )
    })?;

    project
        .reload()
        .with_context(|| "Failed to reload project")?;

    #[allow(unused)]
    let files = match lunar::env::parse_project(&project) {
        Ok(files) => files,
        Err(errors) => {
            let elapsed = now.elapsed();
            println!("Took to initialize project: {:.2?}", elapsed);

            for err in errors.iter() {
                eprintln!("{}", err);
            }

            std::process::exit(1);
        }
    };

    let elapsed = now.elapsed();
    println!("Took to initialize project: {:.2?}", elapsed);

    for (file_path, file) in files.into_iter() {
        use lunar::checker::{analyzer, binder};
        println!("Checking file: {}", file_path.to_string_lossy());

        let now = std::time::Instant::now();
        let (binder, block) = binder::Binder::new(&file);
        let elapsed = now.elapsed();
        println!("Took to bind the source file: {:.2?}", elapsed);

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
        println!("Took to typecheck file: {:.2?}", elapsed);
    }

    Ok(())
}
