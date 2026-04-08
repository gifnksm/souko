use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;

use crate::domain::model::pretty_path::PrettyPath;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(transparent)]
pub(in super::super) struct UnresolvedPath(PathBuf);

// TODO: Replace this polyfill with `Path::trim_trailing_sep` once it becomes
// available on stable Rust (rust-lang/rust#142503).
fn normalize_trailing_separator(path: &Path) -> PathBuf {
    fn is_sep_byte(byte: u8) -> bool {
        if cfg!(windows) {
            byte == b'/' || byte == b'\\'
        } else {
            byte == b'/'
        }
    }

    let bytes = path.as_os_str().as_encoded_bytes();
    let has_trailing_sep = bytes.last().copied().is_some_and(is_sep_byte);

    if !has_trailing_sep || (path.has_root() && path.parent().is_none()) {
        return path.to_owned();
    }

    let mut trimmed = bytes;
    while let Some((last, init)) = trimmed.split_last() {
        if !is_sep_byte(*last) {
            break;
        }
        trimmed = init;
    }

    // SAFETY: Trimming trailing ASCII separator bytes preserves the validity of
    // the underlying platform string encoding.
    PathBuf::from(unsafe { OsStr::from_encoded_bytes_unchecked(trimmed) })
}

impl UnresolvedPath {
    pub(in crate::presentation) fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub(in crate::presentation) fn normalize_with_home<P>(&self, home: &P) -> PrettyPath
    where
        P: AsRef<Path> + ?Sized,
    {
        let home = home.as_ref();
        let resolved_path = match self.0.strip_prefix("~") {
            Ok(rest) => home.join(rest),
            Err(_) => self.0.clone(),
        };
        let display_path = match resolved_path.strip_prefix(home) {
            Ok(rest) => Path::new("~").join(rest),
            Err(_) => resolved_path.clone(),
        };
        PrettyPath::from_pair(
            normalize_trailing_separator(&resolved_path),
            normalize_trailing_separator(&display_path),
        )
    }
}

impl FromStr for UnresolvedPath {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PathBuf::from(s)))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::model::path_like::PathLike as _;

    use super::*;

    #[test]
    fn expand() {
        let home = Path::new("/home/foo");

        // expand first tilde component
        let path = UnresolvedPath::new("~/.config/souko".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("~").join(".config/souko"));
        assert_eq!(path.as_real_path(), home.join(".config/souko"));

        // expand bare tilde to home
        let path = UnresolvedPath::new("~".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home);

        // normalize bare tilde with trailing separator to base tilde
        let path = UnresolvedPath::new("~/".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home);

        // don't expand tilde+username component
        let path = UnresolvedPath::new("~foo/.config/souko".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("~foo/.config/souko"));
        assert_eq!(path.as_real_path(), Path::new("~foo/.config/souko"));

        // don't expand non-first tilde component
        let path = UnresolvedPath::new("/foo/~/baz".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("/foo/~/baz"));
        assert_eq!(path.as_real_path(), Path::new("/foo/~/baz"));

        // normalize home itself to base tilde
        let path = UnresolvedPath::new("/home/foo".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home);

        // normalize home itself with trailing separator to base tilde
        let path = UnresolvedPath::new("/home/foo/".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home);

        // don't normalize paths that only share a string prefix with home
        let path = UnresolvedPath::new("/home/foobar/bar".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("/home/foobar/bar"));
        assert_eq!(path.as_real_path(), Path::new("/home/foobar/bar"));

        // normalize a path under home to a tilde-based display path
        let path = UnresolvedPath::new("/home/foo/bar".into()).normalize_with_home(home);
        assert_eq!(path.as_display_path(), Path::new("~").join("bar"));
        assert_eq!(path.as_real_path(), Path::new("/home/foo/bar"));
    }
}
