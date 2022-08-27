use std::path::PathBuf;

use crate::{Config, OptionalParam};

#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct Args {
    /// Path of the root directory under which the repository will be cloned
    #[clap(long = "root")]
    root_path: Option<PathBuf>,

    /// Git repository to clone repository from
    ///
    /// Following formats are supported:
    ///
    /// * GitHub repository: `gifnksm/souko`
    /// * Abbreviated GitHub URL: `gh:gifnksm/souko`
    /// * Abbreviated GitLab URL: `gl:gifnksm/souko`
    /// * HTTP(S) URL: `https://github.com/gifnksm/souko.git`
    /// * SSH URL: `ssh://git@github.com/gifnksm/souko.git`
    /// * Git URL: `git://github.com/gifnksm/souko.git`
    /// * scp-like: `git@github.com:gifnksm/souko.git`
    #[clap(verbatim_doc_comment)]
    query: String,
}

impl Args {
    pub(crate) fn root_path(&self, config: &Config) -> OptionalParam<'_, PathBuf> {
        OptionalParam::new("root", &self.root_path, || {
            config.root_path().value().to_owned()
        })
    }

    pub(crate) fn query(&self) -> &String {
        &self.query
    }
}
