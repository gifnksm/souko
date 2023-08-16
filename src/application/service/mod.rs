use std::sync::Arc;

use crate::domain::repository::resolve_root::ResolveRoot;

pub(crate) mod root;

pub(crate) struct Service {
    root: root::RootService,
}

impl Service {
    pub(crate) fn new(resolve_root: Arc<dyn ResolveRoot>) -> Self {
        Self {
            root: root::RootService::new(resolve_root),
        }
    }

    pub(crate) fn root(&self) -> &root::RootService {
        &self.root
    }
}
