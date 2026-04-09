/// Source of an application input parameter.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant)]
pub(in crate::presentation) enum AppParamSource {
    /// Value provided through a clap argument source.
    ///
    /// This includes values supplied either by the command-line option itself
    /// or by the environment variable bound to that option via clap `env`.
    CommandLineArgument,
    /// Value loaded from a configuration file.
    ConfigurationFile,
    /// Value synthesized by the application when the user did not provide one.
    ImplicitDefault,
}

#[derive(Debug, Clone)]
pub(in crate::presentation) struct AppParam<T> {
    source: AppParamSource,
    value: T,
}

impl<T> AppParam<T> {
    pub(in crate::presentation) fn new(source: AppParamSource, value: T) -> Self {
        AppParam { source, value }
    }

    pub(in crate::presentation) fn source(&self) -> &AppParamSource {
        &self.source
    }

    pub(in crate::presentation) fn value(&self) -> &T {
        &self.value
    }

    pub(in crate::presentation) fn map<F, U>(self, f: F) -> AppParam<U>
    where
        F: FnOnce(T) -> U,
    {
        AppParam {
            source: self.source,
            value: f(self.value),
        }
    }
}
