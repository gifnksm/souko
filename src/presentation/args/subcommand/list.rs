use std::{
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
        template::{Template, TemplateContext},
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

impl FormatArgs {
    fn validate(&self) -> Result<Format> {
        let FormatArgs { json, template } = self;
        if *json {
            Ok(Format::Json)
        } else if let Some(template) = template {
            template.validate::<RepoListTemplateContext>()?;
            Ok(Format::Template(template.clone()))
        } else {
            Ok(Format::Default)
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
        let format = self.format.validate()?;
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

        let res = match format {
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

#[derive(Debug, Clone, Serialize)]
struct RepoListTemplateContext<'a> {
    root_name: &'a str,
    root_display_path: &'a Path,
    root_real_path: &'a Path,
    root_canonical_path: &'a Path,
    repo_relative_path: &'a Path,
    repo_display_path: &'a Path,
    repo_real_path: &'a Path,
    repo_canonical_path: &'a Path,
}

impl<'a> Default for RepoListTemplateContext<'a> {
    fn default() -> Self {
        Self {
            root_name: Default::default(),
            root_display_path: Path::new(""),
            root_real_path: Path::new(""),
            root_canonical_path: Path::new(""),
            repo_relative_path: Path::new(""),
            repo_display_path: Path::new(""),
            repo_real_path: Path::new(""),
            repo_canonical_path: Path::new(""),
        }
    }
}

impl<'a> TemplateContext for RepoListTemplateContext<'a> {}

impl<'a> RepoListTemplateContext<'a> {
    fn new(root: &'a CanonicalRoot, repo: &'a CanonicalRepo) -> Self {
        Self {
            root_name: root.name(),
            root_display_path: root.path().as_display_path(),
            root_real_path: root.path().as_real_path(),
            root_canonical_path: root.canonical_path(),
            repo_relative_path: repo.relative_path(),
            repo_display_path: repo.path().as_display_path(),
            repo_real_path: repo.path().as_real_path(),
            repo_canonical_path: repo.canonical_path(),
        }
    }
}

fn emit_template<Roots, F, Repos>(roots: Roots, repos_in_root: F, template: Template) -> Result<()>
where
    Roots: Iterator<Item = CanonicalRoot>,
    F: Fn(&CanonicalRoot) -> Repos,
    Repos: Iterator<Item = CanonicalRepo>,
{
    for root in roots {
        for repo in repos_in_root(&root) {
            let context = RepoListTemplateContext::new(&root, &repo);
            println!("{}", template.expand(&context));
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
