use std::path::PathBuf;

use crate::{
    presentation::config::Config,
    util::{optional_param::OptionalParam, tilde_path::TildePath},
};

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
    pub(crate) fn root_path(&self, config: &Config) -> OptionalParam<TildePath> {
        match &self.root_path {
            Some(path) => OptionalParam::new_explicit("root", TildePath::new_verbatim(path)),
            None => config.root_map().default_root().path().clone(),
        }
    }

    pub(crate) fn query(&self) -> &String {
        &self.query
    }
}
