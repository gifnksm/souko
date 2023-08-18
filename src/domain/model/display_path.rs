use std::path::{Path, PathBuf};

use super::path_like::PathLike;

#[derive(Debug, Clone)]
pub(crate) struct DisplayPath {
    real_path: PathBuf,
    display_path: PathBuf,
}

impl DisplayPath {
    pub(crate) fn from_real_path(real_path: PathBuf) -> Self {
        let display_path = real_path.clone();
        Self {
            real_path,
            display_path,
        }
    }

    pub(crate) fn from_pathlike(path: &dyn PathLike) -> Self {
        Self {
            real_path: path.as_real_path().to_owned(),
            display_path: path.as_display_path().to_owned(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_pair(real_path: PathBuf, display_path: PathBuf) -> Self {
        Self {
            real_path,
            display_path,
        }
    }

    pub(crate) fn join(&self, path: &dyn PathLike) -> Self {
        Self {
            real_path: self.real_path.join(path.as_real_path()),
            display_path: self.display_path.join(path.as_display_path()),
        }
    }
}

impl PathLike for DisplayPath {
    fn as_display_path(&self) -> &Path {
        &self.display_path
    }

    fn as_real_path(&self) -> &Path {
        &self.real_path
    }
}
