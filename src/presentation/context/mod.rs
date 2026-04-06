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
}

impl SubcommandContext {
    pub(in crate::presentation) fn new(
        global_ctx: &GlobalContext,
        subcommand: &Subcommand,
    ) -> Result<Self> {
        match subcommand {
            Subcommand::Clone(args) => Ok(Self::Clone(CloneContext::new(global_ctx, args)?)),
            Subcommand::List(args) => Ok(Self::List(ListContext::new(global_ctx, args)?)),
        }
    }
}
