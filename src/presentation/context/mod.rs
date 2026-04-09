use color_eyre::eyre::Result;

use crate::presentation::{
    args::Subcommand,
    context::{clone::CloneContext, global::GlobalContext, list::ListContext},
};

pub(in crate::presentation) mod clone;
pub(crate) mod global;
pub(in crate::presentation) mod list;
pub(in crate::presentation) mod query;
pub(in crate::presentation) mod root;

#[derive(Debug)]
pub(in crate::presentation) enum SubcommandContext {
    Clone(Box<CloneContext>),
    List(Box<ListContext>),
}

impl SubcommandContext {
    pub(in crate::presentation) fn new(
        global_ctx: &GlobalContext,
        subcommand: &Subcommand,
    ) -> Result<Self> {
        match subcommand {
            Subcommand::Clone(args) => {
                Ok(Self::Clone(Box::new(CloneContext::new(global_ctx, args)?)))
            }
            Subcommand::List(args) => Ok(Self::List(Box::new(ListContext::new(global_ctx, args)?))),
        }
    }
}
