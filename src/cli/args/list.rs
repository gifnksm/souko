use color_eyre::eyre::Result;

use crate::{cli::render::list::RepoListTemplateContext, domain::model::template::Template};

#[derive(Debug, Clone, Default, clap::Args)]
pub(in crate::cli) struct ListArgs {
    /// List repositories only under the specified root (repeatable)
    #[arg(long = "root")]
    root_name: Option<Vec<String>>,

    #[command(flatten)]
    format: FormatArgs,
}

#[derive(Debug, Clone, Default, clap::Args)]
#[group(id = "format", multiple = false)]
pub(in crate::cli) struct FormatArgs {
    /// Output repositories as JSON
    #[arg(long)]
    json: bool,
    /// Output each repository using a template string
    #[arg(long)]
    template: Option<Template<RepoListTemplateContext>>,
}

impl FormatArgs {
    fn validate(&self) -> Result<Format> {
        let FormatArgs { json, template } = self;
        if *json {
            Ok(Format::Json)
        } else if let Some(template) = template {
            Ok(Format::Template(template.clone()))
        } else {
            Ok(Format::Default)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::cli) enum Format {
    #[default]
    Default,
    Json,
    Template(Template<RepoListTemplateContext>),
}

impl ListArgs {
    pub(in crate::cli) fn root_name(&self) -> Option<&[String]> {
        self.root_name.as_deref()
    }

    pub(in crate::cli) fn format(&self) -> Result<Format> {
        self.format.validate()
    }
}
