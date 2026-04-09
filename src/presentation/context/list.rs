use color_eyre::eyre::Result;

use crate::presentation::{
    args::list::{Format, ListArgs},
    context::{global::GlobalContext, root::RootContext},
    model::app_param::AppParam,
    render::list::RepoListTemplateContext,
};

#[derive(Debug)]
pub(in crate::presentation) struct ListContext {
    roots: Vec<AppParam<RootContext>>,
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

    pub(in crate::presentation) fn roots(&self) -> &[AppParam<RootContext>] {
        &self.roots
    }

    pub(in crate::presentation) fn format(&self) -> &Format {
        &self.format
    }
}
