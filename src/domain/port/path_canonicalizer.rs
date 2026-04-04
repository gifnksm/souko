use std::{fmt::Debug, path::PathBuf};

use crate::domain::model::{path_like::PathLike, pretty_path::PrettyPath};

#[derive(Debug, thiserror::Error)]
pub(crate) enum PathCanonicalizerError {
    #[error("path not found: {}", path.display())]
    PathNotFound { path: PrettyPath },
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
