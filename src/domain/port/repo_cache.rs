use std::{fmt::Debug, path::Path};

use chrono::{DateTime, Duration, Utc};

use crate::domain::model::{path_like::PathLike, repo::CanonicalRepo, root::CanonicalRoot};

pub(crate) trait RepoCache: Debug {
    /// Loads persisted entries into the in-memory cache.
    fn load(
        &self,
        path: &dyn PathLike,
        now: DateTime<Utc>,
        expire_duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    /// Clears all cached entries.
    fn clear(&self);

    /// Persists initialized entries.
    fn persist(
        &self,
        path: &dyn PathLike,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    /// Returns a handle to the cache entry for the given root/path pair.
    fn entry(&self, root: &CanonicalRoot, relative_path: &Path) -> Box<dyn RepoCacheEntry>;
}

pub(crate) trait RepoCacheEntry: Debug {
    /// Returns the cached repository if present.
    fn get(&self) -> Option<CanonicalRepo>;

    /// Publishes a repository value for this entry.
    fn publish(&self, repo: CanonicalRepo);
}
