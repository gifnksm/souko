use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub(crate) struct Repo {
    name: String,
    relative_path: PathBuf,
    absolute_path: PathBuf,
    is_bare: bool,
}

impl Repo {
    pub(crate) fn new(
        name: String,
        relative_path: PathBuf,
        absolute_path: PathBuf,
        is_bare: bool,
    ) -> Self {
        Self {
            name,
            relative_path,
            absolute_path,
            is_bare,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub(crate) fn is_bare(&self) -> bool {
        self.is_bare
    }
}
