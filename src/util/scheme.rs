use std::{
    borrow::Borrow,
    fmt::{self, Display},
    str::FromStr,
};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub(crate) struct Scheme(String);

impl Borrow<str> for Scheme {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Error)]
pub(crate) enum SchemeParseError {
    #[error("invalid URL scheme `{scheme}`")]
    InvalidScheme { scheme: String },
}

impl FromStr for Scheme {
    type Err = SchemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static SCHEME_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]+$").unwrap());
        if !SCHEME_RE.is_match(s) {
            return Err(SchemeParseError::InvalidScheme {
                scheme: s.to_string(),
            });
        }
        Ok(Self(s.to_owned()))
    }
}

impl TryFrom<String> for Scheme {
    type Error = SchemeParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(s.as_str())
    }
}

impl TryFrom<&str> for Scheme {
    type Error = SchemeParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(Scheme::from_str("foo123").is_ok());
        assert!(Scheme::from_str("fo").is_ok());
        assert!(Scheme::from_str("fo+.-").is_ok());
        assert!(Scheme::from_str("foo_bar").is_err());
        assert!(Scheme::from_str("23foo").is_err());
    }
}
