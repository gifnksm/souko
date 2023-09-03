use color_eyre::eyre::{eyre, Result};
use tracing::Level;

use self::{subcommand::Subcommand, verbosity::Verbosity};
use super::{
    config::Config,
    util::{file, optional_param::OptionalParam, project_dirs::ProjectDirs, tilde_path::TildePath},
};
use crate::{application::service::Service, domain::model::path_like::PathLike};

mod subcommand;
mod verbosity;

#[derive(Debug, Clone, Default, clap::Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(flatten)]
    global_args: GlobalArgs,

    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, Clone, Default, clap::Args)]
struct GlobalArgs {
    #[clap(flatten)]
    verbosity: Verbosity,

    /// Path to souko config file
    #[clap(long = "config", env = "SOUKO_CONFIG", value_parser = TildePath::parse_real_path)]
    config_path: Option<TildePath>,
}

impl GlobalArgs {
    fn config_path(&'_ self) -> OptionalParam<TildePath> {
        OptionalParam::new("config", self.config_path.clone(), || {
            TildePath::from_real_path(ProjectDirs::get().config_dir().join("config.toml"))
        })
    }

    fn config(&self) -> Result<Config> {
        let config_path = self.config_path();
        let config = match file::load_toml(config_path.name(), config_path.value())? {
            Some(config) => config,
            None if config_path.is_default() => Config::default(),
            None => bail!(eyre!(
                "config file not found: {}",
                config_path.value().display()
            )),
        };
        Ok(config)
    }
}

impl Args {
    pub(super) fn verbosity(&self) -> Option<Level> {
        self.global_args.verbosity.verbosity()
    }

    pub(super) fn run(&self, service: &Service) -> Result<()> {
        match &self.subcommand {
            Some(subcommand) => subcommand.run(&self.global_args, service)?,
            None => <Args as clap::CommandFactory>::command().print_help()?,
        }
        Ok(())
    }
}
