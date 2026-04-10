use std::{fmt::Debug, path::Path};

use crate::domain::model::{path_buf_pair::PathBufPair, root::CanonicalRoot};

pub(crate) trait DirWalker: Debug {
    fn entries(
        &self,
        root: &CanonicalRoot,
    ) -> Result<Box<dyn DirEntries>, Box<dyn std::error::Error>>;
}

pub(crate) trait DirEntries:
    Debug + Iterator<Item = Result<Box<dyn DirEntry>, Box<dyn std::error::Error>>>
{
    fn skip_subdir(&mut self);
    fn filter_entry(&mut self, filter: FilterPredicate);
}

pub(crate) type FilterPredicate = Box<dyn FnMut(&dyn DirEntry) -> bool>;

pub(crate) trait DirEntry: Debug {
    fn root(&self) -> &CanonicalRoot;
    fn relative_path(&self) -> &Path;
    fn path(&self) -> &PathBufPair;
    fn is_hidden(&self) -> bool;
}

impl DirEntry for &dyn DirEntry {
    fn root(&self) -> &CanonicalRoot {
        DirEntry::root(*self)
    }

    fn relative_path(&self) -> &Path {
        DirEntry::relative_path(*self)
    }

    fn path(&self) -> &PathBufPair {
        DirEntry::path(*self)
    }

    fn is_hidden(&self) -> bool {
        DirEntry::is_hidden(*self)
    }
}

impl DirEntry for Box<dyn DirEntry> {
    fn root(&self) -> &CanonicalRoot {
        self.as_ref().root()
    }

    fn relative_path(&self) -> &Path {
        self.as_ref().relative_path()
    }

    fn path(&self) -> &PathBufPair {
        self.as_ref().path()
    }

    fn is_hidden(&self) -> bool {
        self.as_ref().is_hidden()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn DirWalker> = None;
    const _: Option<&dyn DirEntries> = None;
    const _: Option<&dyn DirEntry> = None;
}
