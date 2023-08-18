use std::{
    fmt::Debug,
    path,
    path::{Path, PathBuf},
};

pub(crate) trait PathLike: Debug {
    fn as_real_path(&self) -> &Path;
    fn as_display_path(&self) -> &Path;

    fn display(&self) -> path::Display<'_> {
        self.as_display_path().display()
    }
}

impl PathLike for PathBuf {
    fn as_real_path(&self) -> &Path {
        self
    }

    fn as_display_path(&self) -> &Path {
        self
    }
}

impl PathLike for Path {
    fn as_real_path(&self) -> &Path {
        self
    }

    fn as_display_path(&self) -> &Path {
        self
    }
}

impl<T> PathLike for &T
where
    T: PathLike + ?Sized,
{
    fn as_real_path(&self) -> &Path {
        (*self).as_real_path()
    }

    fn as_display_path(&self) -> &Path {
        (*self).as_display_path()
    }
}
