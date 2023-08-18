use std::path::{Path, PathBuf};

use super::path_like::PathLike;

#[derive(Debug, Clone)]
pub(crate) struct DisplayPath {
    path: PathBuf,
    display_path: PathBuf,
}

impl DisplayPath {
    pub(crate) fn from_expanded(path: PathBuf) -> Self {
        let display_path = path.clone();
        Self { path, display_path }
    }

    pub(crate) fn from_pathlike(path: &dyn PathLike) -> Self {
        let path = path.as_path().to_owned();
        let display_path = path.as_display_path().to_owned();
        Self { path, display_path }
    }

    pub(crate) fn from_pair(path: PathBuf, display_path: PathBuf) -> Self {
        Self { path, display_path }
    }

    pub(crate) fn join(&self, path: &dyn PathLike) -> Self {
        Self {
            path: self.path.join(path.as_path()),
            display_path: self.display_path.join(path.as_display_path()),
        }
    }
}

impl PathLike for DisplayPath {
    fn as_display_path(&self) -> &Path {
        &self.display_path
    }

    fn as_path(&self) -> &Path {
        &self.path
    }
}
