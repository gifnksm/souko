use std::{fmt::Display, io, path::PathBuf};

use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{
    cli::args::list::Format,
    domain::model::{
        path_like::PathLike as _,
        repo::CanonicalRepo,
        root::CanonicalRoot,
        template::{Template, TemplateContext},
    },
};

pub(in crate::cli) fn render<W, Roots, Repos>(
    mut out: W,
    format: &Format,
    roots: Roots,
) -> Result<()>
where
    W: io::Write,
    Roots: Iterator<Item = (CanonicalRoot, Repos)>,
    Repos: Iterator<Item = CanonicalRepo>,
{
    match format {
        Format::Default => render_default(&mut out, roots),
        Format::Json => render_json(&mut out, roots),
        Format::Template(template) => render_template(&mut out, roots, template),
    }
}

fn render_default<W, Roots, Repos>(mut out: W, roots: Roots) -> Result<()>
where
    W: io::Write,
    Roots: Iterator<Item = (CanonicalRoot, Repos)>,
    Repos: Iterator<Item = CanonicalRepo>,
{
    for (_root, repos) in roots {
        for repo in repos {
            writeln!(&mut out, "{}", repo.canonical_path().display())?;
        }
    }
    out.flush()?;
    Ok(())
}

fn render_template<W, Roots, Repos>(mut out: W, roots: Roots, template: &Template) -> Result<()>
where
    W: io::Write,
    Roots: Iterator<Item = (CanonicalRoot, Repos)>,
    Repos: Iterator<Item = CanonicalRepo>,
{
    for (root, repos) in roots {
        for repo in repos {
            let context = RepoListTemplateContext::new(&root, &repo);
            writeln!(&mut out, "{}", template.expand(&context))?;
        }
    }
    out.flush()?;
    Ok(())
}

fn render_json<W, Roots, Repos>(mut out: W, roots: Roots) -> Result<()>
where
    W: io::Write,
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

    serde_json::to_writer(&mut out, &list)?;
    out.flush()?;
    Ok(())
}

#[derive(Debug, Clone, Default, Serialize)]
pub(in crate::cli) struct RepoListTemplateContext {
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

fn format_displayable_path(path: impl Display) -> String {
    path.to_string()
}

impl RepoListTemplateContext {
    pub(in crate::cli) fn new(root: &CanonicalRoot, repo: &CanonicalRepo) -> Self {
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
