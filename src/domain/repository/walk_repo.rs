use std::{fmt::Debug, path::Path};

use crate::domain::model::repo::Repo;

pub(crate) trait WalkRepo:
    Debug + Iterator<Item = Result<Box<dyn Entry>, Box<dyn std::error::Error>>>
{
    fn skip_subdir(&mut self);
    fn filter_entry(&mut self, filter: FilterPredicate);
}

pub(crate) type FilterPredicate = Box<dyn FnMut(&dyn Entry) -> bool>;

pub(crate) trait Entry: Debug {
    fn absolute_path(&self) -> &Path;
    fn is_hidden(&self) -> bool;
    fn to_repo(&self) -> Result<Option<Repo>, Box<dyn std::error::Error>>;
}
