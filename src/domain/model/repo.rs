use std::path::{Path, PathBuf};

use super::{query::Query, root::Root};

#[derive(Debug, Clone)]
pub(crate) struct Repo {
    relative_path: PathBuf,
    display_absolute_path: PathBuf,
    absolute_path: PathBuf,
    bare: bool,
}

impl Repo {
    pub(crate) fn from_relative_path(root: &Root, relative_path: PathBuf, bare: bool) -> Self {
        let display_absolute_path = root.display_path().join(&relative_path);
        let absolute_path = root.absolute_path().join(&relative_path);

        Self {
            relative_path,
            display_absolute_path,
            absolute_path,
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

        Self::from_relative_path(root, relative_path, bare)
    }

    pub(crate) fn display_absolute_path(&self) -> &Path {
        &self.display_absolute_path
    }

    pub(crate) fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub(crate) fn bare(&self) -> bool {
        self.bare
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::query;

    #[test]
    fn from_query() {
        let root_display_path = "~/test".into();
        #[cfg(not(windows))]
        let root_absolute_path = "/home/user/test".into();
        #[cfg(windows)]
        let root_absolute_path = r"c:/Users/user/test".into();
        let root = Root::new("test".into(), root_display_path, root_absolute_path);

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
            assert_eq!(repo.relative_path(), Path::new(path_str));
        }
    }
}
