use std::sync::Arc;

use crate::application::service::Service;

mod fs_resolve_root;
mod fs_walk_repo;

pub(crate) fn service() -> Service {
    let resolve_root = Arc::new(fs_resolve_root::FsResolveRoot::new());
    Service::new(resolve_root)
}