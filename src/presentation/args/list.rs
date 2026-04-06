use color_eyre::eyre::Result;

use crate::domain::model::template::{Template, TemplateContext};

#[derive(Debug, Clone, Default, clap::Args)]
pub(in crate::presentation) struct ListArgs {
    /// List repositories only under the specified root (repeatable)
    #[arg(long = "root")]
    root_name: Option<Vec<String>>,

    #[command(flatten)]
    format: FormatArgs,
}

#[derive(Debug, Clone, Default, clap::Args)]
#[group(id = "format", multiple = false)]
pub(in crate::presentation) struct FormatArgs {
    /// Output repositories as JSON
    #[arg(long)]
    json: bool,
    /// Output each repository using a template string
    #[arg(long)]
    template: Option<Template>,
}

impl FormatArgs {
    fn validate<C>(&self) -> Result<Format>
    where
        C: TemplateContext,
    {
        let FormatArgs { json, template } = self;
        if *json {
            Ok(Format::Json)
        } else if let Some(template) = template {
            template.validate::<C>()?;
            Ok(Format::Template(template.clone()))
        } else {
            Ok(Format::Default)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::presentation) enum Format {
    #[default]
    Default,
    Json,
    Template(Template),
}

impl ListArgs {
    pub(in crate::presentation) fn root_name(&self) -> Option<&[String]> {
        self.root_name.as_deref()
    }

    pub(in crate::presentation) fn format<C>(&self) -> Result<Format>
    where
        C: TemplateContext,
    {
        self.format.validate::<C>()
    }
}
