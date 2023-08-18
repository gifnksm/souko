use std::path::{Path, PathBuf};

use super::{display_path::DisplayPath, query::Query, root::Root};

#[derive(Debug, Clone)]
pub(crate) struct Repo {
    relative_path: DisplayPath,
    path: DisplayPath,
    bare: bool,
}

impl Repo {
    pub(crate) fn from_relative_path(root: &Root, relative_path: DisplayPath, bare: bool) -> Self {
        let path = root.path().join(&relative_path);
        Self {
            relative_path,
            path,
            bare,
        }
    }

    pub(crate) fn from_query(root: &Root, query: &Query, bare: bool) -> Self {
        let mut relative_path = PathBuf::new();
        let url = query.url();
        if let Some(host) = url.host_str() {
            if let Some(port) = url.port() {
                relative_path.push(format!("{host}:{port}"))
            } else {
                relative_path.push(host);
            }
        }

        let mut base_url = url.clone();
        base_url.set_path("");
        if let Some(relative) = base_url.make_relative(url) {
            let mut relative = relative.as_str();
            if !bare {
                relative = relative.trim_end_matches(".git");
            }
            for part in relative.split('/') {
                relative_path.push(part);
            }
        }

        let relative_path = DisplayPath::from_real_path(relative_path);
        Self::from_relative_path(root, relative_path, bare)
    }

    pub(crate) fn relative_path(&self) -> &DisplayPath {
        &self.relative_path
    }

    pub(crate) fn path(&self) -> &DisplayPath {
        &self.path
    }

    pub(crate) fn bare(&self) -> bool {
        self.bare
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CanonicalRepo {
    inner: Repo,
    canonical_path: PathBuf,
}

impl CanonicalRepo {
    pub(crate) fn new(repo: Repo, canonical_path: PathBuf) -> Self {
        Self {
            inner: repo,
            canonical_path,
        }
    }

    pub(crate) fn relative_path(&self) -> &DisplayPath {
        self.inner.relative_path()
    }

    pub(crate) fn path(&self) -> &DisplayPath {
        self.inner.path()
    }

    pub(crate) fn bare(&self) -> bool {
        self.inner.bare()
    }

    pub(crate) fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::domain::model::{display_path::DisplayPath, path_like::PathLike, query};

    #[test]
    fn from_query() {
        let root_path = PathBuf::from("/home/user/test");
        let root_display_path = PathBuf::from("~/test");
        let root = Root::new(
            "test".into(),
            DisplayPath::from_pair(root_path, root_display_path),
        );

        let parse_option = query::ParseOption::default();

        let pairs = [
            ("https://github.com/owner/repo.git", "github.com/owner/repo"),
            (
                "https://github.com/owner///repo.git",
                "github.com/owner/repo",
            ),
            (
                "https://github.com:443/owner/repo.git",
                "github.com/owner/repo",
            ),
            (
                "https://owner@example.com:50443/repo.git",
                "example.com:50443/repo",
            ),
            (
                "https://owner:password@example.com:50443/repo.gitx",
                "example.com:50443/repo.gitx",
            ),
            (
                "ssh://git@github.com:22/owner/repo.git",
                "github.com/owner/repo",
            ),
        ];

        for (url_str, path_str) in pairs {
            let query = Query::parse(url_str, &parse_option).unwrap();
            let repo = Repo::from_query(&root, &query, false);
            assert_eq!(repo.relative_path().as_real_path(), Path::new(path_str));
        }
    }
}
