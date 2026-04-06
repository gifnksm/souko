use tracing::Level;

use self::verbosity::Verbosity;
use super::model::{optional_param::OptionalParam, tilde_path::TildePath};
use crate::{
    presentation::args::{clone::CloneArgs, list::ListArgs},
    project_dirs::ProjectDirs,
};

pub(in crate::presentation) mod clone;
pub(in crate::presentation) mod list;
mod verbosity;

#[derive(Debug, Clone, Default, clap::Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub(crate) struct Args {
    #[command(flatten)]
    global_args: GlobalArgs,

    #[command(subcommand)]
    subcommand: Option<Subcommand>,
}

impl Args {
    pub(crate) fn global_args(&self) -> &GlobalArgs {
        &self.global_args
    }

    pub(in crate::presentation) fn subcommand(&self) -> Option<&Subcommand> {
        self.subcommand.as_ref()
    }
}

#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct GlobalArgs {
    #[command(flatten)]
    verbosity: Verbosity,

    /// Path to souko config file
    #[arg(long = "config", env = "SOUKO_CONFIG", value_parser = TildePath::parse_real_path)]
    config_path: Option<TildePath>,

    /// Path to souko repository cache directory
    #[arg(long = "repo-cache", env = "SOUKO_REPO_CACHE", value_parser = TildePath::parse_real_path)]
    repo_cache_path: Option<TildePath>,
}

impl GlobalArgs {
    pub(crate) fn verbosity(&self) -> Option<Level> {
        self.verbosity.verbosity()
    }

    pub(in crate::presentation) fn config_path(
        &self,
        project_dirs: &ProjectDirs,
    ) -> OptionalParam<TildePath> {
        OptionalParam::new("config", self.config_path.clone(), || {
            TildePath::from_real_path(project_dirs.config_dir().join("config.toml"))
        })
    }

    pub(in crate::presentation) fn repo_cache_path(
        &self,
        project_dirs: &ProjectDirs,
    ) -> OptionalParam<TildePath> {
        OptionalParam::new("repo-cache", self.repo_cache_path.clone(), || {
            TildePath::from_real_path(project_dirs.cache_dir().join("repos.json"))
        })
    }
}

#[derive(Debug, Clone, clap::Subcommand)]
pub(in crate::presentation) enum Subcommand {
    /// Clone remote repositories and put them into souko
    Clone(CloneArgs),
    /// List repositories in souko
    List(ListArgs),
}
