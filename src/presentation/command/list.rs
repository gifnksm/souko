use chrono::Utc;
use color_eyre::eyre::Result;

use crate::{
    application::usecase::list::{ListContext as ListUsecaseContext, ListOptions, ListRootInput},
    domain::model::pretty_path::PrettyPath,
    presentation::{
        context::{global::GlobalContext, list::ListContext},
        message, render,
    },
    util::error::FormatErrorChain as _,
};

pub(in crate::presentation) fn dispatch(
    global_ctx: &GlobalContext,
    list_ctx: &ListContext,
) -> Result<()> {
    let format = list_ctx.format();
    let input_roots = list_ctx.roots().iter().map(|root| ListRootInput {
        // Only the synthesized fallback default root is allowed to be missing
        // silently. Roots present in config remain explicit even when their path
        // is omitted and resolved to the default path.
        allow_missing: !root.is_explicit(),
        root: root.value().root().clone(),
    });

    // TODO: make this configurable
    let context = ListUsecaseContext {
        now: Utc::now(),
        repo_cache_path: PrettyPath::new(global_ctx.repo_cache_path()),
    };
    let options = ListOptions::default();

    let roots = global_ctx
        .usecases()
        .list()
        .list_repos(input_roots, context, options)
        .map(|list_root| {
            list_root
                .and_then(|root| Ok((root.root().clone(), root.repos()?.warn_and_skip_errors())))
        })
        .warn_and_skip_errors();

    render::list::render(std::io::stdout().lock(), format, roots)
}

trait WarnAndSkipErrorExt<T, E>: Iterator<Item = Result<T, E>> {
    fn warn_and_skip_errors(self) -> impl Iterator<Item = T>
    where
        Self: Sized,
        E: std::error::Error,
    {
        self.filter_map(|res| {
            res.map_err(|e| message::warn!("{}", e.format_error_chain()))
                .ok()
        })
    }
}

impl<I, T, E> WarnAndSkipErrorExt<T, E> for I where I: Iterator<Item = Result<T, E>> {}
