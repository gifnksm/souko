use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::domain::{
    model::{path_like::PathLike, pretty_path::PrettyPath, root::CanonicalRoot},
    port::dir_walker::{DirEntries, DirEntry, DirWalker, FilterPredicate},
};

#[derive(Debug)]
pub(in crate::infrastructure) struct FsDirWalker {}

impl FsDirWalker {
    pub(in crate::infrastructure) fn new() -> Self {
        Self {}
    }
}

impl DirWalker for FsDirWalker {
    fn entries(
        &self,
        root: &CanonicalRoot,
    ) -> Result<Box<dyn DirEntries>, Box<dyn std::error::Error>> {
        Ok(Box::new(FsDirEntries::new(root)))
    }
}

#[derive(custom_debug_derive::Debug)]
pub(super) struct FsDirEntries {
    root: Arc<CanonicalRoot>,
    iter: walkdir::IntoIter,
    #[debug(skip)]
    filter: Option<FilterPredicate>,
}

impl FsDirEntries {
    pub(super) fn new(root: &CanonicalRoot) -> Self {
        let root = Arc::new(root.clone());
        let iter = walkdir::WalkDir::new(root.path().as_real_path())
            .sort_by_file_name()
            .into_iter();
        Self {
            root,
            iter,
            filter: None,
        }
    }
}

impl DirEntries for FsDirEntries {
    fn skip_subdir(&mut self) {
        self.iter.skip_current_dir();
    }

    fn filter_entry(&mut self, filter: FilterPredicate) {
        let mut filter = filter;
        let filter = match self.filter.take() {
            Some(mut pre_filter) => {
                Box::new(move |entry: &dyn DirEntry| pre_filter(entry) && filter(entry))
            }
            None => filter,
        };

        self.filter = Some(filter);
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
}

impl Iterator for FsDirEntries {
    type Item = Result<Box<dyn DirEntry>, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = itry!(self.iter.next()?.map_err(Error::WalkDir));

            if !entry.file_type().is_dir() {
                tracing::trace!("skipping non-directory: {}", entry.path().display());
                continue;
            }

            let entry = FsDirEntry::new(Arc::clone(&self.root), entry);

            if let Some(filter) = &mut self.filter
                && !filter(&entry)
            {
                tracing::trace!("skipping filtered entry: {}", entry.path().display());
                self.iter.skip_current_dir();
                continue;
            }

            return Some(Ok(Box::new(entry)));
        }
    }
}

#[derive(Debug)]
struct FsDirEntry {
    root: Arc<CanonicalRoot>,
    relative_path: PathBuf,
    path: PrettyPath,
}

impl FsDirEntry {
    fn new(root: Arc<CanonicalRoot>, entry: walkdir::DirEntry) -> Self {
        let relative_path = entry
            .path()
            .strip_prefix(root.path().as_real_path())
            .unwrap() // never panic because the path starts with the root path
            .to_owned();
        let path = root.path().join(&relative_path);

        Self {
            root,
            relative_path,
            path,
        }
    }
}

impl DirEntry for FsDirEntry {
    fn root(&self) -> &CanonicalRoot {
        &self.root
    }

    fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    fn path(&self) -> &PrettyPath {
        &self.path
    }

    fn is_hidden(&self) -> bool {
        self.path()
            .as_real_path()
            .file_name()
            .and_then(|file_name| file_name.to_str().map(|s| s.starts_with('.')))
            .unwrap_or(false)
    }
}
