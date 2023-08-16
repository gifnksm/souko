use std::path::PathBuf;

use color_eyre::eyre::{Result, WrapErr};

use crate::{
    application,
    domain::model::query::Query,
    presentation::{
        args::GlobalArgs,
        config::Config,
        util::{optional_param::OptionalParam, tilde_path::TildePath},
    },
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
    fn root_path(&self, config: &Config) -> OptionalParam<TildePath> {
        match &self.root_path {
            Some(path) => OptionalParam::new_explicit("root", TildePath::new_verbatim(path)),
            None => config.default_root_path().clone(),
        }
    }

    fn query(&self, config: &Config) -> Result<Query> {
        let query_parse_option = config.query_parse_option();
        let query_str = &self.query;
        let query = Query::parse(query_str, &query_parse_option)
            .wrap_err_with(|| format!("invalid query: {query_str}"))?;
        Ok(query)
    }

    pub(super) fn run(&self, global_args: &GlobalArgs) -> Result<()> {
        let config = global_args.config()?;

        let root_path = self.root_path(&config);
        let query = self.query(&config)?;

        application::command::clone::run(root_path.value().as_ref(), &query)?;

        Ok(())
    }
}
