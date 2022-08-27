use std::{
    io::{self, Write},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{
    cli::subcommand::list::Args,
    config,
    walk_repo::{self, WalkRepo},
    App,
};

pub(super) fn run(app: &App, args: &Args) -> Result<()> {
    let config = app.config()?;
    let root_paths = args.root_paths(&config);

    let roots = root_paths
        .into_iter()
        .map(|(name, root)| Root::new(name, root))
        .collect::<Vec<_>>();

    if args.json() {
        emit_json(&roots)?;
    } else {
        emit_text(&roots)?;
    }

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    name: String,
    path: PathBuf,
    absolute_path: PathBuf,
    repos: Vec<Repo>,
}

impl Root {
    fn new(name: impl Into<String>, config: impl Into<config::Root>) -> Self {
        let name = name.into();
        let config = config.into();
        let path = config.path().to_owned();
        let absolute_path = path.canonicalize().unwrap_or_else(|_e| path.to_owned());
        let repos = WalkRepo::new(&path)
            .into_iter()
            .filter_map(|res| {
                if let Err(e) = &res {
                    tracing::warn!("failed to traverse directory: {e}");
                }
                res.ok()
            })
            .map(Repo::from)
            .collect();
        Self {
            name,
            path,
            absolute_path,
            repos,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Repo {
    name: PathBuf,
    absolute_path: PathBuf,
}

impl From<&walk_repo::Repo> for Repo {
    fn from(repo: &walk_repo::Repo) -> Self {
        Repo {
            name: repo.name().to_owned(),
            absolute_path: repo.absolute_path().to_owned(),
        }
    }
}

impl From<walk_repo::Repo> for Repo {
    fn from(repo: walk_repo::Repo) -> Self {
        Self::from(&repo)
    }
}

fn emit_json(roots: &[Root]) -> Result<()> {
    let out = io::stdout();
    let mut out = out.lock();
    serde_json::to_writer(&mut out, roots)?;
    out.flush()?;
    Ok(())
}

fn emit_text(roots: &[Root]) -> Result<()> {
    for root in roots {
        for repo in &root.repos {
            println!("{}", repo.absolute_path.display());
        }
    }
    Ok(())
}
