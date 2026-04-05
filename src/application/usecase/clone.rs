use std::sync::Arc;

use crate::{
    application::support::workdir_guard::WorkdirGuard,
    domain::{
        model::{query::Query, repo::Repo, root::Root},
        port::{Ports, clone_repo::RepoClone, dir_editor::DirEditor},
    },
};

#[derive(Debug)]
pub(crate) struct CloneUsecase {
    dir_editor: Arc<dyn DirEditor>,
    repo_clone: Arc<dyn RepoClone>,
}

impl CloneUsecase {
    pub(crate) fn new(ports: &Ports) -> Self {
        Self {
            dir_editor: Arc::clone(&ports.dir_editor),
            repo_clone: Arc::clone(&ports.repo_clone),
        }
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
        let mut workdir = WorkdirGuard::create(dir_editor, clone_path)?;
        self.repo_clone.clone_repo(query.url(), clone_path, bare)?;
        workdir.persist()?;

        Ok(())
    }
}
