use std::fmt::Debug;

use crate::domain::model::root::{Root, RootSpec};

pub(crate) trait ResolveRoot: Debug {
    fn resolve_root(
        &self,
        spec: &RootSpec,
        should_exist: bool,
    ) -> Result<Option<Root>, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn ResolveRoot> = None;
}
