use color_eyre::eyre::{Result, WrapErr as _, eyre};

use crate::{
    domain::model::path_like::PathLike as _,
    presentation::{
        context::{clone::CloneContext, global::GlobalContext},
        message,
    },
};

pub(super) fn dispatch(global_ctx: &GlobalContext, clone_ctx: &CloneContext) -> Result<()> {
    let root_context = clone_ctx.root_context();
    let query = clone_ctx.query();

    let bare = false;

    message::info!(
        "cloning {} into {}",
        query.original_query(),
        root_context.value().path().display()
    );

    global_ctx
        .usecases()
        .clone()
        .clone_repo(root_context.value().root(), query, bare)
        .map_err(|e| eyre!(e))
        .wrap_err("failed to clone repository")?;

    message::info!("cloned {}", query.original_query());

    Ok(())
}
