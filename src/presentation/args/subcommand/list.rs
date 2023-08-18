use std::{
    io::{self, Write},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{
    application::service::Service,
    domain::model::{
        path_like::PathLike,
        repo::Repo,
        root::{Root, RootSpec},
    },
    presentation::{
        args::GlobalArgs,
        config::Config,
        util::{optional_param::OptionalParam, tilde_path::TildePath},
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
    fn root_specs(&self, config: &Config) -> Vec<OptionalParam<RootSpec>> {
        if let Some(root_paths) = &self.root_path {
            root_paths
                .iter()
                .enumerate()
                .map(|(i, path)| {
                    let name = format!("arg{i}");
                    let spec = RootSpec::new(name, Box::new(TildePath::from_expanded(path)));
                    OptionalParam::new_explicit("root", spec)
                })
                .collect()
        } else {
            config.root_specs()
        }
    }

    pub(super) fn run(&self, global_args: &GlobalArgs, service: &Service) -> Result<()> {
        let config = global_args.config()?;
        let root_specs = self.root_specs(&config);

        let root_service = service.root();
        let roots = root_specs.into_iter().filter_map(|spec| {
            let should_exist = spec.is_explicit();
            match root_service.resolve_root(spec.value(), should_exist) {
                Ok(root) => root,
                Err(e) => {
                    tracing::warn!("{e}");
                    None
                }
            }
        });

        let skip_hidden = true;
        let skip_bare = true;
        let no_recursive = true;
        let repos_in_root = move |root: &Root| {
            let repos = match root_service.find_repos(root, skip_hidden, skip_bare, no_recursive) {
                Ok(repos) => Some(repos),
                Err(e) => {
                    tracing::warn!("{e}");
                    None
                }
            };

            repos.into_iter().flatten().filter_map(|res| match res {
                Ok(repo) => Some(repo),
                Err(e) => {
                    tracing::warn!("{e}");
                    None
                }
            })
        };

        if self.json {
            emit_json(roots, repos_in_root)?;
        } else {
            emit_text(roots, repos_in_root)?;
        }

        Ok(())
    }
}

fn emit_json<Roots, F, Repos>(roots: Roots, repos_in_root: F) -> Result<()>
where
    Roots: Iterator<Item = Root>,
    F: Fn(&Root) -> Repos,
    Repos: Iterator<Item = Repo>,
{
    let list = JsonList {
        roots: roots
            .map(|root| JsonRoot {
                name: root.name().to_owned(),
                path: root.path().as_path().to_owned(),
                display_path: root.path().as_display_path().to_owned(),
                repos: repos_in_root(&root).map(JsonRepo::from).collect(),
            })
            .collect(),
    };

    let out = io::stdout();
    let mut out = out.lock();
    serde_json::to_writer(&mut out, &list)?;
    out.flush()?;
    Ok(())
}

fn emit_text<Roots, F, Repos>(roots: Roots, repos_in_root: F) -> Result<()>
where
    Roots: Iterator<Item = Root>,
    F: Fn(&Root) -> Repos,
    Repos: Iterator<Item = Repo>,
{
    for root in roots {
        for repo in repos_in_root(&root) {
            println!("{}", repo.path().as_path().display());
        }
    }

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonList {
    roots: Vec<JsonRoot>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRoot {
    name: String,
    display_path: PathBuf,
    path: PathBuf,
    repos: Vec<JsonRepo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRepo {
    relative_path: PathBuf,
    display_path: PathBuf,
    path: PathBuf,
}

impl From<Repo> for JsonRepo {
    fn from(value: Repo) -> Self {
        Self {
            relative_path: value.relative_path().as_path().to_owned(),
            display_path: value.path().as_display_path().to_owned(),
            path: value.path().as_path().to_owned(),
        }
    }
}
