use std::fmt::Debug;

use crate::domain::model::root::{CanonicalRoot, Root};

pub(crate) trait CanonicalizeRoot: Debug {
    fn canonicalize_root(
        &self,
        root: &Root,
        should_exist: bool,
    ) -> Result<Option<CanonicalRoot>, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn CanonicalizeRoot> = None;
}
