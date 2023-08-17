use std::sync::Arc;

use crate::domain::repository::{resolve_root::ResolveRoot, walk_repo::WalkRepo};

pub(crate) mod root;

pub(crate) struct Service {
    root: root::RootService,
}

impl Service {
    pub(crate) fn new(resolve_root: Arc<dyn ResolveRoot>, walk_repo: Arc<dyn WalkRepo>) -> Self {
        Self {
            root: root::RootService::new(resolve_root, walk_repo),
        }
    }

    pub(crate) fn root(&self) -> &root::RootService {
        &self.root
    }
}
