use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FindPathError {
    Io(std::io::Error),
    Var(std::env::VarError),
}

impl std::error::Error for FindPathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FindPathError::Io(e) => Some(e),
            FindPathError::Var(e) => Some(e),
        }
    }
}

impl Display for FindPathError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.source().unwrap(), f)
    }
}

impl From<std::io::Error> for FindPathError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::env::VarError> for FindPathError {
    fn from(e: std::env::VarError) -> Self {
        Self::Var(e)
    }
}

/// Returns path to store settings
pub fn default_settings_path() -> Result<PathBuf, FindPathError> {
    let path: PathBuf = std::env::var("APPDATA")?.into();
    if !path.exists() {
        let error = std::io::Error::new(
            ErrorKind::NotFound,
            format!("Path {:?} does not exists", path),
        );
        return Err(error.into());
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_found_and_exist() {
        let path = default_settings_path().unwrap();
        assert!(path.exists());
    }
}
