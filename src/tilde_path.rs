use std::{
    fmt,
    path::{self, Path, PathBuf},
};

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub(crate) struct TildePath {
    original: PathBuf,
    expanded: Option<PathBuf>,
}

impl fmt::Display for TildePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.display().fmt(f)
    }
}

impl AsRef<Path> for TildePath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl From<TildePath> for PathBuf {
    fn from(path: TildePath) -> Self {
        path.as_path().to_owned()
    }
}

impl From<&TildePath> for PathBuf {
    fn from(path: &TildePath) -> Self {
        path.as_path().to_owned()
    }
}

impl<'de> Deserialize<'de> for TildePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        Ok(Self::new(path))
    }
}

impl Serialize for TildePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.original.serialize(serializer)
    }
}

impl TildePath {
    pub(crate) fn new(original: impl Into<PathBuf>) -> Self {
        let original = original.into();
        // replace first tilde only
        let expanded = original.strip_prefix("~").ok().and_then(|path| {
            // if no valid home path could be retrieved from the operating system, don't expand the tilde
            BaseDirs::new().map(|base_dirs| base_dirs.home_dir().join(path))
        });
        Self { original, expanded }
    }

    pub(crate) fn from_expanded(expanded: impl Into<PathBuf>) -> Self {
        let expanded = expanded.into();
        let original = BaseDirs::new()
            .and_then(|base_dirs| expanded.strip_prefix(base_dirs.home_dir()).ok())
            .map(|rest| Path::new("~").join(rest))
            .unwrap_or_else(|| expanded.clone());
        let expanded = (original != expanded).then(|| expanded);
        Self { original, expanded }
    }

    pub(crate) fn new_verbatim(path: impl Into<PathBuf>) -> Self {
        let original = path.into();
        let expanded = None;
        Self { original, expanded }
    }

    pub(crate) fn as_path(&self) -> &Path {
        self.expanded.as_deref().unwrap_or(&self.original)
    }

    pub(crate) fn as_display_path(&self) -> &Path {
        self.original.as_ref()
    }
}

pub(crate) trait PathLike {
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

impl PathLike for TildePath {
    fn as_display_path(&self) -> &Path {
        self.as_display_path()
    }

    fn as_path(&self) -> &Path {
        self.as_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand() {
        let home = BaseDirs::new().unwrap().home_dir().to_owned();
        assert!(home.is_absolute());

        // expand first tilde component
        let path = TildePath::new("~/.config/souko");
        assert_eq!(path.to_string(), "~/.config/souko");
        assert_eq!(
            path.as_ref().to_string_lossy(),
            home.join(".config/souko").to_string_lossy()
        );

        // don't expand tilde+username component
        let path = TildePath::new("~foo/.config/souko");
        assert_eq!(path.to_string(), "~foo/.config/souko");
        assert_eq!(path.as_ref().to_string_lossy(), "~foo/.config/souko");

        // don't expand non-first tilde component
        let path = TildePath::new("/foo/~/baz");
        assert_eq!(path.to_string(), "/foo/~/baz");
        assert_eq!(path.as_ref().to_string_lossy(), "/foo/~/baz");
    }
}
