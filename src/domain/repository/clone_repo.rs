use std::{fmt::Debug, path::Path};

use url::Url;

pub(crate) trait CloneRepo: Debug {
    fn clone_repo(
        &self,
        url: &Url,
        path: &Path,
        bare: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn CloneRepo> = None;
}
