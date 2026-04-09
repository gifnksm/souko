use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;

use crate::{
    app_dirs::AppDirs,
    domain::model::{path_like::PathLike, pretty_path::PrettyPath},
    presentation::model::app_param::AppParamSource,
};

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

    pub(in crate::presentation) fn normalize(
        &self,
        source: &AppParamSource,
        app_dirs: &AppDirs,
    ) -> PrettyPath {
        let home_dir = app_dirs.home_dir();
        let mut resolved_path = match self.0.strip_prefix("~") {
            Ok(rest) => home_dir.join(rest),
            Err(_) => self.0.clone(),
        };

        if resolved_path.is_relative() {
            match source {
                AppParamSource::CommandLineArgument => {
                    resolved_path = app_dirs.working_dir().join(resolved_path);
                }
                AppParamSource::ConfigurationFile { path } => {
                    let config_dir = path.as_real_path().parent().unwrap();
                    assert!(config_dir.is_absolute());
                    resolved_path = config_dir.join(resolved_path);
                }
                AppParamSource::ImplicitDefault => {
                    // default path values should never be relative, so this is likely a bug
                    panic!(
                        "Unexpected relative path from implicit default source: {:?}",
                        self.0
                    )
                }
            }
        }

        let display_path = match resolved_path.strip_prefix(home_dir) {
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
    fn normalize_tilde_and_home_paths() {
        let app_dirs = AppDirs::new_for_test(env!("CARGO_BIN_NAME"), "/home/foo", "/work").unwrap();
        let home_dir = app_dirs.home_dir();
        let config_dir = UnresolvedPath::new(app_dirs.config_dir().to_owned())
            .normalize(&AppParamSource::ImplicitDefault, &app_dirs);
        let config_path = PrettyPath::from_pair(
            config_dir.as_real_path().join("config.toml"),
            config_dir.as_display_path().join("config.toml"),
        );
        let source = AppParamSource::ConfigurationFile { path: config_path };

        // expand first tilde component
        let path = UnresolvedPath::new("~/.config/souko".into()).normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), Path::new("~/.config/souko"));
        assert_eq!(path.as_real_path(), home_dir.join(".config/souko"));

        // expand bare tilde to home
        let path = UnresolvedPath::new("~".into()).normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home_dir);

        // normalize bare tilde with trailing separator to base tilde
        let path = UnresolvedPath::new("~/".into()).normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home_dir);

        // don't expand tilde+username component; it is treated as a relative path
        let path = UnresolvedPath::new("~foo/.config/souko".into()).normalize(&source, &app_dirs);
        assert_eq!(
            path.as_display_path(),
            config_dir.as_display_path().join("~foo/.config/souko")
        );
        assert_eq!(
            path.as_real_path(),
            config_dir.as_real_path().join("~foo/.config/souko")
        );

        // don't expand non-first tilde component
        let absolute_path = home_dir.parent().unwrap().join("~/baz");
        let path = UnresolvedPath::new(absolute_path.clone()).normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), absolute_path);
        assert_eq!(path.as_real_path(), absolute_path);

        // normalize home itself to base tilde
        let path = UnresolvedPath::new(home_dir.to_owned()).normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home_dir);

        // normalize home itself with trailing separator to base tilde
        let path = UnresolvedPath::new(format!("{}/", home_dir.display()).into())
            .normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), Path::new("~"));
        assert_eq!(path.as_real_path(), home_dir);

        // don't normalize paths that only share a string prefix with home
        let sibling_of_home_dir = home_dir.parent().unwrap().join("foobar");
        let path = UnresolvedPath::new(sibling_of_home_dir.clone()).normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), sibling_of_home_dir);
        assert_eq!(path.as_real_path(), sibling_of_home_dir);

        // normalize a path under home to a tilde-based display path
        let path = UnresolvedPath::new(home_dir.join("bar")).normalize(&source, &app_dirs);
        assert_eq!(path.as_display_path(), Path::new("~/bar"));
        assert_eq!(path.as_real_path(), home_dir.join("bar"));
    }

    #[test]
    fn normalize_resolves_relative_paths_from_configuration_file_directory() {
        let app_dirs = AppDirs::new_for_test(env!("CARGO_BIN_NAME"), "/home/foo", "/work").unwrap();
        let config_dir = UnresolvedPath::new(app_dirs.config_dir().join("nested"))
            .normalize(&AppParamSource::ImplicitDefault, &app_dirs);
        let config_path = PrettyPath::from_pair(
            config_dir.as_real_path().join("config.toml"),
            config_dir.as_display_path().join("config.toml"),
        );
        let source = AppParamSource::ConfigurationFile { path: config_path };

        let path = UnresolvedPath::new("repos/root".into()).normalize(&source, &app_dirs);

        assert_eq!(
            path.as_real_path(),
            config_dir.as_real_path().join("repos/root"),
        );
        assert_eq!(
            path.as_display_path(),
            config_dir.as_display_path().join("repos/root"),
        );
    }

    #[test]
    fn normalize_resolves_relative_paths_from_command_line_working_directory() {
        let app_dirs = AppDirs::new_for_test(env!("CARGO_BIN_NAME"), "/home/foo", "/work").unwrap();
        let source = AppParamSource::CommandLineArgument;

        let path = UnresolvedPath::new("repos/cache.json".into()).normalize(&source, &app_dirs);

        let expected_real_path = app_dirs.working_dir().join("repos/cache.json");
        let expected_display_path = expected_real_path.clone();

        assert_eq!(path.as_real_path(), expected_real_path);
        assert_eq!(path.as_display_path(), expected_display_path);
    }
}
