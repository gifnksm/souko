use std::path::PathBuf;

use color_eyre::eyre::Result;
use tracing::Level;

use self::{subcommand::Subcommand, verbosity::Verbosity};
use super::{
    config::Config,
    util::{dwym_fs, optional_param::OptionalParam, project_dirs::ProjectDirs},
};

pub(crate) mod subcommand;
pub(crate) mod verbosity;

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
    #[clap(long = "config", env = "SOUKO_CONFIG")]
    config_path: Option<PathBuf>,
}

impl GlobalArgs {
    fn config_path(&'_ self) -> OptionalParam<PathBuf> {
        OptionalParam::new("config", self.config_path.clone(), || {
            ProjectDirs::get().config_dir().join("config.toml")
        })
    }

    fn config(&self) -> Result<Config> {
        let config_path = self.config_path();
        let config = dwym_fs::load_toml(&config_path)?.unwrap_or_default();
        Ok(config)
    }
}

impl Args {
    pub fn verbosity(&self) -> Option<Level> {
        self.global_args.verbosity.verbosity()
    }

    pub(crate) fn run(&self) -> Result<()> {
        match &self.subcommand {
            Some(subcommand) => subcommand.run(&self.global_args)?,
            None => <Args as clap::CommandFactory>::command().print_help()?,
        }
        Ok(())
    }
}
