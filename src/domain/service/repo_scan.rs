use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::domain::{
    model::{
        path_buf_pair::PathBufPair,
        repo::{CanonicalRepo, Repo},
        root::CanonicalRoot,
    },
    port::{
        Ports,
        dir_walker::{DirEntries, DirEntry, DirWalker},
        path_canonicalizer::{PathCanonicalizer, PathCanonicalizerError},
        repo_probe::{RepoProbe, RepoProbeError},
    },
};

#[derive(Debug, Clone)]
pub(crate) struct RepoScanService {
    path_canonicalizer: Arc<dyn PathCanonicalizer>,
    dir_walker: Arc<dyn DirWalker>,
    repo_probe: Arc<dyn RepoProbe>,
}

impl RepoScanService {
    pub(crate) fn new(ports: &Ports) -> Self {
        Self {
            path_canonicalizer: Arc::clone(&ports.path_canonicalizer),
            dir_walker: Arc::clone(&ports.dir_walker),
            repo_probe: Arc::clone(&ports.repo_probe),
        }
    }

    pub(crate) fn repos(&self, root: &CanonicalRoot) -> Result<Repos, Box<dyn std::error::Error>> {
        Ok(Repos {
            path_canonicalizer: Arc::clone(&self.path_canonicalizer),
            repo_probe: Arc::clone(&self.repo_probe),
            port: self.dir_walker.entries(root)?,
        })
    }
}

#[derive(Debug)]
pub(crate) struct Repos {
    path_canonicalizer: Arc<dyn PathCanonicalizer>,
    repo_probe: Arc<dyn RepoProbe>,
    port: Box<dyn DirEntries>,
}

impl Repos {
    pub(crate) fn skip_subdir(&mut self) {
        self.port.skip_subdir();
    }

    pub(crate) fn filter_entry<F>(&mut self, mut filter: F)
    where
        F: for<'a> FnMut(BorrowedEntry<'a>) -> bool + 'static,
    {
        let path_canonicalizer = Arc::clone(&self.path_canonicalizer);
        let repo_probe = Arc::clone(&self.repo_probe);
        self.port.filter_entry(Box::new(move |port| {
            filter(Entry {
                path_canonicalizer: Arc::clone(&path_canonicalizer),
                repo_probe: Arc::clone(&repo_probe),
                port,
            })
        }));
    }
}

impl Iterator for Repos {
    type Item = Result<OwnedEntry, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.port.next()?;
        Some(entry.map(|port| Entry {
            path_canonicalizer: Arc::clone(&self.path_canonicalizer),
            repo_probe: Arc::clone(&self.repo_probe),
            port,
        }))
    }
}

pub(crate) type OwnedEntry = Entry<Box<dyn DirEntry>>;
pub(crate) type BorrowedEntry<'a> = Entry<&'a dyn DirEntry>;

#[derive(Debug, thiserror::Error)]
enum ToRepoError {
    #[error(
        "failed to get canonical path of repository {} in root `{root_name}`",
        repo_relative_path.display()
    )]
    CanonicalizeRepo {
        root_name: String,
        repo_relative_path: PathBuf,
        #[source]
        source: PathCanonicalizerError,
    },
}

#[derive(Debug)]
pub(crate) struct Entry<P> {
    path_canonicalizer: Arc<dyn PathCanonicalizer>,
    repo_probe: Arc<dyn RepoProbe>,
    port: P,
}

impl<P> Entry<P>
where
    P: DirEntry,
{
    pub(crate) fn root(&self) -> &CanonicalRoot {
        self.port.root()
    }

    pub(crate) fn relative_path(&self) -> &Path {
        self.port.relative_path()
    }

    pub(crate) fn path(&self) -> &PathBufPair {
        self.port.path()
    }

    pub(crate) fn is_hidden(&self) -> bool {
        self.port.is_hidden()
    }

    pub(crate) fn to_repo(&self) -> Result<Option<CanonicalRepo>, Box<dyn std::error::Error>> {
        let repo_probe = match self.repo_probe.probe(self.path()) {
            Ok(repo_probe) => repo_probe,
            Err(RepoProbeError::NotARepo { .. }) => return Ok(None),
            Err(e @ RepoProbeError::Backend(_)) => return Err(e.into()),
        };
        let repo = Repo::from_relative_path(
            self.root().as_root(),
            self.relative_path().to_owned(),
            repo_probe.is_bare,
        );
        let canonical_path =
            self.path_canonicalizer
                .canonicalize(repo.path())
                .map_err(|source| ToRepoError::CanonicalizeRepo {
                    root_name: self.root().name().to_owned(),
                    repo_relative_path: repo.relative_path().to_owned(),
                    source,
                })?;
        let canonical_repo = CanonicalRepo::new(repo, canonical_path);
        Ok(Some(canonical_repo))
    }
}
