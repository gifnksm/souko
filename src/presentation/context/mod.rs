use color_eyre::eyre::Result;

use crate::presentation::{
    args::Subcommand,
    context::{clone::CloneContext, global::GlobalContext, list::ListContext},
};

pub(in crate::presentation) mod clone;
pub(crate) mod global;
pub(in crate::presentation) mod list;

#[derive(Debug)]
pub(in crate::presentation) enum SubcommandContext {
    Clone(CloneContext),
    List(ListContext),
    Help,
}

impl SubcommandContext {
    pub(in crate::presentation) fn new(
        global_ctx: &GlobalContext,
        subcommand: Option<&Subcommand>,
    ) -> Result<Self> {
        match subcommand {
            Some(Subcommand::Clone(args)) => Ok(Self::Clone(CloneContext::new(global_ctx, args)?)),
            Some(Subcommand::List(args)) => Ok(Self::List(ListContext::new(global_ctx, args)?)),
            None => Ok(Self::Help),
        }
    }
}
