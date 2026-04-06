use color_eyre::eyre::{Result, WrapErr as _};

use crate::{
    domain::model::{query::Query, root::Root},
    presentation::{
        args::clone::CloneArgs, context::global::GlobalContext,
        model::optional_param::OptionalParam,
    },
};

#[derive(Debug)]
pub(in crate::presentation) struct CloneContext {
    root: OptionalParam<Root>,
    query: Query,
}

impl CloneContext {
    pub(in crate::presentation) fn new(
        global_ctx: &GlobalContext,
        args: &CloneArgs,
    ) -> Result<Self> {
        let root = match args.root_name() {
            Some(name) => global_ctx.root_by_name(name)?,
            None => global_ctx.default_root(),
        };

        let query_parse_option = global_ctx.config().query_parse_option();
        let query_str = args.query();
        let query = Query::parse(query_str, &query_parse_option)
            .wrap_err_with(|| format!("invalid query: {query_str}"))?;

        Ok(Self { root, query })
    }

    pub(in crate::presentation) fn root(&self) -> &OptionalParam<Root> {
        &self.root
    }

    pub(in crate::presentation) fn query(&self) -> &Query {
        &self.query
    }
}
