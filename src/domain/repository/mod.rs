use std::sync::Arc;

use self::{resolve_root::ResolveRoot, walk_repo::WalkRepo};

pub(crate) mod resolve_root;
pub(crate) mod walk_repo;

#[derive(Debug, Clone)]
pub(crate) struct Repository {
    pub(crate) resolve_root: Arc<dyn ResolveRoot>,
    pub(crate) walk_repo: Arc<dyn WalkRepo>,
}
