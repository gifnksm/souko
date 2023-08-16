use std::{io, path::PathBuf};

use super::fs_walk_repo::FsWalkRepo;
use crate::domain::{
    model::root::{Root, RootSpec},
    repository::{resolve_root::ResolveRoot, walk_repo::WalkRepo},
};

#[derive(Debug)]
pub(super) struct FsResolveRoot;

impl FsResolveRoot {
    pub(super) fn new() -> Self {
        Self
    }
}

#[derive(Debug, thiserror::Error)]
enum ResolveError {
    #[error("failed to get absolute path of root {root_name}: {path}")]
    Canonicalize {
        root_name: String,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl ResolveRoot for FsResolveRoot {
    fn resolve_root(
        &self,
        spec: &RootSpec,
        should_exist: bool,
    ) -> Result<Option<Root>, Box<dyn std::error::Error>> {
        let name = spec.name().to_owned();
        let display_path = spec.path().as_display_path().to_owned();
        let absolute_path = match spec.path().as_path().canonicalize() {
            Ok(path) => path,
            Err(err) if !should_exist && err.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(err) => {
                return Err(ResolveError::Canonicalize {
                    root_name: name.clone(),
                    path: spec.path().as_path().to_owned(),
                    source: err,
                }
                .into())
            }
        };

        let root = Root::new(name, display_path, absolute_path);
        Ok(Some(root))
    }

    fn repo_walker(&self, root: &Root) -> Result<Box<dyn WalkRepo>, Box<dyn std::error::Error>> {
        Ok(Box::new(FsWalkRepo::new(root)))
    }
}