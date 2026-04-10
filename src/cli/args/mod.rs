use tracing::Level;

use self::verbosity::Verbosity;
use super::input::{app_param::AppParam, unresolved_path::UnresolvedPath};
use crate::{
    app_dirs::AppDirs,
    cli::{
        args::{clone::CloneArgs, list::ListArgs},
        input::app_param::AppParamSource,
    },
};

pub(in crate::cli) mod clone;
pub(in crate::cli) mod list;
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

    pub(in crate::cli) fn subcommand(&self) -> Option<&Subcommand> {
        self.subcommand.as_ref()
    }
}

#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct GlobalArgs {
    #[command(flatten)]
    verbosity: Verbosity,

    /// Path to souko config file
    #[arg(long = "config", env = "SOUKO_CONFIG")]
    config_path: Option<UnresolvedPath>,

    /// Path to souko repository cache directory
    #[arg(long = "repo-cache", env = "SOUKO_REPO_CACHE")]
    repo_cache_path: Option<UnresolvedPath>,
}

impl GlobalArgs {
    pub(crate) fn verbosity(&self) -> Option<Level> {
        self.verbosity.verbosity()
    }

    pub(in crate::cli) fn config_path(&self, app_dirs: &AppDirs) -> AppParam<UnresolvedPath> {
        let (source, value) = self
            .config_path
            .as_ref()
            .map(|path| (AppParamSource::CommandLineArgument, path.clone()))
            .unwrap_or_else(|| {
                (
                    AppParamSource::ImplicitDefault,
                    UnresolvedPath::new(app_dirs.config_dir().join("config.toml")),
                )
            });
        AppParam::new(source, value)
    }

    pub(in crate::cli) fn repo_cache_path(&self, app_dirs: &AppDirs) -> AppParam<UnresolvedPath> {
        let (source, value) = self
            .repo_cache_path
            .as_ref()
            .map(|path| (AppParamSource::CommandLineArgument, path.clone()))
            .unwrap_or_else(|| {
                (
                    AppParamSource::ImplicitDefault,
                    UnresolvedPath::new(app_dirs.cache_dir().join("repos.json")),
                )
            });
        AppParam::new(source, value)
    }
}

#[derive(Debug, Clone, clap::Subcommand)]
pub(in crate::cli) enum Subcommand {
    /// Clone remote repositories and put them into souko
    Clone(CloneArgs),
    /// List repositories in souko
    List(ListArgs),
}
