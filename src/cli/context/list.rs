use color_eyre::eyre::Result;

use crate::cli::{
    args::list::{Format, ListArgs},
    context::{global::GlobalContext, root::RootContext},
    input::app_param::AppParam,
};

#[derive(Debug)]
pub(in crate::cli) struct ListContext {
    roots: Vec<AppParam<RootContext>>,
    format: Format,
}

impl ListContext {
    pub(in crate::cli) fn new(global_ctx: &GlobalContext, args: &ListArgs) -> Result<Self> {
        let root_map = global_ctx.root_map();
        let roots = match args.root_name() {
            Some(names) => names
                .iter()
                .map(|name| root_map.root_by_name_or_err(name).cloned())
                .collect::<Result<Vec<_>>>()?,
            None => root_map.all_roots().cloned().collect(),
        };
        let format = args.format()?;
        Ok(Self { roots, format })
    }

    pub(in crate::cli) fn roots(&self) -> &[AppParam<RootContext>] {
        &self.roots
    }

    pub(in crate::cli) fn format(&self) -> &Format {
        &self.format
    }
}
