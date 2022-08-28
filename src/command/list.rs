use std::{
    io::{self, Write},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{
    cli::subcommand::list::Args,
    config, fs,
    walk_repo::{self, WalkRepo},
    App,
};

pub(super) fn run(app: &App, args: &Args) -> Result<()> {
    let config = app.config()?;
    let root_paths = args.root_paths(&config);

    let roots = root_paths
        .into_iter()
        .filter_map(|(name, root)| Root::new(name, root))
        .collect::<Vec<_>>();
    let list = List { roots };

    if args.json() {
        emit_json(&list)?;
    } else {
        emit_text(&list)?;
    }

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct List {
    roots: Vec<Root>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    name: String,
    display_path: PathBuf,
    absolute_path: PathBuf,
    repos: Vec<Repo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Repo {
    name: PathBuf,
    display_path: PathBuf,
    absolute_path: PathBuf,
}

impl Root {
    fn new(name: impl Into<String>, config: impl Into<config::Root>) -> Option<Self> {
        let name = name.into();
        let config = config.into();
        let path = config.path();
        let display_path = path.value().as_display_path().to_owned();
        let absolute_path = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(e) => {
                tracing::warn!("failed to get absolute path of {}: {}", path.name(), e);
                return None;
            }
        }?;

        let repos = WalkRepo::new(path.value().clone())
            .into_iter()
            .filter_map(|res| {
                if let Err(e) = &res {
                    tracing::warn!("failed to traverse directory: {e}");
                }
                res.ok()
            })
            .map(Repo::from)
            .collect();

        Some(Self {
            name,
            display_path,
            absolute_path,
            repos,
        })
    }
}

impl From<&walk_repo::Repo> for Repo {
    fn from(repo: &walk_repo::Repo) -> Self {
        Repo {
            name: repo.name().to_owned(),
            display_path: repo.display_path().to_owned(),
            absolute_path: repo.absolute_path().to_owned(),
        }
    }
}

impl From<walk_repo::Repo> for Repo {
    fn from(repo: walk_repo::Repo) -> Self {
        Self::from(&repo)
    }
}

fn emit_json(list: &List) -> Result<()> {
    let out = io::stdout();
    let mut out = out.lock();
    serde_json::to_writer(&mut out, list)?;
    out.flush()?;
    Ok(())
}

fn emit_text(list: &List) -> Result<()> {
    for root in &list.roots {
        for repo in &root.repos {
            println!("{}", repo.absolute_path.display());
        }
    }
    Ok(())
}
