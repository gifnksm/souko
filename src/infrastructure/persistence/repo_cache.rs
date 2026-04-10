use std::{
    collections::{HashMap, hash_map::Entry},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
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
        port::repo_cache::{RepoCache, RepoCacheEntry},
    },
    util::file,
};

#[derive(Debug)]
pub(in crate::infrastructure) struct JsonRepoCache(Mutex<JsonRepoCacheInner>);

impl JsonRepoCache {
    pub(in crate::infrastructure) fn new() -> Self {
        Self(Mutex::new(JsonRepoCacheInner {
            now: None,
            expire_duration: None,
            cache: JsonCache::default(),
        }))
    }
}

#[derive(Debug)]
struct JsonRepoCacheInner {
    now: Option<DateTime<Utc>>,
    expire_duration: Option<Duration>,
    cache: JsonCache,
}

impl RepoCache for JsonRepoCache {
    fn load(
        &self,
        path: &dyn PathLike,
        now: DateTime<Utc>,
        expire_duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut this = self.0.lock().unwrap();
        this.now = Some(now);
        this.expire_duration = Some(expire_duration);
        this.cache = file::load_json("repository cache", &path.as_real_path())?.unwrap_or_default();
        this.cache.remove_invalid_repos(&now, expire_duration);
        Ok(())
    }

    fn clear(&self) {
        let mut this = self.0.lock().unwrap();
        this.cache = JsonCache::default();
    }

    fn persist(
        &self,
        path: &dyn PathLike,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut this = self.0.lock().unwrap();
        if let (Some(now), Some(expire_duration)) = (this.now, this.expire_duration) {
            this.cache.remove_invalid_repos(&now, expire_duration);
            file::store_json("repository cache", &path.as_real_path(), &this.cache)?;
        }
        Ok(())
    }

    fn entry(&self, root: &CanonicalRoot, relative_path: &Path) -> Box<dyn RepoCacheEntry> {
        let mut this = self.0.lock().unwrap();
        let root_cache = this.cache.entry(root);
        let entry = root_cache.entry(relative_path.to_owned());
        Box::new(RepoCacheEntryHandler {
            now: this.now.unwrap_or_else(Utc::now),
            root: root.as_root().clone(),
            relative_path: relative_path.to_owned(),
            entry,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct JsonCache {
    roots: HashMap<String, JsonRootEntry>,
}

impl JsonCache {
    fn remove_invalid_repos(&mut self, now: &DateTime<Utc>, expire_duration: Duration) {
        for root_cache in self.roots.values_mut() {
            root_cache.remove_invalid_repos(now, expire_duration);
        }
    }

    fn entry(&mut self, root: &CanonicalRoot) -> &mut JsonRootEntry {
        match self.roots.entry(root.name().to_owned()) {
            Entry::Occupied(mut entry) => {
                if !entry.get().is_valid(root) {
                    *entry.get_mut() = JsonRootEntry::new(root);
                }
                entry.into_mut()
            }
            Entry::Vacant(entry) => entry.insert(JsonRootEntry::new(root)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRootEntry {
    real_path: PathBuf,
    display_path: PathBuf,
    canonical_path: PathBuf,
    repos: HashMap<PathBuf, Arc<Mutex<Option<JsonRepoEntry>>>>,
}

impl JsonRootEntry {
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
        self.repos.retain(|_, entry| {
            entry
                .lock()
                .unwrap()
                .as_ref()
                .is_some_and(|entry| entry.is_valid(now, expire_duration))
        });
    }

    fn entry(&mut self, relative_path: PathBuf) -> Arc<Mutex<Option<JsonRepoEntry>>> {
        Arc::clone(
            self.repos
                .entry(relative_path)
                .or_insert_with(|| Arc::new(Mutex::new(None))),
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRepoEntry {
    timestamp: DateTime<Utc>,
    canonical_path: PathBuf,
    bare: bool,
}

impl JsonRepoEntry {
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

#[derive(Debug)]
struct RepoCacheEntryHandler {
    now: DateTime<Utc>,
    root: Root,
    relative_path: PathBuf,
    entry: Arc<Mutex<Option<JsonRepoEntry>>>,
}

impl RepoCacheEntry for RepoCacheEntryHandler {
    fn get(&self) -> Option<CanonicalRepo> {
        let entry = self.entry.lock().unwrap();
        entry
            .as_ref()
            .map(|repo| repo.to_canonical_repo(&self.root, self.relative_path.clone()))
    }

    fn publish(&self, repo: CanonicalRepo) {
        let mut entry = self.entry.lock().unwrap();
        *entry = Some(JsonRepoEntry {
            timestamp: self.now,
            canonical_path: repo.canonical_path().to_owned(),
            bare: repo.bare(),
        });
    }
}
