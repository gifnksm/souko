use git2_credentials::CredentialHandler;
use url::Url;

use crate::domain::repository::clone_repo::CloneRepo;

#[derive(Debug)]
pub(super) struct FsCloneRepo {}

impl FsCloneRepo {
    pub(super) fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to open git config: {source}")]
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

impl CloneRepo for FsCloneRepo {
    fn clone_repo(
        &self,
        url: &url::Url,
        path: &std::path::Path,
        bare: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut cb = git2::RemoteCallbacks::new();
        let git_config =
            git2::Config::open_default().map_err(|err| Error::OpenConfig { source: err })?;
        let mut ch = CredentialHandler::new(git_config);
        cb.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });

        let mut po = git2::ProxyOptions::new();
        po.auto();

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(cb)
            .proxy_options(po)
            .download_tags(git2::AutotagOption::All)
            .update_fetchhead(true);

        let _repo = git2::build::RepoBuilder::new()
            .bare(bare)
            .fetch_options(fo)
            .clone(url.as_str(), path)
            .map_err(|err| Error::Clone {
                url: url.clone(),
                source: err,
            })?;

        Ok(())
    }
}
