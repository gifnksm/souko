use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, WrapErr};
use url::Url;

use crate::{args, Args, Config, Query, Repo, RepoIndex};

pub(super) fn run(args: &Args, clone_args: &args::Clone) -> Result<()> {
    let config_path = args.config();
    let config = config_path.load_toml::<Config>()?.unwrap_or_default();

    let root_path = clone_args.root(&config);
    let query = clone_args.query();

    let query_config = config.query_config();
    let query =
        Query::parse(query, &query_config).wrap_err_with(|| format!("invalid query: {}", query))?;

    let dest_path = make_dest_path(root_path.value(), query.url());
    fs::create_dir_all(&dest_path).wrap_err_with(|| {
        format!(
            "failed to create destination directory: {}",
            dest_path.display()
        )
    })?;

    tracing::debug!(query = %query.original_query(), url = %query.url(), dest = %dest_path.display());

    tracing::info!(
        "Cloning {} into {}",
        query.original_query(),
        dest_path.display()
    );

    let repo = git2::Repository::clone(query.url().as_str(), &dest_path)
        .wrap_err_with(|| format!("failed to clone git repository from {}", query.url()))?;
    let repo = Repo::try_from(&repo).expect("BUG: failed to convert git2::Repository to Repo");

    let repo_index_path = args.repo_index();
    let mut repo_index = repo_index_path
        .load_json::<RepoIndex>()?
        .unwrap_or_default();

    let mut updated = false;
    if super::import::import_repo(&mut repo_index, repo) {
        updated = true;
    }
    if updated {
        repo_index_path.store_json(&repo_index)?;
    }

    Ok(())
}

fn make_dest_path(root_path: &Path, url: &Url) -> PathBuf {
    let mut dest_path = root_path.to_owned();
    if let Some(host) = url.host_str() {
        if let Some(port) = url.port() {
            dest_path.push(format!("{}:{}", host, port));
        } else {
            dest_path.push(host);
        }
    }

    let mut base_url = url.clone();
    base_url.set_path("");
    let relative = base_url.make_relative(url);
    if let Some(relative) = relative {
        let relative = relative.trim_end_matches(".git");
        for part in relative.split('/') {
            dest_path.push(part);
        }
    }

    dest_path
}
