use std::fmt::Debug;

use crate::domain::model::{path_like::PathLike, pretty_path::PrettyPath};

#[derive(Debug, thiserror::Error)]
pub(crate) enum RepoProbeError {
    #[error("not a git repository: {}", path.display())]
    NotARepo { path: PrettyPath },
    #[error(transparent)]
    Backend(#[from] Box<dyn std::error::Error>),
}

pub(crate) trait RepoProbe: Debug {
    fn probe(&self, path: &dyn PathLike) -> Result<RepoProbeResult, RepoProbeError>;
}

#[derive(Debug)]
pub(crate) struct RepoProbeResult {
    pub(crate) is_bare: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn RepoProbe> = None;
}
