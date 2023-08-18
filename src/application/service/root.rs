use std::sync::Arc;

use super::helper::workdir::Workdir;
use crate::domain::{
    model::{
        path_like::PathLike,
        query::Query,
        repo::Repo,
        root::{Root, RootSpec},
    },
    repository::{
        clone_repo::CloneRepo,
        edit_dir::EditDir,
        resolve_root::ResolveRoot,
        walk_repo::{Repos, WalkRepo},
        Repository,
    },
};

#[derive(Debug, Clone)]
pub(crate) struct RootService {
    resolve_root: Arc<dyn ResolveRoot>,
    walk_repo: Arc<dyn WalkRepo>,
    clone_repo: Arc<dyn CloneRepo>,
    edit_dir: Arc<dyn EditDir>,
}

impl RootService {
    pub(crate) fn new(repository: &Repository) -> Self {
        Self {
            resolve_root: Arc::clone(&repository.resolve_root),
            walk_repo: Arc::clone(&repository.walk_repo),
            clone_repo: Arc::clone(&repository.clone_repo),
            edit_dir: Arc::clone(&repository.edit_dir),
        }
    }

    pub(crate) fn resolve_root(
        &self,
        root: &RootSpec,
        should_exist: bool,
    ) -> Result<Option<Root>, Box<dyn std::error::Error>> {
        self.resolve_root.resolve_root(root, should_exist)
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

    pub(crate) fn find_repos(
        &self,
        root: &Root,
        skip_hidden: bool,
        skip_bare: bool,
        no_recursive: bool,
    ) -> Result<FindRepos, Box<dyn std::error::Error>> {
        FindRepos::new(&*self.walk_repo, root, skip_hidden, skip_bare, no_recursive)
    }
}

#[derive(Debug)]
pub(crate) struct FindRepos {
    skip_bare: bool,
    no_recursive: bool,
    walker: Box<dyn Repos>,
}

impl FindRepos {
    fn new(
        walk_repo: &dyn WalkRepo,
        root: &Root,
        skip_hidden: bool,
        skip_bare: bool,
        no_recursive: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut walker = walk_repo.walk_repo(root)?;
        if skip_hidden {
            walker.filter_entry(Box::new(|e| !e.is_hidden()));
        }
        Ok(FindRepos {
            skip_bare,
            no_recursive,
            walker,
        })
    }
}

impl Iterator for FindRepos {
    type Item = Result<Repo, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = itry!(self.walker.next()?);
            let repo = itry!(entry.to_repo());
            if self.no_recursive && repo.is_some() {
                self.walker.skip_subdir();
            }
            match repo {
                Some(repo) if self.skip_bare && repo.bare() => {
                    tracing::trace!("skipping bare repo: {}", repo.path().display());
                    continue;
                }
                Some(repo) => return Some(Ok(repo)),
                None => {
                    tracing::trace!("skipping non-git-repository: {}", entry.path().display());
                    continue;
                }
            }
        }
    }
}
