use color_eyre::eyre::Result;

use crate::{cli::subcommand::list::Args, App, RepoIndex};

pub(super) fn run(app: &App, _args: &Args) -> Result<()> {
    let repo_index_path = app.repo_index();
    let repo_index = repo_index_path
        .load_json::<RepoIndex>()?
        .unwrap_or_default();

    for repo in repo_index.repos() {
        println!("{}", repo.path().display());
    }

    Ok(())
}
