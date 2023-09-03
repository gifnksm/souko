use std::{fmt::Debug, path::Path};

use chrono::{DateTime, Duration, Utc};

use crate::domain::model::{path_like::PathLike, repo::CanonicalRepo, root::CanonicalRoot};

pub(crate) trait RepoCache: Debug {
    /// Loads the cache at path.
    fn load(
        &mut self,
        path: &dyn PathLike,
        now: DateTime<Utc>,
        expire_duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    /// Clears the cache.
    fn clear(&mut self, now: DateTime<Utc>);

    /// Stores the cache.
    fn store(
        &mut self,
        path: &dyn PathLike,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    /// Gets the cached repository.
    ///
    /// Returns `None` if the repository is not cached.
    fn get(&mut self, root: &CanonicalRoot, relative_path: &Path) -> Option<CanonicalRepo>;

    /// Inserts the repository in the cache.
    ///
    /// Before calling `insert`, `load` or `clear` must be called.
    fn insert(&mut self, root: &CanonicalRoot, repo: &CanonicalRepo);
}
