use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use color_eyre::eyre::eyre;

use super::helper::workdir::Workdir;
use crate::domain::{
    model::{
        path_like::PathLike,
        pretty_path::PrettyPath,
        query::Query,
        repo::{CanonicalRepo, Repo},
        root::{CanonicalRoot, Root},
    },
    port::{
        Ports,
        clone_repo::RepoClone,
        dir_editor::DirEditor,
        path_canonicalizer::{PathCanonicalizer, PathCanonicalizerError},
        repo_cache::RepoCache,
    },
    service::repo_scan::{OwnedEntry, RepoScanService, Repos},
};
use crate::util::error::FormatErrorChain as _;

#[derive(Debug, Clone)]
pub(crate) struct RootService {
    path_canonicalizer: Arc<dyn PathCanonicalizer>,
    repo_scan: RepoScanService,
    repo_clone: Arc<dyn RepoClone>,
    dir_editor: Arc<dyn DirEditor>,
    repo_cache: Arc<Mutex<dyn RepoCache>>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RootServiceError {
    #[error("root `{name}` does not exist: {}", path.display())]
    RootNotExist { name: String, path: PrettyPath },
    #[error("failed to get canonical path of root `{name}`")]
    CanonicalizeRoot {
        name: String,
        #[source]
        source: PathCanonicalizerError,
    },
}

impl RootService {
    pub(crate) fn new(ports: &Ports) -> Self {
        let repo_scan = RepoScanService::new(ports);
        Self {
            path_canonicalizer: Arc::clone(&ports.path_canonicalizer),
            repo_scan,
            repo_clone: Arc::clone(&ports.repo_clone),
            dir_editor: Arc::clone(&ports.dir_editor),
            repo_cache: Arc::clone(&ports.repo_cache),
        }
    }

    pub(crate) fn canonicalize_root(&self, root: &Root) -> Result<CanonicalRoot, RootServiceError> {
        let canonical_path = match self.path_canonicalizer.canonicalize(root.path()) {
            Ok(path) => path,
            Err(PathCanonicalizerError::PathNotFound { path }) => {
                return Err(RootServiceError::RootNotExist {
                    name: root.name().to_owned(),
                    path,
                });
            }
            Err(source) => bail!(RootServiceError::CanonicalizeRoot {
                name: root.name().to_owned(),
                source
            }),
        };
        let canonical_root = CanonicalRoot::new(root.clone(), canonical_path);
        Ok(canonical_root)
    }

    pub(crate) fn clone_repo(
        &self,
        root: &Root,
        query: &Query,
        bare: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let repo = Repo::from_query(root, query, bare);
        let clone_path = repo.path();

        let dir_editor = Arc::clone(&self.dir_editor);
        let mut workdir = Workdir::create(dir_editor, clone_path)?;
        self.repo_clone.clone_repo(query.url(), clone_path, bare)?;
        workdir.persist()?;

        Ok(())
    }

    pub(crate) fn load_repo_cache(
        &self,
        path: &dyn PathLike,
        now: DateTime<Utc>,
        expire_duration: Duration,
    ) {
        let mut repo_cache = self.repo_cache.lock().unwrap();
        if let Err(err) = repo_cache.load(path, now, expire_duration) {
            let err =
                eyre!(err).wrap_err(format!("failed to load repo cache ({})", path.display()));
            tracing::warn!("{}", err.format_error_chain());
            repo_cache.clear(now);
        }
    }

    pub(crate) fn store_repo_cache(&self, path: &dyn PathLike) {
        let mut repo_cache = self.repo_cache.lock().unwrap();
        if let Err(err) = repo_cache.store(path) {
            let err =
                eyre!(err).wrap_err(format!("failed to store repo cache ({})", path.display()));
            tracing::warn!("{}", err.format_error_chain());
        }
    }

    pub(crate) fn find_repos(
        &self,
        root: &CanonicalRoot,
        skip_hidden: bool,
        skip_bare: bool,
        no_recursive: bool,
    ) -> Result<FindRepos, Box<dyn std::error::Error>> {
        FindRepos::new(
            Arc::clone(&self.repo_cache),
            &self.repo_scan,
            root,
            skip_hidden,
            skip_bare,
            no_recursive,
        )
    }
}

#[derive(Debug)]
pub(crate) struct FindRepos {
    repo_cache: Arc<Mutex<dyn RepoCache>>,
    repos: Repos,
    skip_bare: bool,
    no_recursive: bool,
}

impl FindRepos {
    fn new(
        repo_cache: Arc<Mutex<dyn RepoCache>>,
        repo_scan: &RepoScanService,
        root: &CanonicalRoot,
        skip_hidden: bool,
        skip_bare: bool,
        no_recursive: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut repos = repo_scan.repos(root)?;
        if skip_hidden {
            repos.filter_entry(|e| !e.is_hidden());
        }
        Ok(FindRepos {
            repo_cache,
            repos,
            skip_bare,
            no_recursive,
        })
    }

    fn entry_to_repo(
        &self,
        entry: &OwnedEntry,
    ) -> Result<Option<CanonicalRepo>, Box<dyn std::error::Error>> {
        let mut cache_service = self.repo_cache.lock().unwrap();
        if let Some(repo) = cache_service.get(entry.root(), entry.relative_path()) {
            return Ok(Some(repo));
        }
        let repo = entry.to_repo()?;
        if let Some(repo) = &repo {
            cache_service.insert(entry.root(), repo);
        }
        Ok(repo)
    }
}

impl Iterator for FindRepos {
    type Item = Result<CanonicalRepo, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = itry!(self.repos.next()?);
            let Some(repo) = itry!(self.entry_to_repo(&entry)) else {
                tracing::trace!("skipping non-git-repository: {}", entry.path().display());
                continue;
            };
            if self.no_recursive {
                self.repos.skip_subdir();
            }
            if self.skip_bare && repo.bare() {
                tracing::trace!("skipping bare repo: {}", repo.path().display());
                continue;
            }
            return Some(Ok(repo));
        }
    }
}
