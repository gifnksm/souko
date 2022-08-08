use serde::{Deserialize, Serialize};

use crate::Repo;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct RepoIndex {
    #[serde(default)]
    repos: Vec<Repo>,
}

impl RepoIndex {
    pub(crate) fn repos(&self) -> &[Repo] {
        // TODO: check whethe the repository is already in the index
        &self.repos
    }

    pub(crate) fn push(&mut self, repo: Repo) -> bool {
        if self.repos.iter().any(|r| r.path() == repo.path()) {
            return false;
        }

        self.repos.push(repo);
        true
    }
}
