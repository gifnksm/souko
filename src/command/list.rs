use color_eyre::eyre::Result;

use crate::{args::List, Args, RepoIndex};

pub(super) fn run(args: &Args, _list_args: &List) -> Result<()> {
    let repo_index_path = args.repo_index();
    let repo_index = repo_index_path
        .load_json::<RepoIndex>()?
        .unwrap_or_default();

    for repo in repo_index.repos() {
        println!("{}", repo.path().display());
    }

    Ok(())
}
