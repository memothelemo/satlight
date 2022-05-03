use super::*;

use std::{
    collections::{HashMap, VecDeque},
    path::{self, PathBuf},
};
use thiserror::Error;
use walkdir::WalkDir;

use lunarscript::common::{Config, ConfigError};

/// Errors given when loading or doing something with Lunar project.
#[derive(Debug, Error)]
pub enum ProjectError {
    /// This error caused by a config load error
    #[error("{0}")]
    Config(ConfigError),

    /// This error caused by a directory has no config file
    #[error("A directory has a no config file 'lunarcfg.json'")]
    DirectoryNoConfig,

    /// This error caused by a config file doesn't have a parent to begin with.
    #[error("Config file has no parent directory")]
    ConfigNoParentDir,

    /// This error caused by a source reload error
    #[error("Cannot reload source file {0}: {1}")]
    SourceFileReload(PathBuf, SourceFileReloadError),

    /// This error caused by an IO
    #[error("{0}")]
    IO(std::io::Error),
}

/// The entire Lunar project session. Created by a
/// main project directory that has `lunarcfg.json` file.
#[derive(Debug)]
pub struct Project {
    config: Config,
    files: HashMap<PathBuf, SourceFile>,
    root: PathBuf,
}

impl Project {
    fn new(config: Config, root: PathBuf) -> Self {
        Project {
            config,
            files: HashMap::new(),
            root,
        }
    }

    fn gather_source_file_paths(&self) -> Result<Vec<PathBuf>, ProjectError> {
        // output of source files
        let mut results = Vec::new();

        // combine source directory with root directory
        let src_dir = self.root.join(&self.config.get().source_dir);

        // recursive file search
        for entry in WalkDir::new(src_dir) {
            let entry = entry
                .map_err(|e| e.into_io_error().unwrap())
                .map_err(ProjectError::IO)?;

            // accept if it is a file
            if !entry.file_type().is_file() {
                continue;
            }

            let file_path = entry.path();
            if let Some(file_ext) = file_path.extension() {
                if file_ext == "lun" {
                    results.push(file_path.to_path_buf());
                }
            }
        }

        Ok(results)
    }

    /// Update the entire project with newest changes within the
    /// project root directory.
    pub fn reload(&mut self) -> Result<(), ProjectError> {
        log::debug!("Reloading project {}", self.root().to_string_lossy());

        // gather all of the current source files
        let current_files = self.gather_source_file_paths()?;
        for file_path in current_files.iter() {
            if let Some(source_file) = self.files.get_mut(file_path) {
                source_file.reload().map_err(|e| {
                    ProjectError::SourceFileReload(source_file.path().to_path_buf(), e)
                })?;
            } else {
                self.files.insert(
                    file_path.to_path_buf(),
                    SourceFile::new(file_path).map_err(ProjectError::IO)?,
                );
            }
        }

        // remove leftover files?
        let deleted_files_queue = self
            .files
            .keys()
            .filter(|v| !current_files.contains(v))
            .map(|v| v.to_path_buf())
            .collect::<Vec<PathBuf>>();

        for file_path in deleted_files_queue {
            self.files.remove(&file_path);
        }

        Ok(())
    }

    /// Gets the entire project's config info
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Gets the entire project's source files from 'sourceDir' entry
    pub fn files(&self) -> Vec<&SourceFile> {
        self.files.values().collect::<Vec<&SourceFile>>()
    }

    /// Gets the project's current directory root
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Gets the source code of the specific file path
    pub fn get_source_code(&self, path: &PathBuf) -> Option<String> {
        self.files.get(path).map(|v| v.contents().to_string())
    }
}

/// Loads Lunar project from the config file path
pub fn from_file<T: AsRef<path::Path>>(file: T) -> Result<Project, ProjectError> {
    // load the config file
    let cfg = Config::load_file(&file).map_err(ProjectError::Config)?;

    // resolve the path and get the parent directory of its file
    // this function requires rust 1.5.0
    #[allow(clippy::or_fun_call)]
	#[rustfmt::skip]
	let parent_dir =
		std::fs::canonicalize(file.as_ref().parent().ok_or(ProjectError::DirectoryNoConfig)?).map_err(ProjectError::IO)?;

    Ok(Project::new(cfg, parent_dir))
}

/// Loads Lunar project from the directory path
pub fn from_dir<T: AsRef<path::Path>>(dir: T) -> Result<Project, ProjectError> {
    // look for config file candidates, because sometimes there are many lunarcfg variants
    let candidates = look_cfg_files(&dir).map_err(ProjectError::IO)?;

    // look for the best candidates in a deque vector
    if let Some(cfg_path) = candidates.front() {
        let cfg = Config::load_file(&cfg_path).map_err(ProjectError::Config)?;
        Ok(Project::new(cfg, dir.as_ref().to_path_buf()))
    } else {
        Err(ProjectError::DirectoryNoConfig)
    }
}

/// Looks for files starting with 'lunarcfg' and is JSON. (`lunarcfg.json` file)
pub fn look_cfg_files<T: AsRef<path::Path>>(dir: T) -> Result<VecDeque<PathBuf>, std::io::Error> {
    let mut candidates = VecDeque::new();
    let directory = std::fs::read_dir(&dir)?;
    for entry in directory {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let file_path = entry.path();
        if let Some(file_ext) = file_path.extension() {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            // find a file starts with 'lunarcfg' and its extension is a JSON file
            // example: 'lunarcfg.json' or 'lunarcfg-other.json'
            if file_ext == "json" && file_name.starts_with("lunarcfg") {
                // first priority
                if file_name == "lunarcfg" {
                    candidates.push_back(file_path);
                } else {
                    candidates.push_front(file_path);
                }
            }
        }
    }
    Ok(candidates)
}
