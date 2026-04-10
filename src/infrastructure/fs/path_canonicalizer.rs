use std::{io, path::PathBuf};

use crate::domain::{
    model::{path_buf_pair::PathBufPair, path_like::PathLike},
    port::path_canonicalizer::{PathCanonicalizer, PathCanonicalizerError},
};

#[derive(Debug)]
pub(in crate::infrastructure) struct FsPathCanonicalizer {}

impl FsPathCanonicalizer {
    pub(in crate::infrastructure) fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to get absolute path of {}", path.display())]
    Canonicalize {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl PathCanonicalizer for FsPathCanonicalizer {
    fn canonicalize(&self, path: &dyn PathLike) -> Result<PathBuf, PathCanonicalizerError> {
        match dunce::canonicalize(path.as_real_path()) {
            Ok(path) => Ok(path),
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                Err(PathCanonicalizerError::PathNotFound {
                    path: PathBufPair::new(path),
                })
            }
            Err(err) => Err(PathCanonicalizerError::Backend(
                Error::Canonicalize {
                    path: path.as_real_path().to_owned(),
                    source: err,
                }
                .into(),
            )),
        }
    }
}
