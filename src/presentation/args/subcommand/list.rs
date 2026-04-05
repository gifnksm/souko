use std::{
    fmt::Display,
    io::{self, Write},
    path::PathBuf,
};

use chrono::Utc;
use color_eyre::eyre::{Result, eyre};
use serde::Serialize;

use crate::{
    application::usecase::{
        Usecases,
        list::{ListContext, ListOptions, ListRootInput},
    },
    domain::model::{
        path_like::PathLike,
        pretty_path::PrettyPath,
        repo::CanonicalRepo,
        root::{CanonicalRoot, Root},
        template::{Template, TemplateContext},
    },
    presentation::{
        args::GlobalArgs, config::Config, message, model::optional_param::OptionalParam,
    },
    project_dirs::ProjectDirs,
    util::error::FormatErrorChain as _,
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
        usecases: &Usecases,
        project_dirs: &ProjectDirs,
    ) -> Result<()> {
        let format = self.format.validate()?;
        let config = global_args.config(project_dirs)?;
        let input_roots =
            self.roots(&config, project_dirs)?
                .into_iter()
                .map(|root| ListRootInput {
                    allow_missing: !root.is_explicit(),
                    root: root.value().clone(),
                });

        // TODO: make this configurable
        let context = ListContext {
            now: Utc::now(),
            repo_cache_path: PrettyPath::new(global_args.repo_cache_path(project_dirs).value()),
        };
        let options = ListOptions::default();

        let roots = usecases
            .list()
            .list_repos(input_roots, context, options)
            .map(|list_root| {
                list_root.and_then(|root| {
                    Ok((root.root().clone(), root.repos()?.warn_and_skip_errors()))
                })
            })
            .warn_and_skip_errors();
        match format {
            Format::Default => emit_text(roots),
            Format::Json => emit_json(roots),
            Format::Template(template) => emit_template(roots, template),
        }
    }
}

trait WarnAndSkipErrorExt<T, E>: Iterator<Item = Result<T, E>> {
    fn warn_and_skip_errors(self) -> impl Iterator<Item = T>
    where
        Self: Sized,
        E: std::error::Error,
    {
        self.filter_map(|res| {
            res.map_err(|e| message::warn!("{}", e.format_error_chain()))
                .ok()
        })
    }
}

impl<I, T, E> WarnAndSkipErrorExt<T, E> for I where I: Iterator<Item = Result<T, E>> {}

fn emit_json<Roots, Repos>(roots: Roots) -> Result<()>
where
    Roots: Iterator<Item = (CanonicalRoot, Repos)>,
    Repos: Iterator<Item = CanonicalRepo>,
{
    let list = JsonList {
        roots: roots
            .map(|(root, repos)| JsonRoot {
                name: root.name().to_owned(),
                display_path: root.path().as_display_path().to_owned(),
                real_path: root.path().as_real_path().to_owned(),
                canonical_path: root.canonical_path().to_owned(),
                repos: repos.map(JsonRepo::from).collect(),
            })
            .collect(),
    };

    let out = io::stdout();
    let mut out = out.lock();
    serde_json::to_writer(&mut out, &list)?;
    out.flush()?;
    Ok(())
}

fn emit_text<Roots, Repos>(roots: Roots) -> Result<()>
where
    Roots: Iterator<Item = (CanonicalRoot, Repos)>,
    Repos: Iterator<Item = CanonicalRepo>,
{
    for (_root, repos) in roots {
        for repo in repos {
            println!("{}", repo.canonical_path().display());
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Default, Serialize)]
struct RepoListTemplateContext {
    root_name: String,
    // Store paths as already-formatted strings.
    // This avoids serialization failures for non-UTF8 paths when converting
    // template context into JSON-backed string maps.
    root_display_path: String,
    root_real_path: String,
    root_canonical_path: String,
    repo_relative_path: String,
    repo_display_path: String,
    repo_real_path: String,
    repo_canonical_path: String,
}

impl TemplateContext for RepoListTemplateContext {}

impl RepoListTemplateContext {
    fn new(root: &CanonicalRoot, repo: &CanonicalRepo) -> Self {
        Self {
            root_name: root.name().to_owned(),
            root_display_path: format_displayable_path(root.path().as_display_path().display()),
            root_real_path: format_displayable_path(root.path().as_real_path().display()),
            root_canonical_path: format_displayable_path(root.canonical_path().display()),
            repo_relative_path: format_displayable_path(
                repo.relative_path().as_real_path().display(),
            ),
            repo_display_path: format_displayable_path(repo.path().as_display_path().display()),
            repo_real_path: format_displayable_path(repo.path().as_real_path().display()),
            repo_canonical_path: format_displayable_path(repo.canonical_path().display()),
        }
    }
}

fn format_displayable_path(path: impl Display) -> String {
    path.to_string()
}

fn emit_template<Roots, Repos>(roots: Roots, template: Template) -> Result<()>
where
    Roots: Iterator<Item = (CanonicalRoot, Repos)>,
    Repos: Iterator<Item = CanonicalRepo>,
{
    for (root, repos) in roots {
        for repo in repos {
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
