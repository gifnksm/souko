use std::{fmt::Debug, path::PathBuf};

use crate::domain::model::{path_buf_pair::PathBufPair, path_like::PathLike};

#[derive(Debug, thiserror::Error)]
pub(crate) enum PathCanonicalizerError {
    #[error("path not found: {}", path.display())]
    PathNotFound { path: PathBufPair },
    #[error(transparent)]
    Backend(#[from] Box<dyn std::error::Error>),
}

pub(crate) trait PathCanonicalizer: Debug {
    fn canonicalize(&self, path: &dyn PathLike) -> Result<PathBuf, PathCanonicalizerError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn PathCanonicalizer> = None;
}
