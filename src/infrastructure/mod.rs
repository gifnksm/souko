use std::sync::Arc;

use crate::domain::repository::Repository;

mod fs_canonicalize_root;
mod fs_clone_repo;
mod fs_edit_dir;
mod fs_walk_repo;

pub(crate) fn repository() -> Repository {
    Repository {
        canonicalize_root: Arc::new(fs_canonicalize_root::FsCanonicalizeRoot::new()),
        walk_repo: Arc::new(fs_walk_repo::FsWalkRepo::new()),
        clone_repo: Arc::new(fs_clone_repo::FsCloneRepo::new()),
        edit_dir: Arc::new(fs_edit_dir::FsEditDir::new()),
    }
}
