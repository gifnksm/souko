use color_eyre::eyre::Result;
use souko::RepoIndex;

use crate::cli::Args;

pub(super) fn run(args: &Args) -> Result<()> {
    let repo_index_path = args.repo_index();
    let repo_index = repo_index_path
        .load_json::<RepoIndex>()?
        .unwrap_or_default();

    for repo in repo_index.repos() {
        println!("{}", repo.path().display());
    }

    Ok(())
}
