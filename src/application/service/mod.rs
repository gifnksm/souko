use crate::domain::repository::Repository;

pub(crate) mod root;

pub(crate) struct Service {
    root: root::RootService,
}

impl Service {
    pub(crate) fn new(repository: &Repository) -> Self {
        Self {
            root: root::RootService::new(repository),
        }
    }

    pub(crate) fn root(&self) -> &root::RootService {
        &self.root
    }
}
