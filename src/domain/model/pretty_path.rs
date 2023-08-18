use std::path::{Path, PathBuf};

use super::path_like::PathLike;

#[derive(Debug, Default, Clone)]
pub(crate) struct PrettyPath {
    real_path: PathBuf,
    display_path: PathBuf,
}

impl PrettyPath {
    pub(crate) fn from_real_path(real_path: PathBuf) -> Self {
        let display_path = real_path.clone();
        Self {
            real_path,
            display_path,
        }
    }

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

impl PathLike for PrettyPath {
    fn as_display_path(&self) -> &Path {
        &self.display_path
    }

    fn as_real_path(&self) -> &Path {
        &self.real_path
    }
}
