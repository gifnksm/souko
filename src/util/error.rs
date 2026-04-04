use std::fmt::{self, Display, Formatter};

pub(crate) fn format_error_chain<'a, E>(error: &'a E) -> ErrorChainDisplay<'a>
where
    E: AsRef<dyn std::error::Error> + 'a,
{
    ErrorChainDisplay {
        error: error.as_ref(),
    }
}

pub(crate) struct ErrorChainDisplay<'a> {
    error: &'a dyn std::error::Error,
}

impl Display for ErrorChainDisplay<'_> {
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
