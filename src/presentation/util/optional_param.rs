#[derive(Debug, Clone)]
enum OptionalParamValue<T> {
    /// Explicitly specified value (via command line argument, environment variable, configuration file, etc.)
    Explicit(T),
    /// Automatically determined value
    Default(T),
}

impl<T> OptionalParamValue<T> {
    fn is_default(&self) -> bool {
        matches!(self, Self::Default(_))
    }

    fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit(_))
    }

    fn as_ref(&self) -> &T {
        match self {
            Self::Explicit(v) => v,
            Self::Default(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OptionalParam<T> {
    name: &'static str,
    value: OptionalParamValue<T>,
}

impl<T> OptionalParam<T> {
    pub(crate) fn new(name: &'static str, value: Option<T>, default: impl FnOnce() -> T) -> Self {
        let value = match value {
            Some(path) => OptionalParamValue::Explicit(path),
            None => OptionalParamValue::Default(default()),
        };
        OptionalParam { name, value }
    }

    pub(crate) fn new_default(name: &'static str, value: T) -> Self {
        let value = OptionalParamValue::Default(value);
        OptionalParam { name, value }
    }

    pub(crate) fn new_explicit(name: &'static str, value: T) -> Self {
        let value = OptionalParamValue::Explicit(value);
        OptionalParam { name, value }
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn value(&self) -> &T {
        self.value.as_ref()
    }

    pub(crate) fn is_default(&self) -> bool {
        self.value.is_default()
    }

    pub(crate) fn is_explicit(&self) -> bool {
        self.value.is_explicit()
    }

    pub(crate) fn map<U>(self, f: impl FnOnce(T) -> U) -> OptionalParam<U> {
        OptionalParam {
            name: self.name,
            value: match self.value {
                OptionalParamValue::Explicit(v) => OptionalParamValue::Explicit(f(v)),
                OptionalParamValue::Default(v) => OptionalParamValue::Default(f(v)),
            },
        }
    }
}
