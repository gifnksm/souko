use std::sync::Arc;

use crate::domain::repository::Repository;

mod fs_resolve_root;
mod fs_walk_repo;

pub(crate) fn repository() -> Repository {
    Repository {
        resolve_root: Arc::new(fs_resolve_root::FsResolveRoot::new()),
        walk_repo: Arc::new(fs_walk_repo::FsWalkRepo::new()),
    }
}
