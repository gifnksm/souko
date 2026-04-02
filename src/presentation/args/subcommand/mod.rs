use color_eyre::eyre::Result;

use super::GlobalArgs;
use crate::{application::service::Service, project_dirs::ProjectDirs};

mod clone;
mod list;

#[derive(Debug, Clone, clap::Subcommand)]
pub(super) enum Subcommand {
    /// Clone remote repositories and put them into souko
    Clone(CloneArgs),
    /// List repositories in souko
    List(ListArgs),
}

impl Subcommand {
    pub(super) fn run(
        &self,
        global_args: &GlobalArgs,
        service: &Service,
        project_dirs: &ProjectDirs,
    ) -> Result<()> {
        match self {
            Self::Clone(args) => args.inner.run(global_args, service, project_dirs),
            Self::List(args) => args.inner.run(global_args, service, project_dirs),
        }
    }
}

// To prevent leak of clone::Args and list::Args, we wrap them with our own Args
#[derive(Debug, Clone, clap::Args)]
pub(super) struct CloneArgs {
    #[command(flatten)]
    inner: clone::Args,
}
#[derive(Debug, Clone, clap::Args)]
pub(super) struct ListArgs {
    #[command(flatten)]
    inner: list::Args,
}
