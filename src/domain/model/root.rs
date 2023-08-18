use super::{display_path::DisplayPath, path_like::PathLike};

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
    path: DisplayPath,
}

impl Root {
    pub(crate) fn new(name: String, path: DisplayPath) -> Self {
        Self { name, path }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn path(&self) -> &DisplayPath {
        &self.path
    }
}
