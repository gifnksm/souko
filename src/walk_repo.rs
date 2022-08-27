use thiserror::Error;
use walkdir::WalkDir;

use crate::{repo, Repo};

#[derive(Debug)]
pub(crate) struct WalkRepo {
    walk_dir: WalkDir,
}

impl WalkRepo {
    pub(crate) fn new(walk_dir: WalkDir) -> Self {
        Self { walk_dir }
    }
}

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
    #[error(transparent)]
    ReadRepo(#[from] repo::ReadError),
}

impl IntoIterator for WalkRepo {
    type Item = Result<Repo, Error>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
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

            let repo = itry!(Repo::try_from(&repo));
            self.iter.skip_current_dir();

            return Some(Ok(repo));
        }
    }
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    let file_name = entry.file_name();
    file_name != "."
        && file_name != ".."
        && file_name
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
}
