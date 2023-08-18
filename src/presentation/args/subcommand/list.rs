use std::{
    io::{self, Write},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{
    application::service::Service,
    domain::model::{
        display_path::DisplayPath,
        path_like::PathLike,
        repo::CanonicalRepo,
        root::{CanonicalRoot, Root},
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
    fn roots(&self, config: &Config) -> Vec<OptionalParam<Root>> {
        if let Some(root_paths) = &self.root_path {
            root_paths
                .iter()
                .enumerate()
                .map(|(i, path)| {
                    let name = format!("arg{i}");
                    let root = Root::new(
                        name,
                        DisplayPath::from_pathlike(&TildePath::from_expanded(path)),
                    );
                    OptionalParam::new_explicit("root", root)
                })
                .collect()
        } else {
            config.roots()
        }
    }

    pub(super) fn run(&self, global_args: &GlobalArgs, service: &Service) -> Result<()> {
        let config = global_args.config()?;
        let roots = self.roots(&config);

        let root_service = service.root();
        let roots = roots.into_iter().filter_map(|root| {
            let should_exist = root.is_explicit();
            match root_service.canonicalize_root(root.value(), should_exist) {
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
        let repos_in_root = move |root: &CanonicalRoot| {
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
    Roots: Iterator<Item = CanonicalRoot>,
    F: Fn(&CanonicalRoot) -> Repos,
    Repos: Iterator<Item = CanonicalRepo>,
{
    let list = JsonList {
        roots: roots
            .map(|root| JsonRoot {
                name: root.name().to_owned(),
                display_path: root.path().as_display_path().to_owned(),
                real_path: root.path().as_real_path().to_owned(),
                canonical_path: root.canonical_path().to_owned(),
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
    Roots: Iterator<Item = CanonicalRoot>,
    F: Fn(&CanonicalRoot) -> Repos,
    Repos: Iterator<Item = CanonicalRepo>,
{
    for root in roots {
        for repo in repos_in_root(&root) {
            println!("{}", repo.canonical_path().display());
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
    real_path: PathBuf,
    display_path: PathBuf,
    canonical_path: PathBuf,
    repos: Vec<JsonRepo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRepo {
    relative_path: PathBuf,
    real_path: PathBuf,
    display_path: PathBuf,
    canonical_path: PathBuf,
}

impl From<CanonicalRepo> for JsonRepo {
    fn from(value: CanonicalRepo) -> Self {
        Self {
            relative_path: value.relative_path().as_real_path().to_owned(),
            display_path: value.path().as_display_path().to_owned(),
            real_path: value.path().as_real_path().to_owned(),
            canonical_path: value.canonical_path().to_owned(),
        }
    }
}
