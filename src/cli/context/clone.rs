use color_eyre::eyre::{Result, WrapErr as _};

use crate::{
    cli::{
        args::clone::CloneArgs,
        context::{global::GlobalContext, root::RootContext},
        input::app_param::AppParam,
    },
    domain::model::query::Query,
};

#[derive(Debug)]
pub(in crate::cli) struct CloneContext {
    root: AppParam<RootContext>,
    query: Query,
}

impl CloneContext {
    pub(in crate::cli) fn new(global_ctx: &GlobalContext, args: &CloneArgs) -> Result<Self> {
        let root = match args.root_name() {
            Some(name) => global_ctx.root_map().root_by_name_or_err(name)?,
            None => global_ctx.root_map().default_root(),
        }
        .clone();

        let query_parse_option = global_ctx.query().parse_option();
        let query_str = args.query();
        let query = Query::parse(query_str, query_parse_option)
            .wrap_err_with(|| format!("invalid query: {query_str}"))?;

        Ok(Self { root, query })
    }

    pub(in crate::cli) fn root_context(&self) -> &AppParam<RootContext> {
        &self.root
    }

    pub(in crate::cli) fn query(&self) -> &Query {
        &self.query
    }
}
