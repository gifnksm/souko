use color_eyre::eyre::{Result, bail};

use crate::{
    app_dirs::AppDirs,
    application::usecase::Usecases,
    domain::model::{path_like::PathLike as _, pretty_path::PrettyPath},
    presentation::{
        args::Args,
        config::Config,
        context::{query::QueryContext, root::RootContextMap},
        model::app_param::AppParam,
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
        let config_path = args.global_args().config_path(&app_dirs);
        let config_path = config_path
            .as_ref()
            .map(|path| path.normalize(config_path.source(), &app_dirs));
        let config = load_config(&config_path)?;
        let root_map = RootContextMap::new(config_path.value(), &config.roots, &app_dirs);
        let query = QueryContext::from_config(&config.query);
        let repo_cache_path = args.global_args().repo_cache_path(&app_dirs);
        let repo_cache_path = repo_cache_path
            .value()
            .normalize(repo_cache_path.source(), &app_dirs);
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

fn load_config(path: &AppParam<PrettyPath>) -> Result<Config> {
    match file::load_toml("configuration file", path.value())? {
        Some(config) => Ok(config),
        None if path.source().is_implicit_default() => Ok(Config::default()),
        None => bail!("config file not found: {}", path.value().display()),
    }
}
