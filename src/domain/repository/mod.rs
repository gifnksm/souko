use std::sync::Arc;

use self::{
    clone_repo::CloneRepo, edit_dir::EditDir, resolve_root::ResolveRoot, walk_repo::WalkRepo,
};

pub(crate) mod clone_repo;
pub(crate) mod edit_dir;
pub(crate) mod resolve_root;
pub(crate) mod walk_repo;

#[derive(Debug, Clone)]
pub(crate) struct Repository {
    pub(crate) resolve_root: Arc<dyn ResolveRoot>,
    pub(crate) walk_repo: Arc<dyn WalkRepo>,
    pub(crate) clone_repo: Arc<dyn CloneRepo>,
    pub(crate) edit_dir: Arc<dyn EditDir>,
}
