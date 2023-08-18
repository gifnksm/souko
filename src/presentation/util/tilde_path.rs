use std::{
    convert::Infallible,
    path::{Path, PathBuf},
};

use directories::BaseDirs;
use serde::Deserialize;

use crate::domain::model::{display_path::DisplayPath, path_like::PathLike};

#[derive(Debug, Default, Clone)]
pub(in super::super) struct TildePath(DisplayPath);

impl From<TildePath> for DisplayPath {
    fn from(path: TildePath) -> Self {
        path.0
    }
}

impl From<&TildePath> for DisplayPath {
    fn from(path: &TildePath) -> Self {
        path.0.clone()
    }
}

impl<'de> Deserialize<'de> for TildePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        Ok(Self::from_display_path(path))
    }
}

impl PathLike for TildePath {
    fn as_real_path(&self) -> &Path {
        self.0.as_real_path()
    }

    fn as_display_path(&self) -> &Path {
        self.0.as_display_path()
    }
}

impl TildePath {
    pub(in super::super) fn from_display_path(display_path: PathBuf) -> Self {
        // replace first tilde only
        let real_path = display_path
            .strip_prefix("~")
            .ok()
            .and_then(|path| {
                // if no valid home path could be retrieved from the operating system, don't expand the tilde
                BaseDirs::new().map(|base_dirs| base_dirs.home_dir().join(path))
            })
            .unwrap_or_else(|| display_path.clone());

        let path = DisplayPath::from_pair(real_path, display_path);

        Self(path)
    }

    pub(in super::super) fn from_real_path(real_path: PathBuf) -> Self {
        let display_path = BaseDirs::new()
            .and_then(|base_dirs| real_path.strip_prefix(base_dirs.home_dir()).ok())
            .map(|rest| Path::new("~").join(rest))
            .unwrap_or_else(|| real_path.clone());

        let path = DisplayPath::from_pair(real_path, display_path);
        Self(path)
    }

    pub(in super::super) fn parse_real_path(real_path: &str) -> Result<Self, Infallible> {
        Ok(Self::from_real_path(real_path.into()))
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
        let path = TildePath::from_display_path("~/.config/souko".into());
        assert_eq!(path.display().to_string(), "~/.config/souko");
        assert_eq!(
            path.as_real_path().to_string_lossy(),
            home.join(".config/souko").to_string_lossy()
        );

        // don't expand tilde+username component
        let path = TildePath::from_display_path("~foo/.config/souko".into());
        assert_eq!(path.display().to_string(), "~foo/.config/souko");
        assert_eq!(path.as_real_path().to_string_lossy(), "~foo/.config/souko");

        // don't expand non-first tilde component
        let path = TildePath::from_display_path("/foo/~/baz".into());
        assert_eq!(path.display().to_string(), "/foo/~/baz");
        assert_eq!(path.as_real_path().to_string_lossy(), "/foo/~/baz");
    }
}
