use std::fmt::Debug;

use url::Url;

use crate::domain::model::path_like::PathLike;

pub(crate) trait CloneRepo: Debug {
    fn clone_repo(
        &self,
        url: &Url,
        path: &dyn PathLike,
        bare: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn CloneRepo> = None;
}
