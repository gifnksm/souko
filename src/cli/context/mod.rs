use color_eyre::eyre::Result;

use crate::cli::{
    args::Subcommand,
    context::{clone::CloneContext, global::GlobalContext, list::ListContext},
};

pub(in crate::cli) mod clone;
pub(crate) mod global;
pub(in crate::cli) mod list;
pub(in crate::cli) mod query;
pub(in crate::cli) mod root;

#[derive(Debug)]
pub(in crate::cli) enum SubcommandContext {
    Clone(Box<CloneContext>),
    List(Box<ListContext>),
}

impl SubcommandContext {
    pub(in crate::cli) fn new(global_ctx: &GlobalContext, subcommand: &Subcommand) -> Result<Self> {
        match subcommand {
            Subcommand::Clone(args) => {
                Ok(Self::Clone(Box::new(CloneContext::new(global_ctx, args)?)))
            }
            Subcommand::List(args) => Ok(Self::List(Box::new(ListContext::new(global_ctx, args)?))),
        }
    }
}
