use std::sync::Arc;

use self::{
    canonicalize_root::CanonicalizeRoot, clone_repo::CloneRepo, edit_dir::EditDir,
    walk_repo::WalkRepo,
};

pub(crate) mod canonicalize_root;
pub(crate) mod clone_repo;
pub(crate) mod edit_dir;
pub(crate) mod walk_repo;

#[derive(Debug, Clone)]
pub(crate) struct Repository {
    pub(crate) canonicalize_root: Arc<dyn CanonicalizeRoot>,
    pub(crate) walk_repo: Arc<dyn WalkRepo>,
    pub(crate) clone_repo: Arc<dyn CloneRepo>,
    pub(crate) edit_dir: Arc<dyn EditDir>,
}
