use color_eyre::eyre::Result;

use crate::{
    domain::model::root::Root,
    presentation::{
        args::list::{Format, ListArgs},
        context::global::GlobalContext,
        model::optional_param::OptionalParam,
        render::list::RepoListTemplateContext,
    },
};

#[derive(Debug)]
pub(in crate::presentation) struct ListContext {
    roots: Vec<OptionalParam<Root>>,
    format: Format,
}

impl ListContext {
    pub(in crate::presentation) fn new(
        global_ctx: &GlobalContext,
        args: &ListArgs,
    ) -> Result<Self> {
        let roots = match args.root_name() {
            Some(names) => global_ctx.roots_by_names(names)?,
            None => global_ctx.all_roots(),
        };
        let format = args.format::<RepoListTemplateContext>()?;
        Ok(Self { roots, format })
    }

    pub(in crate::presentation) fn roots(&self) -> &[OptionalParam<Root>] {
        &self.roots
    }

    pub(in crate::presentation) fn format(&self) -> &Format {
        &self.format
    }
}
