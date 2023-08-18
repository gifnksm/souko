use std::sync::Arc;

use crate::domain::{
    model::{display_path::DisplayPath, path_like::PathLike, repo::Repo, root::Root},
    repository::walk_repo::{Entry, FilterPredicate, Repos, WalkRepo},
};

#[derive(Debug)]
pub(super) struct FsWalkRepo {}

impl FsWalkRepo {
    pub(super) fn new() -> Self {
        Self {}
    }
}

impl WalkRepo for FsWalkRepo {
    fn walk_repo(&self, root: &Root) -> Result<Box<dyn Repos>, Box<dyn std::error::Error>> {
        Ok(Box::new(FsRepos::new(root)))
    }
}

#[derive(custom_debug_derive::Debug)]
pub(super) struct FsRepos {
    root: Arc<Root>,
    iter: walkdir::IntoIter,
    #[debug(skip)]
    filter: Option<FilterPredicate>,
}

impl FsRepos {
    pub(super) fn new(root: &Root) -> Self {
        let root = Arc::new(root.clone());
        let iter = walkdir::WalkDir::new(root.path().as_path())
            .sort_by_file_name()
            .into_iter();
        Self {
            root,
            iter,
            filter: None,
        }
    }
}

impl Repos for FsRepos {
    fn skip_subdir(&mut self) {
        self.iter.skip_current_dir();
    }

    fn filter_entry(&mut self, filter: FilterPredicate) {
        let mut filter = filter;
        let filter = match self.filter.take() {
            Some(mut pre_filter) => {
                Box::new(move |entry: &dyn Entry| pre_filter(entry) && filter(entry))
            }
            None => filter,
        };

        self.filter = Some(filter);
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
}

impl Iterator for FsRepos {
    type Item = Result<Box<dyn Entry>, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = itry!(self.iter.next()?.map_err(Error::WalkDir));

            if !entry.file_type().is_dir() {
                tracing::trace!("skipping non-directory: {}", entry.path().display());
                continue;
            }

            let entry = FsEntry::new(Arc::clone(&self.root), entry);

            if let Some(filter) = &mut self.filter {
                if !filter(&entry) {
                    tracing::trace!("skipping filtered entry: {}", entry.path().display());
                    self.iter.skip_current_dir();
                    continue;
                }
            }

            return Some(Ok(Box::new(entry)));
        }
    }
}

#[derive(Debug)]
struct FsEntry {
    root: Arc<Root>,
    relative_path: DisplayPath,
    path: DisplayPath,
}

impl FsEntry {
    fn new(root: Arc<Root>, entry: walkdir::DirEntry) -> Self {
        let relative_path = entry
            .path()
            .strip_prefix(root.path().as_path())
            .unwrap() // never panic because the path starts with the root path
            .to_owned();
        let relative_path = DisplayPath::from_expanded(relative_path);
        let path = root.path().join(&relative_path);

        Self {
            root,
            relative_path,
            path,
        }
    }
}

impl Entry for FsEntry {
    fn path(&self) -> &DisplayPath {
        &self.path
    }

    fn is_hidden(&self) -> bool {
        self.path()
            .as_path()
            .file_name()
            .and_then(|file_name| file_name.to_str().map(|s| s.starts_with('.')))
            .unwrap_or(false)
    }

    fn to_repo(&self) -> Result<Option<Repo>, Box<dyn std::error::Error>> {
        let git2_repo = match git2::Repository::open(self.path().as_path()) {
            Ok(repo) => repo,
            Err(_e) => return Ok(None),
        };
        let bare = git2_repo.is_bare();
        let repo = Repo::from_relative_path(&self.root, self.relative_path.clone(), bare);
        Ok(Some(repo))
    }
}
