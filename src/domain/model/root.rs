use std::path::{Path, PathBuf};

use super::path_like::PathLike;

#[derive(Debug)]
pub(crate) struct RootSpec {
    name: String,
    path: Box<dyn PathLike>,
}

impl RootSpec {
    pub(crate) fn new(name: String, path: Box<dyn PathLike>) -> Self {
        Self { name, path }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn path(&self) -> &dyn PathLike {
        &*self.path
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Root {
    name: String,
    display_path: PathBuf,
    absolute_path: PathBuf,
}

impl Root {
    pub(crate) fn new(name: String, display_path: PathBuf, absolute_path: PathBuf) -> Self {
        assert!(absolute_path.is_absolute());
        Self {
            name,
            display_path,
            absolute_path,
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn display_path(&self) -> &Path {
        &self.display_path
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }
}
