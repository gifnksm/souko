use std::fmt::{self, Display, Formatter};

pub(crate) trait FormatErrorChain {
    fn format_error_chain(&self) -> impl Display + '_;
}

impl<E> FormatErrorChain for E
where
    E: std::error::Error + ?Sized,
{
    fn format_error_chain(&self) -> impl Display + '_ {
        ErrorChainDisplay { error: self }
    }
}

struct ErrorChainDisplay<'a, E>
where
    E: ?Sized,
{
    error: &'a E,
}

impl<E> Display for ErrorChainDisplay<'_, E>
where
    E: std::error::Error + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;

        let mut source = self.error.source();
        while let Some(err) = source {
            write!(f, "\n  caused by: {err}")?;
            source = err.source();
        }

        Ok(())
    }
}
