use std::sync::Arc;

use crate::domain::repository::Repository;

mod fs_clone_repo;
mod fs_edit_dir;
mod fs_resolve_root;
mod fs_walk_repo;

pub(crate) fn repository() -> Repository {
    Repository {
        resolve_root: Arc::new(fs_resolve_root::FsResolveRoot::new()),
        walk_repo: Arc::new(fs_walk_repo::FsWalkRepo::new()),
        clone_repo: Arc::new(fs_clone_repo::FsCloneRepo::new()),
        edit_dir: Arc::new(fs_edit_dir::FsEditDir::new()),
    }
}
