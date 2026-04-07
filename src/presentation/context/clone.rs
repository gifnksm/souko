use color_eyre::eyre::{Result, WrapErr as _, eyre};

use crate::{
    domain::model::query::Query,
    presentation::{
        args::clone::CloneArgs,
        context::{global::GlobalContext, root::RootContext},
        model::optional_param::OptionalParam,
    },
};

#[derive(Debug)]
pub(in crate::presentation) struct CloneContext {
    root: OptionalParam<RootContext>,
    query: Query,
}

impl CloneContext {
    pub(in crate::presentation) fn new(
        global_ctx: &GlobalContext,
        args: &CloneArgs,
    ) -> Result<Self> {
        let root = match args.root_name() {
            Some(name) => global_ctx
                .root_map()
                .root_by_name(name)
                .ok_or_else(|| eyre!("root `{name}` not found in config file"))?,
            None => global_ctx.root_map().default_root(),
        }
        .clone();

        let query_parse_option = global_ctx.query().parse_option();
        let query_str = args.query();
        let query = Query::parse(query_str, query_parse_option)
            .wrap_err_with(|| format!("invalid query: {query_str}"))?;

        Ok(Self { root, query })
    }

    pub(in crate::presentation) fn root_context(&self) -> &OptionalParam<RootContext> {
        &self.root
    }

    pub(in crate::presentation) fn query(&self) -> &Query {
        &self.query
    }
}
