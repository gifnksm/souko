use color_eyre::eyre::{Result, eyre};

use crate::{
    application::usecase::Usecases,
    domain::model::{path_like::PathLike as _, root::Root},
    presentation::{
        args::Args,
        config::Config,
        model::{optional_param::OptionalParam, tilde_path::TildePath},
    },
    project_dirs::ProjectDirs,
    util::file,
};

#[derive(Debug)]
pub(in crate::presentation) struct GlobalContext {
    usecases: Usecases,
    project_dirs: ProjectDirs,
    config: Config,
    repo_cache_path: TildePath,
}

impl GlobalContext {
    pub(in crate::presentation) fn new(
        args: &Args,
        usecases: Usecases,
        project_dirs: ProjectDirs,
    ) -> Result<Self> {
        let config_path = args.global_args().config_path(&project_dirs);
        let config = load_config(config_path.as_ref())?;
        let repo_cache_path = args
            .global_args()
            .repo_cache_path(&project_dirs)
            .value()
            .clone();
        Ok(Self {
            usecases,
            project_dirs,
            config,
            repo_cache_path,
        })
    }

    pub(in crate::presentation) fn usecases(&self) -> &Usecases {
        &self.usecases
    }

    pub(in crate::presentation) fn config(&self) -> &Config {
        &self.config
    }

    pub(in crate::presentation) fn repo_cache_path(&self) -> &TildePath {
        &self.repo_cache_path
    }

    pub(in crate::presentation) fn default_root(&self) -> OptionalParam<Root> {
        self.config.default_root(&self.project_dirs)
    }

    pub(in crate::presentation) fn root_by_name(&self, name: &str) -> Result<OptionalParam<Root>> {
        let roots = self.config.roots(&self.project_dirs);
        roots
            .get(name)
            .cloned()
            .ok_or_else(|| eyre!("root `{name}` not found in config file"))
    }

    pub(in crate::presentation) fn roots_by_names(
        &self,
        names: &[String],
    ) -> Result<Vec<OptionalParam<Root>>> {
        let roots = self.config.roots(&self.project_dirs);
        names
            .iter()
            .map(|name| {
                roots
                    .get(name)
                    .cloned()
                    .ok_or_else(|| eyre!("root `{name}` not found in config file"))
            })
            .collect()
    }

    pub(in crate::presentation) fn all_roots(&self) -> Vec<OptionalParam<Root>> {
        self.config
            .roots(&self.project_dirs)
            .values()
            .cloned()
            .collect()
    }
}

fn load_config(path: OptionalParam<&TildePath>) -> Result<Config> {
    match file::load_toml(path.name(), path.value())? {
        Some(config) => Ok(config),
        None if path.is_default() => Ok(Config::default()),
        None => bail!(eyre!("config file not found: {}", path.value().display())),
    }
}
