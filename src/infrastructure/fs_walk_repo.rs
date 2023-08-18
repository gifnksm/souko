use std::{io, path::PathBuf, sync::Arc};

use crate::domain::{
    model::{
        path_like::PathLike,
        pretty_path::PrettyPath,
        repo::{CanonicalRepo, Repo},
        root::CanonicalRoot,
    },
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
    fn walk_repo(
        &self,
        root: &CanonicalRoot,
    ) -> Result<Box<dyn Repos>, Box<dyn std::error::Error>> {
        Ok(Box::new(FsRepos::new(root)))
    }
}

#[derive(custom_debug_derive::Debug)]
pub(super) struct FsRepos {
    root: Arc<CanonicalRoot>,
    iter: walkdir::IntoIter,
    #[debug(skip)]
    filter: Option<FilterPredicate>,
}

impl FsRepos {
    pub(super) fn new(root: &CanonicalRoot) -> Self {
        let root = Arc::new(root.clone());
        let iter = walkdir::WalkDir::new(root.path().as_real_path())
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
    #[error("failed to canonicalize: {path}")]
    Canonicalize {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
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
    root: Arc<CanonicalRoot>,
    entry: walkdir::DirEntry,
    relative_path: PrettyPath,
    path: PrettyPath,
}

impl FsEntry {
    fn new(root: Arc<CanonicalRoot>, entry: walkdir::DirEntry) -> Self {
        let relative_path = entry
            .path()
            .strip_prefix(root.path().as_real_path())
            .unwrap() // never panic because the path starts with the root path
            .to_owned();
        let relative_path = PrettyPath::from_real_path(relative_path);
        let path = root.path().join(&relative_path);

        Self {
            root,
            entry,
            relative_path,
            path,
        }
    }
}

impl Entry for FsEntry {
    fn path(&self) -> &PrettyPath {
        &self.path
    }

    fn is_hidden(&self) -> bool {
        self.path()
            .as_real_path()
            .file_name()
            .and_then(|file_name| file_name.to_str().map(|s| s.starts_with('.')))
            .unwrap_or(false)
    }

    fn to_repo(&self) -> Result<Option<CanonicalRepo>, Box<dyn std::error::Error>> {
        let git2_repo = match git2::Repository::open(self.path().as_real_path()) {
            Ok(repo) => repo,
            Err(_e) => return Ok(None),
        };
        let bare = git2_repo.is_bare();
        let repo = Repo::from_relative_path(self.root.as_root(), self.relative_path.clone(), bare);

        let canonical_path =
            self.entry
                .path()
                .canonicalize()
                .map_err(|err| Error::Canonicalize {
                    path: self.entry.path().to_owned(),
                    source: err,
                })?;

        let canonical_repo = CanonicalRepo::new(repo, canonical_path);

        Ok(Some(canonical_repo))
    }
}
