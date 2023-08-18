use std::fmt::Debug;

use crate::domain::model::{pretty_path::PrettyPath, repo::CanonicalRepo, root::CanonicalRoot};

pub(crate) trait WalkRepo: Debug {
    fn walk_repo(&self, root: &CanonicalRoot)
        -> Result<Box<dyn Repos>, Box<dyn std::error::Error>>;
}

pub(crate) trait Repos:
    Debug + Iterator<Item = Result<Box<dyn Entry>, Box<dyn std::error::Error>>>
{
    fn skip_subdir(&mut self);
    fn filter_entry(&mut self, filter: FilterPredicate);
}

pub(crate) type FilterPredicate = Box<dyn FnMut(&dyn Entry) -> bool>;

pub(crate) trait Entry: Debug {
    fn path(&self) -> &PrettyPath;
    fn is_hidden(&self) -> bool;
    fn to_repo(&self) -> Result<Option<CanonicalRepo>, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn WalkRepo> = None;
    const _: Option<&dyn Repos> = None;
    const _: Option<&dyn Entry> = None;
}
