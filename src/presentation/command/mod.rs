use color_eyre::eyre::Result;

use crate::presentation::{SubcommandContext, context::global::GlobalContext};

mod clone;
mod list;

pub(crate) fn dispatch(
    global_ctx: &GlobalContext,
    subcommand_ctx: &SubcommandContext,
) -> Result<()> {
    match subcommand_ctx {
        SubcommandContext::Clone(clone_ctx) => clone::dispatch(global_ctx, clone_ctx),
        SubcommandContext::List(list_ctx) => list::dispatch(global_ctx, list_ctx),
    }
}
