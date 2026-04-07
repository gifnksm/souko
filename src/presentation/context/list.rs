use color_eyre::eyre::Result;

use crate::presentation::{
    args::list::{Format, ListArgs},
    context::{global::GlobalContext, root::RootContext},
    model::optional_param::OptionalParam,
    render::list::RepoListTemplateContext,
};

#[derive(Debug)]
pub(in crate::presentation) struct ListContext {
    roots: Vec<OptionalParam<RootContext>>,
    format: Format,
}

impl ListContext {
    pub(in crate::presentation) fn new(
        global_ctx: &GlobalContext,
        args: &ListArgs,
    ) -> Result<Self> {
        let root_map = global_ctx.root_map();
        let roots = match args.root_name() {
            Some(names) => names
                .iter()
                .map(|name| root_map.root_by_name_or_err(name).cloned())
                .collect::<Result<Vec<_>>>()?,
            None => root_map.all_roots().cloned().collect(),
        };
        let format = args.format::<RepoListTemplateContext>()?;
        Ok(Self { roots, format })
    }

    pub(in crate::presentation) fn roots(&self) -> &[OptionalParam<RootContext>] {
        &self.roots
    }

    pub(in crate::presentation) fn format(&self) -> &Format {
        &self.format
    }
}
