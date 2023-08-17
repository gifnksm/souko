use std::sync::Arc;

use crate::domain::{
    model::{
        repo::Repo,
        root::{Root, RootSpec},
    },
    repository::{
        resolve_root::ResolveRoot,
        walk_repo::{Repos, WalkRepo},
        Repository,
    },
};

#[derive(Debug, Clone)]
pub(crate) struct RootService {
    resolve_root: Arc<dyn ResolveRoot>,
    walk_repo: Arc<dyn WalkRepo>,
}

impl RootService {
    pub(crate) fn new(repository: &Repository) -> Self {
        Self {
            resolve_root: Arc::clone(&repository.resolve_root),
            walk_repo: Arc::clone(&repository.walk_repo),
        }
    }

    pub(crate) fn resolve_root(
        &self,
        root: &RootSpec,
        should_exist: bool,
    ) -> Result<Option<Root>, Box<dyn std::error::Error>> {
        self.resolve_root.resolve_root(root, should_exist)
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
                    tracing::trace!("skipping bare repo: {}", repo.absolute_path().display());
                    continue;
                }
                Some(repo) => return Some(Ok(repo)),
                None => {
                    tracing::trace!(
                        "skipping non-git-repository: {}",
                        entry.absolute_path().display()
                    );
                    continue;
                }
            }
        }
    }
}
