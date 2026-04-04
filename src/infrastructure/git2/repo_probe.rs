use crate::domain::{
    model::{path_like::PathLike, pretty_path::PrettyPath},
    port::repo_probe::{RepoProbe, RepoProbeError, RepoProbeResult},
};

#[derive(Debug)]
pub(in crate::infrastructure) struct Git2RepoProbe {}

impl Git2RepoProbe {
    pub(in crate::infrastructure) fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to open repository: {}", path.display())]
    Open {
        path: PrettyPath,
        #[source]
        source: git2::Error,
    },
}

impl RepoProbe for Git2RepoProbe {
    fn probe(&self, path: &dyn PathLike) -> Result<RepoProbeResult, RepoProbeError> {
        let repo = git2::Repository::open(path.as_real_path()).map_err(|source| {
            let path = PrettyPath::new(path);
            if source.code() == git2::ErrorCode::NotFound {
                RepoProbeError::NotARepo { path }
            } else {
                RepoProbeError::Backend(Error::Open { path, source }.into())
            }
        })?;
        Ok(RepoProbeResult {
            is_bare: repo.is_bare(),
        })
    }
}
