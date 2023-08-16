use std::{
    fmt::Debug,
    path,
    path::{Path, PathBuf},
};

pub(crate) trait PathLike: Debug {
    fn display(&self) -> path::Display<'_> {
        self.as_display_path().display()
    }

    fn as_display_path(&self) -> &Path;
    fn as_path(&self) -> &Path;
}

impl PathLike for PathBuf {
    fn as_display_path(&self) -> &Path {
        self
    }

    fn as_path(&self) -> &Path {
        self
    }
}

impl PathLike for Path {
    fn as_display_path(&self) -> &Path {
        self
    }

    fn as_path(&self) -> &Path {
        self
    }
}

impl<T> PathLike for &T
where
    T: PathLike,
{
    fn as_display_path(&self) -> &Path {
        (*self).as_display_path()
    }

    fn as_path(&self) -> &Path {
        (*self).as_path()
    }
}
