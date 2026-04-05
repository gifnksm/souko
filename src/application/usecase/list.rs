use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, TimeDelta, Utc};
use color_eyre::eyre::eyre;

use crate::{
    domain::{
        model::{
            path_like::PathLike,
            pretty_path::PrettyPath,
            repo::CanonicalRepo,
            root::{CanonicalRoot, Root},
        },
        port::{
            Ports,
            path_canonicalizer::{PathCanonicalizer, PathCanonicalizerError},
            repo_cache::RepoCache,
        },
        service::repo_scan::{OwnedEntry, RepoScanService, Repos},
    },
    util::error::FormatErrorChain as _,
};

#[derive(Debug, Clone)]
pub(crate) struct ListOptions {
    pub(crate) skip_hidden: bool,
    pub(crate) skip_bare: bool,
    pub(crate) no_recursive: bool,
    pub(crate) cache_expire_duration: TimeDelta,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            skip_hidden: true,
            skip_bare: true,
            no_recursive: true,
            cache_expire_duration: Duration::try_days(3).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ListContext {
    pub(crate) now: DateTime<Utc>,
    pub(crate) repo_cache_path: PrettyPath,
}

#[derive(Debug)]
pub(crate) struct ListRootInput {
    pub(crate) allow_missing: bool,
    pub(crate) root: Root,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ListUsecaseError {
    #[error("root `{name}` does not exist: {}", path.display())]
    RootNotExist { name: String, path: PrettyPath },
    #[error("failed to get canonical path of root `{name}`")]
    CanonicalizeRoot {
        name: String,
        #[source]
        source: PathCanonicalizerError,
    },
    #[error(transparent)]
    Backend(#[from] Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub(crate) struct ListUsecase {
    path_canonicalizer: Arc<dyn PathCanonicalizer>,
    repo_cache: Arc<Mutex<dyn RepoCache>>,
    repo_scan_service: RepoScanService,
}

impl ListUsecase {
    pub(crate) fn new(ports: &Ports) -> Self {
        Self {
            path_canonicalizer: Arc::clone(&ports.path_canonicalizer),
            repo_cache: Arc::clone(&ports.repo_cache),
            repo_scan_service: RepoScanService::new(ports),
        }
    }

    pub(crate) fn list_repos<I>(
        &self,
        roots: I,
        context: ListContext,
        options: ListOptions,
    ) -> ListRoots<'_, I::IntoIter>
    where
        I: IntoIterator<Item = ListRootInput>,
    {
        self.load_repo_cache(&context, &options);
        ListRoots {
            usecase: self,
            path_canonicalizer: Arc::clone(&self.path_canonicalizer),
            repo_cache: Arc::clone(&self.repo_cache),
            repo_scan_service: self.repo_scan_service.clone(),
            context,
            options,
            roots: roots.into_iter(),
        }
    }

    fn load_repo_cache(&self, context: &ListContext, options: &ListOptions) {
        let mut repo_cache = self.repo_cache.lock().unwrap();
        if let Err(err) = repo_cache.load(
            &context.repo_cache_path,
            context.now,
            options.cache_expire_duration,
        ) {
            let err = eyre!(err).wrap_err(format!(
                "failed to load repo cache ({})",
                context.repo_cache_path.display()
            ));
            tracing::warn!("{}", err.format_error_chain());
            repo_cache.clear(context.now);
        }
    }

    fn store_repo_cache(&self, context: &ListContext) {
        let mut repo_cache = self.repo_cache.lock().unwrap();
        if let Err(err) = repo_cache.store(&context.repo_cache_path) {
            let err = eyre!(err).wrap_err(format!(
                "failed to store repo cache ({})",
                context.repo_cache_path.display()
            ));
            tracing::warn!("{}", err.format_error_chain());
        }
    }
}

#[derive(Debug)]
pub(crate) struct ListRoots<'a, I> {
    usecase: &'a ListUsecase,
    path_canonicalizer: Arc<dyn PathCanonicalizer>,
    repo_cache: Arc<Mutex<dyn RepoCache>>,
    repo_scan_service: RepoScanService,
    context: ListContext,
    options: ListOptions,
    roots: I,
}

impl<I> Drop for ListRoots<'_, I> {
    fn drop(&mut self) {
        self.usecase.store_repo_cache(&self.context);
    }
}

impl<I> ListRoots<'_, I> {
    fn canonicalize_root(&self, root: &Root) -> Result<CanonicalRoot, ListUsecaseError> {
        let canonical_path = match self.path_canonicalizer.canonicalize(root.path()) {
            Ok(path) => path,
            Err(PathCanonicalizerError::PathNotFound { path }) => {
                return Err(ListUsecaseError::RootNotExist {
                    name: root.name().to_owned(),
                    path,
                });
            }
            Err(source) => bail!(ListUsecaseError::CanonicalizeRoot {
                name: root.name().to_owned(),
                source
            }),
        };
        let canonical_root = CanonicalRoot::new(root.clone(), canonical_path);
        Ok(canonical_root)
    }
}

impl<I> Iterator for ListRoots<'_, I>
where
    I: Iterator<Item = ListRootInput>,
{
    type Item = Result<ListRoot, ListUsecaseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let root = self.roots.next()?;
            return match self.canonicalize_root(&root.root) {
                Ok(root) => Some(Ok(ListRoot {
                    repo_cache: Arc::clone(&self.repo_cache),
                    repo_scan: self.repo_scan_service.clone(),
                    options: self.options.clone(),
                    root,
                })),
                Err(ListUsecaseError::RootNotExist { .. }) if root.allow_missing => continue,
                Err(e) => Some(Err(e)),
            };
        }
    }
}

#[derive(Debug)]
pub(crate) struct ListRoot {
    repo_scan: RepoScanService,
    repo_cache: Arc<Mutex<dyn RepoCache>>,
    options: ListOptions,
    root: CanonicalRoot,
}

impl ListRoot {
    pub(crate) fn root(&self) -> &CanonicalRoot {
        &self.root
    }

    pub(crate) fn repos(&self) -> Result<ListRepos, ListUsecaseError> {
        let mut repos = self.repo_scan.repos(&self.root)?;
        if self.options.skip_hidden {
            repos.filter_entry(|e| !e.is_hidden());
        }
        Ok(ListRepos {
            repo_cache: Arc::clone(&self.repo_cache),
            options: self.options.clone(),
            repos,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ListRepos {
    repo_cache: Arc<Mutex<dyn RepoCache>>,
    options: ListOptions,
    repos: Repos,
}

impl ListRepos {
    fn entry_to_repo(&self, entry: &OwnedEntry) -> Result<Option<CanonicalRepo>, ListUsecaseError> {
        {
            let mut cache_service = self.repo_cache.lock().unwrap();
            if let Some(repo) = cache_service.get(entry.root(), entry.relative_path()) {
                return Ok(Some(repo));
            }
        }

        let repo = entry.to_repo()?;
        if let Some(repo) = &repo {
            let mut cache_service = self.repo_cache.lock().unwrap();
            if let Some(cached_repo) = cache_service.get(entry.root(), entry.relative_path()) {
                return Ok(Some(cached_repo));
            }
            cache_service.insert(entry.root(), repo);
        }
        Ok(repo)
    }
}

impl Iterator for ListRepos {
    type Item = Result<CanonicalRepo, ListUsecaseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = itry!(self.repos.next()?);
            let Some(repo) = itry!(self.entry_to_repo(&entry)) else {
                tracing::trace!("skipping non-git-repository: {}", entry.path().display());
                continue;
            };
            if self.options.no_recursive {
                self.repos.skip_subdir();
            }
            if self.options.skip_bare && repo.bare() {
                tracing::trace!("skipping bare repo: {}", repo.path().display());
                continue;
            }
            return Some(Ok(repo));
        }
    }
}
