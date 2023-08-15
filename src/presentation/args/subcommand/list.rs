use std::{
    io::{self, Write},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{
    application::service::walk_repo::{self, WalkRepo},
    presentation::{
        args::GlobalArgs,
        config::{self, Config},
        util::{dwym_fs, optional_param::OptionalParam, tilde_path::TildePath},
    },
};

#[derive(Debug, Clone, Default, clap::Args)]
pub(super) struct Args {
    /// Path of the root directory under which the repository will be cloned
    #[clap(long = "root")]
    root_path: Option<Vec<PathBuf>>,

    /// Output as JSON
    #[clap(long)]
    json: bool,
}

impl Args {
    fn root_confs(&self, config: &Config) -> Vec<RootConfig> {
        let confs: Vec<_> = if let Some(root_paths) = &self.root_path {
            root_paths
                .iter()
                .enumerate()
                .map(|(i, path)| {
                    let name = format!("arg{i}");
                    let config = config::Root::new(TildePath::from_expanded(path));
                    (name, config)
                })
                .collect()
        } else {
            config
                .root_map()
                .map()
                .iter()
                .map(|(name, config)| (name.clone(), config.clone()))
                .collect()
        };

        confs
            .into_iter()
            .filter_map(|(name, config)| RootConfig::new(name.clone(), config.path()))
            .collect()
    }

    pub(super) fn run(&self, global_args: &GlobalArgs) -> Result<()> {
        let config = global_args.config()?;
        let root_confs = self.root_confs(&config);

        if self.json {
            emit_json(root_confs)?;
        } else {
            emit_text(root_confs)?;
        }

        Ok(())
    }
}

fn emit_json(root_confs: Vec<RootConfig>) -> Result<()> {
    let list = List {
        roots: root_confs.into_iter().map(Root::new).collect(),
    };

    let out = io::stdout();
    let mut out = out.lock();
    serde_json::to_writer(&mut out, &list)?;
    out.flush()?;
    Ok(())
}

fn emit_text(root_confs: Vec<RootConfig>) -> Result<()> {
    for root_config in root_confs {
        for repo in root_config.repos() {
            println!("{}", repo.absolute_path.display());
        }
    }

    Ok(())
}

#[derive(Debug)]
struct RootConfig {
    name: String,
    display_path: PathBuf,
    absolute_path: PathBuf,
}

impl RootConfig {
    fn new(name: String, path: &OptionalParam<TildePath>) -> Option<Self> {
        let display_path: PathBuf = path.value().as_display_path().to_owned();
        let absolute_path = match dwym_fs::canonicalize(path) {
            Ok(path) => path,
            Err(e) => {
                tracing::warn!("{e}");
                return None;
            }
        }?;

        Some(Self {
            name,
            display_path,
            absolute_path,
        })
    }

    fn repos(&self) -> impl Iterator<Item = Repo> + '_ {
        WalkRepo::new(&self.absolute_path)
            .into_iter()
            .filter_map(|res| match res {
                Ok(repo) => Some(repo),
                Err(e) => {
                    tracing::warn!("failed to traverse directory: {e}");
                    None
                }
            })
            .map(Repo::from)
    }
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
    fn new(config: RootConfig) -> Self {
        let repos = config.repos().collect();
        let RootConfig {
            name,
            display_path,
            absolute_path,
        } = config;

        Self {
            name,
            display_path,
            absolute_path,
            repos,
        }
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
