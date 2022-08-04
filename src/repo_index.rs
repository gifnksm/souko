use serde::{Deserialize, Serialize};

use crate::Repo;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RepoIndex {
    #[serde(default)]
    repos: Vec<Repo>,
}

impl RepoIndex {
    pub fn repos(&self) -> &[Repo] {
        // TODO: check whethe the repository is already in the index
        &self.repos
    }

    pub fn push(&mut self, repo: Repo) -> bool {
        if self.repos.iter().any(|r| r.path() == repo.path()) {
            return false;
        }

        self.repos.push(repo);
        true
    }
}
