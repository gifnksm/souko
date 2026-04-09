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
        // Only the synthesized fallback `default` root with `ImplicitDefault`
        // source is allowed to be missing silently. Roots loaded from the
        // configuration file keep `ConfigurationFile` source even when their
        // path is omitted and resolved to the default path.
        allow_missing_root: root.source().is_implicit_default(),
        visit_hidden_dirs: root.value().visit_hidden_dirs(),
        visit_repo_subdirs: root.value().visit_repo_subdirs(),
        include_bare_repo: root.value().include_bare_repo(),
        root: root.value().root().clone(),
    });

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
