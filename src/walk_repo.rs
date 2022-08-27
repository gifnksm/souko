use std::{
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub(crate) struct WalkRepo {
    root_dir: PathBuf,
    walk_dir: WalkDir,
}

impl WalkRepo {
    pub(crate) fn new(root_path: impl Into<PathBuf>) -> Self {
        let root_dir = root_path.into();
        let walk_dir = WalkDir::new(&root_dir);
        Self { root_dir, walk_dir }
    }
}

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
    #[error("`{path}` is bare repository")]
    BareRepo { path: PathBuf },
    #[error("failed to get absolute path of {path}: {source}")]
    GetAbsolutePath { path: PathBuf, source: io::Error },
}

impl IntoIterator for WalkRepo {
    type Item = Result<Repo, Error>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            root_dir: self.root_dir,
            iter: self.walk_dir.into_iter(),
        }
    }
}

macro_rules! itry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(From::from(err))),
        }
    };
}

#[derive(Debug)]
pub(crate) struct IntoIter {
    root_dir: PathBuf,
    iter: walkdir::IntoIter,
}

impl Iterator for IntoIter {
    type Item = Result<Repo, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = itry!(self.iter.next()?);

            if !entry.file_type().is_dir() {
                tracing::trace!("skipping non-directory: {}", entry.path().display());
                continue;
            }

            if is_hidden(&entry) {
                tracing::trace!("skipping hidden directory: {}", entry.path().display());
                self.iter.skip_current_dir();
                continue;
            }

            let repo = match git2::Repository::open(&entry.path()) {
                Ok(repo) => repo,
                Err(e) => {
                    tracing::trace!(error = %e, "skipping non-git repository: {}", entry.path().display());
                    continue;
                }
            };

            let repo = itry!(Repo::new(&self.root_dir, &entry, &repo));
            self.iter.skip_current_dir();

            return Some(Ok(repo));
        }
    }
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .path()
        .file_name()
        .and_then(|file_name| file_name.to_str().map(|s| s.starts_with('.')))
        .unwrap_or(false)
}

pub(crate) struct Repo {
    name: PathBuf,
    absolute_path: PathBuf,
}

impl Repo {
    fn new(root_dir: &Path, entry: &DirEntry, repo: &git2::Repository) -> Result<Self, Error> {
        let name = entry.path().strip_prefix(root_dir).unwrap().to_owned();

        let workdir = repo.workdir().ok_or_else(|| Error::BareRepo {
            path: repo.path().to_owned(),
        })?;

        let absolute_path = workdir.canonicalize().map_err(|e| Error::GetAbsolutePath {
            path: workdir.to_owned(),
            source: e,
        })?;

        Ok(Self {
            name,
            absolute_path,
        })
    }

    pub(crate) fn name(&self) -> &Path {
        &self.name
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }
}
