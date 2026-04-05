use std::sync::{Arc, Mutex};

use crate::domain::port::repo_probe::RepoProbe;

use self::{
    clone_repo::RepoClone, dir_editor::DirEditor, dir_walker::DirWalker,
    path_canonicalizer::PathCanonicalizer, repo_cache::RepoCache,
};

pub(crate) mod clone_repo;
pub(crate) mod dir_editor;
pub(crate) mod dir_walker;
pub(crate) mod path_canonicalizer;
pub(crate) mod repo_cache;
pub(crate) mod repo_probe;

#[derive(Debug, Clone)]
pub(crate) struct Ports {
    pub(crate) path_canonicalizer: Arc<dyn PathCanonicalizer>,
    pub(crate) dir_walker: Arc<dyn DirWalker>,
    pub(crate) dir_editor: Arc<dyn DirEditor>,
    pub(crate) repo_clone: Arc<dyn RepoClone>,
    pub(crate) repo_probe: Arc<dyn RepoProbe>,
    pub(crate) repo_cache: Arc<Mutex<dyn RepoCache>>,
}
