use std::{
    io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Repo {
    path: PathBuf,
}

#[derive(Debug, Error)]
pub(crate) enum ReadError {
    #[error("`{path}` is not a git repository: {source}")]
    NotGitRepo { path: PathBuf, source: git2::Error },
    #[error("`{path}` is bare repository")]
    BareRepo { path: PathBuf },
    #[error("failed to get absolute path of {path}: {source}")]
    GetAbsolutePath { path: PathBuf, source: io::Error },
}

impl Repo {
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn read(path: impl AsRef<Path>) -> Result<Repo, ReadError> {
        let path = path.as_ref();
        let repo = git2::Repository::open(path).map_err(|e| ReadError::NotGitRepo {
            path: path.to_owned(),
            source: e,
        })?;
        Self::try_from(&repo)
    }
}

impl TryFrom<&git2::Repository> for Repo {
    type Error = ReadError;

    fn try_from(repo: &git2::Repository) -> Result<Self, Self::Error> {
        let workdir = repo.workdir().ok_or_else(|| ReadError::BareRepo {
            path: repo.path().to_owned(),
        })?;

        let path = workdir
            .canonicalize()
            .map_err(|e| ReadError::GetAbsolutePath {
                path: workdir.to_owned(),
                source: e,
            })?;

        Ok(Self { path })
    }
}
