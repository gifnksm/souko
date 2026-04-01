use std::{
    collections::HashMap,
    io::{self, Write},
    path::{Path, PathBuf},
};

use chrono::{Duration, Utc};
use color_eyre::eyre::{eyre, Result};
use serde::Serialize;

use crate::{
    application::service::Service,
    domain::model::{
        path_like::PathLike,
        repo::CanonicalRepo,
        root::{CanonicalRoot, Root},
        template::Template,
    },
    presentation::{
        args::GlobalArgs,
        config::Config,
        model::{optional_param::OptionalParam, project_dirs::ProjectDirs},
    },
};

#[derive(Debug, Clone, Default, clap::Args)]
pub(super) struct Args {
    /// List repositories only under the specified root (repeatable)
    #[arg(long = "root")]
    root_name: Option<Vec<String>>,

    #[command(flatten)]
    format: FormatArgs,
}

#[derive(Debug, Clone, Default, clap::Args)]
#[group(id = "format", multiple = false)]
struct FormatArgs {
    /// Output repositories as JSON
    #[arg(long)]
    json: bool,
    /// Output each repository using a template string
    #[arg(long)]
    template: Option<Template>,
}

#[derive(Debug, Clone, Default)]
enum Format {
    #[default]
    Default,
    Json,
    Template(Template),
}

impl From<FormatArgs> for Format {
    fn from(value: FormatArgs) -> Self {
        let FormatArgs { json, template } = value;
        if json {
            Self::Json
        } else if let Some(template) = template {
            Self::Template(template)
        } else {
            Self::Default
        }
    }
}

impl Args {
    fn roots(
        &self,
        config: &Config,
        project_dirs: &ProjectDirs,
    ) -> Result<Vec<OptionalParam<Root>>> {
        let roots = if let Some(root_names) = &self.root_name {
            let roots = config.roots(project_dirs);
            root_names
                .iter()
                .map(|name| {
                    roots
                        .get(name)
                        .cloned()
                        .ok_or_else(|| eyre!("root `{name}` not found in config file"))
                })
                .collect::<Result<_>>()?
        } else {
            config.roots(project_dirs).values().cloned().collect()
        };
        Ok(roots)
    }

    pub(super) fn run(
        &self,
        global_args: &GlobalArgs,
        service: &Service,
        project_dirs: &ProjectDirs,
    ) -> Result<()> {
        let config = global_args.config(project_dirs)?;
        let roots = self.roots(&config, project_dirs)?;

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

        // TODO: make this configurable
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

        // TODO: make this configurable
        let now = Utc::now();
        let cache_expire_duration = Duration::try_days(3).unwrap();

        let repo_cache_path = global_args.repo_cache_path(project_dirs);
        root_service.load_repo_cache(repo_cache_path.value(), now, cache_expire_duration);

        let res = match self.format.clone().into() {
            Format::Default => emit_text(roots, repos_in_root),
            Format::Json => emit_json(roots, repos_in_root),
            Format::Template(template) => emit_template(roots, repos_in_root, template),
        };

        root_service.store_repo_cache(repo_cache_path.value());

        res
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

fn format_path(path: &Path) -> String {
    path.display().to_string()
}

fn template_variables(root: &CanonicalRoot, repo: &CanonicalRepo) -> HashMap<&'static str, String> {
    HashMap::from_iter([
        ("root_name", root.name().to_string()),
        (
            "root_display_path",
            format_path(root.path().as_display_path()),
        ),
        ("root_real_path", format_path(root.path().as_real_path())),
        ("root_canonical_path", format_path(root.canonical_path())),
        (
            "repo_relative_path",
            format_path(repo.relative_path().as_real_path()),
        ),
        (
            "repo_display_path",
            format_path(repo.path().as_display_path()),
        ),
        ("repo_real_path", format_path(repo.path().as_real_path())),
        ("repo_canonical_path", format_path(repo.canonical_path())),
    ])
}

fn emit_template<Roots, F, Repos>(roots: Roots, repos_in_root: F, template: Template) -> Result<()>
where
    Roots: Iterator<Item = CanonicalRoot>,
    F: Fn(&CanonicalRoot) -> Repos,
    Repos: Iterator<Item = CanonicalRepo>,
{
    for root in roots {
        for repo in repos_in_root(&root) {
            let variables = template_variables(&root, &repo);
            println!("{}", template.expand(&variables));
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
