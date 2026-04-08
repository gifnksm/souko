use color_eyre::eyre::{Result, eyre};

use crate::{
    app_dirs::AppDirs,
    application::usecase::Usecases,
    domain::model::{path_like::PathLike as _, pretty_path::PrettyPath},
    presentation::{
        args::Args,
        config::Config,
        context::{query::QueryContext, root::RootContextMap},
        model::optional_param::OptionalParam,
    },
    util::file,
};

#[derive(Debug)]
pub(in crate::presentation) struct GlobalContext {
    usecases: Usecases,
    root_map: RootContextMap,
    query: QueryContext,
    repo_cache_path: PrettyPath,
}

impl GlobalContext {
    pub(in crate::presentation) fn new(
        args: &Args,
        usecases: Usecases,
        app_dirs: AppDirs,
    ) -> Result<Self> {
        let current_dir = std::env::current_dir()?;

        // Args-sourced paths are resolved relative to the current working directory.
        let config_path = args.global_args().config_path(&app_dirs, &current_dir);
        let repo_cache_path = args
            .global_args()
            .repo_cache_path(&app_dirs, &current_dir)
            .value()
            .clone();

        // Config-sourced paths (e.g. root paths) are resolved relative to the config file's
        // directory; see RootContextMap::new.
        let config_dir = config_path
            .value()
            .as_real_path()
            .parent()
            .unwrap_or(config_path.value().as_real_path());
        let config = load_config(&config_path)?;
        let root_map = RootContextMap::new(&config.roots, &app_dirs, config_dir);
        let query = QueryContext::from_config(&config.query);

        Ok(Self {
            usecases,
            root_map,
            query,
            repo_cache_path,
        })
    }

    pub(in crate::presentation) fn usecases(&self) -> &Usecases {
        &self.usecases
    }

    pub(in crate::presentation) fn repo_cache_path(&self) -> &PrettyPath {
        &self.repo_cache_path
    }

    pub(in crate::presentation) fn root_map(&self) -> &RootContextMap {
        &self.root_map
    }

    pub(in crate::presentation) fn query(&self) -> &QueryContext {
        &self.query
    }
}

fn load_config(path: &OptionalParam<PrettyPath>) -> Result<Config> {
    match file::load_toml(path.name(), path.value())? {
        Some(config) => Ok(config),
        None if path.is_default() => Ok(Config::default()),
        None => bail!(eyre!("config file not found: {}", path.value().display())),
    }
}
