use std::io;

use color_eyre::eyre::Result;

use crate::{cli::subcommand::list::Args, App, RepoIndex};

pub(super) fn run(app: &App, args: &Args) -> Result<()> {
    let repo_index_path = app.repo_index();
    let repo_index = repo_index_path
        .load_json::<RepoIndex>()?
        .unwrap_or_default();

    if args.json() {
        emit_json(&repo_index)?;
    } else {
        emit_text(&repo_index)?;
    }

    Ok(())
}

fn emit_json(repo_index: &RepoIndex) -> Result<()> {
    serde_json::to_writer(io::stdout(), repo_index)?;
    Ok(())
}

fn emit_text(repo_index: &RepoIndex) -> Result<()> {
    for repo in repo_index.repos() {
        println!("{}", repo.path().display());
    }
    Ok(())
}
