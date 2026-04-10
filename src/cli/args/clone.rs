#[derive(Debug, Clone, Default, clap::Args)]
pub(in crate::cli) struct CloneArgs {
    /// Name of the root under which the repository will be cloned
    #[arg(long = "root")]
    root_name: Option<String>,

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
    #[arg(verbatim_doc_comment)]
    query: String,
}

impl CloneArgs {
    pub(in crate::cli) fn root_name(&self) -> Option<&str> {
        self.root_name.as_deref()
    }

    pub(in crate::cli) fn query(&self) -> &str {
        &self.query
    }
}
