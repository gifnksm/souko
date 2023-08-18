use std::{io, path::PathBuf};

use crate::domain::{
    model::{
        path_like::PathLike,
        root::{CanonicalRoot, Root},
    },
    repository::canonicalize_root::CanonicalizeRoot,
};

#[derive(Debug)]
pub(super) struct FsCanonicalizeRoot {}

impl FsCanonicalizeRoot {
    pub(super) fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to get absolute path of root {root_name}: {path}")]
    Canonicalize {
        root_name: String,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl CanonicalizeRoot for FsCanonicalizeRoot {
    fn canonicalize_root(
        &self,
        root: &Root,
        should_exist: bool,
    ) -> Result<Option<CanonicalRoot>, Box<dyn std::error::Error>> {
        let real_path = root.path().as_real_path();
        let canonical_path = match real_path.canonicalize() {
            Ok(path) => path,
            Err(err) if !should_exist && err.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(err) => bail!(Error::Canonicalize {
                root_name: root.name().to_owned(),
                path: real_path.to_owned(),
                source: err,
            }),
        };

        let root = CanonicalRoot::new(root.clone(), canonical_path);
        Ok(Some(root))
    }
}
