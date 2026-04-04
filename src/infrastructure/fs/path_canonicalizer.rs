use std::{io, path::PathBuf};

use crate::domain::{
    model::{path_like::PathLike, pretty_path::PrettyPath},
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
                    path: PrettyPath::new(path),
                })
            }
            Err(err) => bail!(PathCanonicalizerError::Backend(
                Error::Canonicalize {
                    path: path.as_real_path().to_owned(),
                    source: err,
                }
                .into()
            )),
        }
    }
}
