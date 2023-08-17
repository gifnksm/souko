use std::sync::Arc;

use crate::domain::{
    model::{
        repo::Repo,
        root::{Root, RootSpec},
    },
    repository::{resolve_root::ResolveRoot, walk_repo::WalkRepo},
};

#[derive(Debug, Clone)]
pub(crate) struct RootService {
    root_resolver: Arc<dyn ResolveRoot>,
}

impl RootService {
    pub(crate) fn new(root_resolver: Arc<dyn ResolveRoot>) -> Self {
        Self { root_resolver }
    }

    pub(crate) fn resolve_root(
        &self,
        root: &RootSpec,
        should_exist: bool,
    ) -> Result<Option<Root>, Box<dyn std::error::Error>> {
        self.root_resolver.resolve_root(root, should_exist)
    }

    pub(crate) fn find_repos(
        &self,
        root: &Root,
        skip_hidden: bool,
        skip_bare: bool,
        no_recursive: bool,
    ) -> Result<
        impl Iterator<Item = Result<Repo, Box<dyn std::error::Error>>>,
        Box<dyn std::error::Error>,
    > {
        let mut walker = self.root_resolver.repo_walker(root)?;
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

#[derive(Debug)]
pub(crate) struct FindRepos {
    skip_bare: bool,
    no_recursive: bool,
    walker: Box<dyn WalkRepo>,
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
