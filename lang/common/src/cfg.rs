use salite_macros::FieldCall;
use serde::Deserialize;
use std::path::{self, PathBuf};
use thiserror::Error;

/// Errors given when doing something with Config struct.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// This error caused by an IO error.
    #[error("{0}")]
    IO(std::io::Error),

    /// This error caused by a config parse error
    #[error("Failed to parse config: {0}")]
    Parse(serde_json::Error),
}

/// Compiler configuration contents for the Salite project object.
#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct ConfigInfo {
    /// The entire directory must be mostly filled with
    /// Salite source files
    #[serde(rename = "sourceDir")]
    pub source_dir: PathBuf,

    /// Output Lua files compiled from Salite source files
    #[serde(rename = "outDir")]
    pub output_dir: PathBuf,
}

impl ConfigInfo {
    /// Parses any string into ConfigInfo object (if it parses successfully)
    pub fn parse(contents: &str) -> Result<ConfigInfo, serde_json::Error> {
        serde_json::from_str(contents)
    }
}

/// Compiler configuration for the Salite project object.
#[derive(FieldCall)]
pub struct Config {
    /// The contents of the configuration file itself
    #[exclude]
    info: ConfigInfo,

    /// The location of the configuration file
    path: Option<PathBuf>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("info", &self.info)
            .field(
                "path",
                &(match &self.path {
                    Some(p) => format!("{}", p.to_string_lossy()),
                    None => "No path".to_string(),
                }),
            )
            .finish()
    }
}

impl Config {
    /// Creates a new Config object
    pub fn new(info: ConfigInfo, path: PathBuf) -> Self {
        Config {
            info,
            path: Some(path),
        }
    }

    /// Creates a new Config object without any path source involved
    pub fn no_file(info: ConfigInfo) -> Self {
        Config { info, path: None }
    }

    /// Loads a file and tries to parse that file to get the output Config object.
    pub fn load_file<T: AsRef<path::Path>>(path: T) -> Result<Self, ConfigError> {
        let contents = std::fs::read_to_string(&path).map_err(ConfigError::IO)?;
        let contents = ConfigInfo::parse(&contents).map_err(ConfigError::Parse)?;
        let path = std::fs::canonicalize(path).map_err(ConfigError::IO)?;
        Ok(Config::new(contents, path))
    }

    /// Gets the config info from the object.
    pub fn get(&self) -> &ConfigInfo {
        &self.info
    }
}
