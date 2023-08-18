use std::path::PathBuf;

use color_eyre::eyre::{eyre, Result, WrapErr};

use crate::{
    application::service::Service,
    domain::model::{display_path::DisplayPath, path_like::PathLike, query::Query, root::Root},
    presentation::{args::GlobalArgs, config::Config, util::optional_param::OptionalParam},
};

#[derive(Debug, Clone, Default, clap::Args)]
pub(super) struct Args {
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
    fn root(&self, config: &Config) -> OptionalParam<Root> {
        match &self.root_path {
            Some(path) => OptionalParam::new_explicit(
                "root",
                Root::new("arg".to_string(), DisplayPath::from_real_path(path.clone())),
            ),
            None => config.default_root().clone(),
        }
    }

    fn query(&self, config: &Config) -> Result<Query> {
        let query_parse_option = config.query_parse_option();
        let query_str = &self.query;
        let query = Query::parse(query_str, &query_parse_option)
            .wrap_err_with(|| format!("invalid query: {query_str}"))?;
        Ok(query)
    }

    pub(super) fn run(&self, global_args: &GlobalArgs, service: &Service) -> Result<()> {
        let config = global_args.config()?;

        let root = self.root(&config);
        let query = self.query(&config)?;

        let root_service = service.root();

        let bare = false;

        tracing::info!(
            "Cloning {} into {}",
            query.original_query(),
            root.value().path().display()
        );

        root_service
            .clone_repo(root.value(), &query, bare)
            .map_err(|e| eyre!(e))
            .wrap_err("failed to clone repository")?;

        tracing::info!("Cloned {}", query.original_query());

        Ok(())
    }
}
