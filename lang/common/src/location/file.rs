use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

#[derive(Clone, PartialEq, Hash)]
pub struct FilePath(pub(crate) String);

impl FilePath {
    #[inline]
    pub fn new(str: &str) -> Self {
        str.into()
    }

    pub fn get(&self) -> &String {
        &self.0
    }

    pub fn as_path_buf(&self) -> PathBuf {
        PathBuf::from(self.0.to_string())
    }
}

impl Eq for FilePath {}

impl std::fmt::Debug for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.escape_default().to_string().fmt(f)
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&Path> for FilePath {
    fn from(p: &Path) -> Self {
        FilePath(format!("{}", p.to_string_lossy()))
    }
}

impl From<PathBuf> for FilePath {
    fn from(buf: PathBuf) -> Self {
        FilePath(format!("{}", buf.to_string_lossy()))
    }
}

impl From<&str> for FilePath {
    fn from(s: &str) -> Self {
        FilePath(s.to_string())
    }
}

impl From<String> for FilePath {
    fn from(p: String) -> Self {
        FilePath(p)
    }
}

#[derive(Clone)]
pub struct FileInfo {
    pub path: FilePath,
    pub buf: String,
}

impl std::fmt::Debug for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"SourceFile("{}")"#, self.path)
    }
}

impl FileInfo {
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self, io::Error> {
        let mut source = File::open(&path)?;
        let mut buf = String::new();
        source.read_to_string(&mut buf)?;
        drop(source);

        Ok(FileInfo {
            path: format!("{}", path.as_ref().to_string_lossy()).into(),
            buf,
        })
    }
}
