use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        model::{
            path_like::PathLike,
            repo::{CanonicalRepo, Repo},
            root::{CanonicalRoot, Root},
        },
        repository::repo_cache::RepoCache,
    },
    util::file,
};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct FsRepoCache {
    now: Option<DateTime<Utc>>,
    cache: Cache,
}

impl FsRepoCache {
    pub(super) fn new() -> Self {
        Self {
            now: None,
            cache: Cache::default(),
        }
    }
}

impl RepoCache for FsRepoCache {
    fn load(
        &mut self,
        path: &dyn PathLike,
        now: DateTime<Utc>,
        expire_duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.cache = file::load_json("repository cache", &path.as_real_path())?.unwrap_or_default();
        self.cache.remove_invalid_repos(&now, expire_duration);
        self.now = Some(now);
        Ok(())
    }

    fn clear(&mut self, now: DateTime<Utc>) {
        self.now = Some(now);
        self.cache = Cache::default();
    }

    fn store(
        &mut self,
        path: &dyn PathLike,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        file::store_json("repository cache", &path.as_real_path(), &self.cache)?;
        Ok(())
    }

    fn get(&mut self, root: &CanonicalRoot, relative_path: &Path) -> Option<CanonicalRepo> {
        let root_cache = self.cache.get_root_cache(root)?;
        let repo_cache = root_cache.get_repo_cache(relative_path)?;
        Some(repo_cache.to_canonical_repo(root.as_root(), relative_path.to_owned()))
    }

    fn insert(&mut self, root: &CanonicalRoot, repo: &CanonicalRepo) {
        let root_cache = self.cache.insert_root_cache(root);
        let now = self
            .now
            .expect("load or clear must be called before insert");
        root_cache.insert_canonical_repo(repo, now);
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct Cache {
    roots: HashMap<String, RootCacheEntry>,
}

impl Cache {
    fn remove_invalid_repos(&mut self, now: &DateTime<Utc>, expire_duration: Duration) {
        for root_cache in self.roots.values_mut() {
            root_cache.remove_invalid_repos(now, expire_duration);
        }
    }

    fn get_root_cache(&mut self, root: &CanonicalRoot) -> Option<&mut RootCacheEntry> {
        let Entry::Occupied(entry) = self.roots.entry(root.name().to_owned()) else {
            return None;
        };

        if !entry.get().is_valid(root) {
            entry.remove();
            return None;
        }

        Some(entry.into_mut())
    }

    fn insert_root_cache(&mut self, root: &CanonicalRoot) -> &mut RootCacheEntry {
        match self.roots.entry(root.name().to_owned()) {
            Entry::Occupied(mut entry) => {
                if !entry.get().is_valid(root) {
                    *entry.get_mut() = RootCacheEntry::new(root);
                }
                entry.into_mut()
            }
            Entry::Vacant(entry) => entry.insert(RootCacheEntry::new(root)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RootCacheEntry {
    real_path: PathBuf,
    display_path: PathBuf,
    canonical_path: PathBuf,
    repos: HashMap<PathBuf, RepoCacheEntry>,
}

impl RootCacheEntry {
    fn new(root: &CanonicalRoot) -> Self {
        Self {
            real_path: root.path().as_real_path().to_owned(),
            display_path: root.path().as_display_path().to_owned(),
            canonical_path: root.canonical_path().to_owned(),
            repos: HashMap::new(),
        }
    }

    fn is_valid(&self, root: &CanonicalRoot) -> bool {
        self.canonical_path == root.canonical_path()
    }

    fn remove_invalid_repos(&mut self, now: &DateTime<Utc>, expire_duration: Duration) {
        self.repos
            .retain(|_, entry| entry.is_valid(now, expire_duration));
    }

    fn get_repo_cache(&mut self, relative_path: &Path) -> Option<&RepoCacheEntry> {
        self.repos.get(relative_path)
    }

    fn insert_canonical_repo(&mut self, repo: &CanonicalRepo, now: DateTime<Utc>) {
        let relative_path = repo.relative_path().as_real_path().to_owned();
        let entry = RepoCacheEntry {
            timestamp: now,
            canonical_path: repo.canonical_path().to_owned(),
            bare: repo.bare(),
        };
        self.repos.insert(relative_path, entry);
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RepoCacheEntry {
    timestamp: DateTime<Utc>,
    canonical_path: PathBuf,
    bare: bool,
}

impl RepoCacheEntry {
    fn is_valid(&self, now: &DateTime<Utc>, expire_duration: Duration) -> bool {
        *now - self.timestamp <= expire_duration
    }

    fn to_repo(&self, root: &Root, relative_path: PathBuf) -> Repo {
        Repo::from_relative_path(root, relative_path, self.bare)
    }

    fn to_canonical_repo(&self, root: &Root, relative_path: PathBuf) -> CanonicalRepo {
        let repo = self.to_repo(root, relative_path);
        CanonicalRepo::new(repo, self.canonical_path.clone())
    }
}
