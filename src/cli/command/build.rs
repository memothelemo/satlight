use anyhow::Context;

use super::*;
use salite::{
    ast::Position,
    checker::{Analyzer, EnvContext, Resolver},
    common::*,
};

pub fn command_build(path: PathBuf, current_dir: bool) -> Result<(), anyhow::Error> {
    log::debug!("Initial directory: {}", path.to_string_lossy());
    log::info!("Initializing project");

    let mut project = salite::env::project::from_dir(&path).with_context(|| {
        format!(
            "Failed to load project from the {}",
            if current_dir {
                "current directory".to_string()
            } else {
                path.to_string_lossy().to_string()
            }
        )
    })?;

    project
        .reload()
        .with_context(|| "Failed to reload project")?;

    let files = salite::env::parse_project(&project).map_err(|e| {
        anyhow::anyhow!("There are parse errors in the following:\n{}", {
            let mut list = Vec::new();
            for err in e.iter() {
                list.push(err.to_string());
            }
            list.join("\n")
        })
    })?;

    let mut env = project.check(&files);
    let env_ptr = memory::SafePtr::from_ptr(&mut env as *mut EnvContext);

    for (file_path, module) in env.modules_mut().iter_mut() {
        Resolver::from_result(module, env_ptr.clone())
            .map_err(|e| {
                anyhow::anyhow!(
                    "{}: {}",
                    Position::from_offset(
                        e.span().start,
                        &project.get_source_code(file_path).unwrap()
                    ),
                    e
                )
            })
            .with_context(|| format!("Failed to check {}", file_path.to_string_lossy()))?;
    }

    for (file_path, module) in env.modules().iter() {
        Analyzer::analyze(module.ctx.clone(), &module.file)
            .map_err(|e| {
                anyhow::anyhow!(
                    "{}: {}",
                    Position::from_offset(
                        e.span().start,
                        &project.get_source_code(file_path).unwrap()
                    ),
                    e
                )
            })
            .with_context(|| format!("Failed to check {}", file_path.to_string_lossy()))?;
    }

    log::info!("Done");

    Ok(())
}
