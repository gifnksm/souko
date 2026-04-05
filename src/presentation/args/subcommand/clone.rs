use color_eyre::eyre::{Result, WrapErr, eyre};

use crate::{
    application::usecase::Usecases,
    domain::model::{path_like::PathLike, query::Query, root::Root},
    presentation::{
        args::GlobalArgs, config::Config, message, model::optional_param::OptionalParam,
    },
    project_dirs::ProjectDirs,
};

#[derive(Debug, Clone, Default, clap::Args)]
pub(super) struct Args {
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

impl Args {
    fn root(&self, config: &Config, project_dirs: &ProjectDirs) -> Result<OptionalParam<Root>> {
        let root = match &self.root_name {
            Some(name) => {
                let roots = config.roots(project_dirs);
                roots
                    .get(name)
                    .cloned()
                    .ok_or_else(|| eyre!("root `{name}` not found in config file"))?
            }
            None => config.default_root(project_dirs).clone(),
        };
        Ok(root)
    }

    fn query(&self, config: &Config) -> Result<Query> {
        let query_parse_option = config.query_parse_option();
        let query_str = &self.query;
        let query = Query::parse(query_str, &query_parse_option)
            .wrap_err_with(|| format!("invalid query: {query_str}"))?;
        Ok(query)
    }

    pub(super) fn run(
        &self,
        global_args: &GlobalArgs,
        usecases: &Usecases,
        project_dirs: &ProjectDirs,
    ) -> Result<()> {
        let config = global_args.config(project_dirs)?;

        let root = self.root(&config, project_dirs)?;
        let query = self.query(&config)?;

        let bare = false;

        message::info!(
            "cloning {} into {}",
            query.original_query(),
            root.value().path().display()
        );

        usecases
            .clone()
            .clone_repo(root.value(), &query, bare)
            .map_err(|e| eyre!(e))
            .wrap_err("failed to clone repository")?;

        message::info!("cloned {}", query.original_query());

        Ok(())
    }
}
