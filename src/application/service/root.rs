use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};

use super::helper::workdir::Workdir;
use crate::domain::{
    model::{
        path_like::PathLike,
        query::Query,
        repo::{CanonicalRepo, Repo},
        root::{CanonicalRoot, Root},
    },
    repository::{
        canonicalize_root::CanonicalizeRoot,
        clone_repo::CloneRepo,
        edit_dir::EditDir,
        repo_cache::RepoCache,
        walk_repo::{Entry, Repos, WalkRepo},
        Repository,
    },
};

#[derive(Debug, Clone)]
pub(crate) struct RootService {
    canonicalize_root: Arc<dyn CanonicalizeRoot>,
    walk_repo: Arc<dyn WalkRepo>,
    clone_repo: Arc<dyn CloneRepo>,
    edit_dir: Arc<dyn EditDir>,
    repo_cache: Arc<Mutex<dyn RepoCache>>,
}

impl RootService {
    pub(crate) fn new(repository: &Repository) -> Self {
        Self {
            canonicalize_root: Arc::clone(&repository.canonicalize_root),
            walk_repo: Arc::clone(&repository.walk_repo),
            clone_repo: Arc::clone(&repository.clone_repo),
            edit_dir: Arc::clone(&repository.edit_dir),
            repo_cache: Arc::clone(&repository.repo_cache),
        }
    }

    pub(crate) fn canonicalize_root(
        &self,
        root: &Root,
        should_exist: bool,
    ) -> Result<Option<CanonicalRoot>, Box<dyn std::error::Error>> {
        self.canonicalize_root.canonicalize_root(root, should_exist)
    }

    pub(crate) fn clone_repo(
        &self,
        root: &Root,
        query: &Query,
        bare: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let repo = Repo::from_query(root, query, bare);
        let clone_path = repo.path();

        let edit_dir = Arc::clone(&self.edit_dir);
        let mut workdir = Workdir::create(edit_dir, clone_path)?;
        self.clone_repo.clone_repo(query.url(), clone_path, bare)?;
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
            tracing::warn!("failed to load repo cache: {err} ({})", path.display());
            repo_cache.clear(now);
        }
    }

    pub(crate) fn store_repo_cache(&self, path: &dyn PathLike) {
        let mut repo_cache = self.repo_cache.lock().unwrap();
        if let Err(err) = repo_cache.store(path) {
            tracing::warn!("failed to store repo cache: {err} ({})", path.display());
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
            &*self.walk_repo,
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
    walker: Box<dyn Repos>,
    skip_bare: bool,
    no_recursive: bool,
}

impl FindRepos {
    fn new(
        repo_cache: Arc<Mutex<dyn RepoCache>>,
        walk_repo: &dyn WalkRepo,
        root: &CanonicalRoot,
        skip_hidden: bool,
        skip_bare: bool,
        no_recursive: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut walker = walk_repo.walk_repo(root)?;
        if skip_hidden {
            walker.filter_entry(Box::new(|e| !e.is_hidden()));
        }
        Ok(FindRepos {
            repo_cache,
            walker,
            skip_bare,
            no_recursive,
        })
    }

    fn entry_to_repo(
        &self,
        entry: &dyn Entry,
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
            let entry = itry!(self.walker.next()?);
            let Some(repo) = itry!(self.entry_to_repo(&*entry)) else {
                tracing::trace!("skipping non-git-repository: {}", entry.path().display());
                continue;
            };
            if self.no_recursive {
                self.walker.skip_subdir();
            }
            if self.skip_bare && repo.bare() {
                tracing::trace!("skipping bare repo: {}", repo.path().display());
                continue;
            }
            return Some(Ok(repo));
        }
    }
}
