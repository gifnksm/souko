use std::{
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

use crate::tilde_path::PathLike;

#[derive(Debug)]
pub(crate) struct WalkRepo<P> {
    root_dir: P,
    walk_dir: WalkDir,
}

impl<P> WalkRepo<P> {
    pub(crate) fn new(root_dir: P) -> Self
    where
        P: PathLike,
    {
        let walk_dir = WalkDir::new(root_dir.as_path()).sort_by_file_name();
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

impl<P> IntoIterator for WalkRepo<P>
where
    P: PathLike,
{
    type Item = Result<Repo, Error>;
    type IntoIter = IntoIter<P>;

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
pub(crate) struct IntoIter<P> {
    root_dir: P,
    iter: walkdir::IntoIter,
}

impl<P> Iterator for IntoIter<P>
where
    P: PathLike,
{
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

            let repo = match git2::Repository::open(entry.path()) {
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
    display_path: PathBuf,
    absolute_path: PathBuf,
}

impl Repo {
    fn new<P>(root_dir: &P, entry: &DirEntry, repo: &git2::Repository) -> Result<Self, Error>
    where
        P: PathLike,
    {
        let name = entry
            .path()
            .strip_prefix(root_dir.as_path())
            .unwrap()
            .to_owned();

        let workdir = repo.workdir().ok_or_else(|| Error::BareRepo {
            path: repo.path().to_owned(),
        })?;

        let display_path = root_dir.as_display_path().join(&name);

        let absolute_path = workdir.canonicalize().map_err(|e| Error::GetAbsolutePath {
            path: workdir.to_owned(),
            source: e,
        })?;

        Ok(Self {
            name,
            display_path,
            absolute_path,
        })
    }

    pub(crate) fn name(&self) -> &Path {
        &self.name
    }

    pub(crate) fn display_path(&self) -> &Path {
        &self.display_path
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }
}
