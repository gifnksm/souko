use color_eyre::eyre::Result;

use crate::{args, Args, Repo, RepoIndex, WalkRepo};

pub(super) fn run(args: &Args, import_args: &args::Import) -> Result<()> {
    let repo_index_path = args.repo_index();

    let mut repo_index = repo_index_path
        .load_json::<RepoIndex>()?
        .unwrap_or_default();

    let mut updated = false;

    for repo_path in import_args.repos() {
        if import_args.recursive() {
            let walk_dir = walkdir::WalkDir::new(repo_path);
            let walk_repo = WalkRepo::new(walk_dir).include_hidden(import_args.hidden());
            for repo in walk_repo {
                let repo = match repo {
                    Ok(repo) => repo,
                    Err(e) => {
                        tracing::warn!(error = %e, "failed to import repository under {}", repo_path.display());
                        continue;
                    }
                };
                if import_repo(&mut repo_index, repo) {
                    updated = true;
                }
            }
        } else {
            let repo = match Repo::read(repo_path) {
                Ok(repo) => repo,
                Err(e) => {
                    tracing::warn!(error = %e, "failed to import as git repository: {}", repo_path.display());
                    continue;
                }
            };
            if import_repo(&mut repo_index, repo) {
                updated = true;
            }
        };
    }

    if updated && !import_args.dry_run() {
        repo_index_path.store_json(&repo_index)?;
    }

    Ok(())
}

pub(crate) fn import_repo(repo_index: &mut RepoIndex, repo: Repo) -> bool {
    let repo_path = repo.path().to_owned();
    let imported = repo_index.push(repo);
    if imported {
        tracing::info!("importing repository {}", repo_path.display());
    } else {
        tracing::warn!(
            "repository already exists in the index: {}",
            repo_path.display()
        );
    }
    imported
}
