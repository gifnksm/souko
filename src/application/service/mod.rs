use crate::domain::port::Ports;

mod helper;
pub(crate) mod root;

pub(crate) struct Service {
    root: root::RootService,
}

impl Service {
    pub(crate) fn new(ports: &Ports) -> Self {
        Self {
            root: root::RootService::new(ports),
        }
    }

    pub(crate) fn root(&self) -> &root::RootService {
        &self.root
    }
}
