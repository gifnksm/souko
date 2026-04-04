use git2_credentials::CredentialHandler;
use url::Url;

use crate::domain::{model::path_like::PathLike, port::clone_repo::RepoClone};

#[derive(Debug)]
pub(in crate::infrastructure) struct Git2RepoClone {}

impl Git2RepoClone {
    pub(in crate::infrastructure) fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to open git config")]
    OpenConfig {
        #[source]
        source: git2::Error,
    },
    #[error("failed to clone git repository from {url}")]
    Clone {
        url: Url,
        #[source]
        source: git2::Error,
    },
}

impl RepoClone for Git2RepoClone {
    fn clone_repo(
        &self,
        url: &url::Url,
        path: &dyn PathLike,
        bare: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut callback = git2::RemoteCallbacks::new();
        let git_config =
            git2::Config::open_default().map_err(|err| Error::OpenConfig { source: err })?;
        let mut credential_handler = CredentialHandler::new(git_config);
        callback.credentials(move |url, username, allowed| {
            credential_handler.try_next_credential(url, username, allowed)
        });

        let mut proxy_opt = git2::ProxyOptions::new();
        proxy_opt.auto();

        let mut fetch_opt = git2::FetchOptions::new();
        fetch_opt
            .remote_callbacks(callback)
            .proxy_options(proxy_opt)
            .download_tags(git2::AutotagOption::All)
            .update_fetchhead(true);

        let _repo = git2::build::RepoBuilder::new()
            .bare(bare)
            .fetch_options(fetch_opt)
            .clone(url.as_str(), path.as_real_path())
            .map_err(|err| Error::Clone {
                url: url.clone(),
                source: err,
            })?;

        Ok(())
    }
}
