use std::path::{Path, PathBuf};

use super::root::Root;

#[derive(Debug, Clone)]
pub(crate) struct Repo {
    relative_path: PathBuf,
    display_absolute_path: PathBuf,
    absolute_path: PathBuf,
    bare: bool,
}

impl Repo {
    pub(crate) fn from_relative_path(root: &Root, relative_path: PathBuf, bare: bool) -> Self {
        let display_absolute_path = root.display_path().join(&relative_path);
        let absolute_path = root.absolute_path().join(&relative_path);

        Self {
            relative_path,
            display_absolute_path,
            absolute_path,
            bare,
        }
    }

    pub(crate) fn display_absolute_path(&self) -> &Path {
        &self.display_absolute_path
    }

    pub(crate) fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub(crate) fn bare(&self) -> bool {
        self.bare
    }
}
