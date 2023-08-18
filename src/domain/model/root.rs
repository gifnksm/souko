use std::path::{Path, PathBuf};

use super::pretty_path::PrettyPath;

#[derive(Debug, Clone)]
pub(crate) struct Root {
    name: String,
    path: PrettyPath,
}

impl Root {
    pub(crate) fn new(name: String, path: PrettyPath) -> Self {
        Self { name, path }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn path(&self) -> &PrettyPath {
        &self.path
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CanonicalRoot {
    inner: Root,
    canonical_path: PathBuf,
}

impl CanonicalRoot {
    pub(crate) fn new(root: Root, canonical_path: PathBuf) -> Self {
        Self {
            inner: root,
            canonical_path,
        }
    }

    pub(crate) fn as_root(&self) -> &Root {
        &self.inner
    }

    pub(crate) fn name(&self) -> &str {
        self.inner.name()
    }

    pub(crate) fn path(&self) -> &PrettyPath {
        self.inner.path()
    }

    pub(crate) fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }
}
