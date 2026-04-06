use crate::{
    application::usecase::{clone::CloneUsecase, list::ListUsecase},
    domain::port::Ports,
};

pub(crate) mod clone;
pub(crate) mod list;

#[derive(Debug)]
pub(crate) struct Usecases {
    clone: CloneUsecase,
    list: ListUsecase,
}

impl Usecases {
    pub(crate) fn new(ports: &Ports) -> Self {
        Self {
            clone: CloneUsecase::new(ports),
            list: ListUsecase::new(ports),
        }
    }

    pub(crate) fn clone(&self) -> &CloneUsecase {
        &self.clone
    }

    pub(crate) fn list(&self) -> &ListUsecase {
        &self.list
    }
}
