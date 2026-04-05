use std::sync::{Arc, Mutex};

use crate::{
    domain::port::Ports,
    infrastructure::{
        fs::{FsDirEditor, FsDirWalker, FsPathCanonicalizer},
        git2::{Git2RepoClone, Git2RepoProbe},
        persistence::JsonRepoCache,
    },
};

mod fs;
mod git2;
mod persistence;

pub(crate) fn ports() -> Ports {
    Ports {
        path_canonicalizer: Arc::new(FsPathCanonicalizer::new()),
        dir_walker: Arc::new(FsDirWalker::new()),
        dir_editor: Arc::new(FsDirEditor::new()),
        repo_clone: Arc::new(Git2RepoClone::new()),
        repo_probe: Arc::new(Git2RepoProbe::new()),
        repo_cache: Arc::new(Mutex::new(JsonRepoCache::new())),
    }
}
