use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    str::FromStr,
};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::Template;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "&str")]
pub struct Scheme(String);

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
pub enum SchemeParseError {
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

#[derive(Debug, Clone)]
pub struct QueryConfig {
    pub default_scheme: Option<Scheme>,
    pub scheme_alias: HashMap<Scheme, Scheme>,
    pub custom_scheme: HashMap<Scheme, Template>,
}

#[derive(Debug, Clone)]
pub struct Query {
    original_query: String,
    url: Url,
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("invalid URL {}: {source}", ErrorDisplayHelper { original_query, expanded_query })]
    InvalidUrl {
        original_query: String,
        expanded_query: String,
        source: url::ParseError,
    },
    #[error("no scheme specified {}", ErrorDisplayHelper { original_query, expanded_query })]
    NoSchemeSpecified {
        original_query: String,
        expanded_query: String,
    },
    #[error("invalid config: circular alias {}", ErrorDisplayHelper { original_query, expanded_query })]
    CircularAlias {
        original_query: String,
        expanded_query: String,
    },
}

#[derive(Debug)]
struct ErrorDisplayHelper<'a> {
    original_query: &'a str,
    expanded_query: &'a str,
}

impl Display for ErrorDisplayHelper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.original_query == self.expanded_query {
            write!(f, "`{}`", self.original_query)
        } else {
            write!(
                f,
                "`{}` (expanded to `{}`)",
                self.original_query, self.expanded_query
            )
        }
    }
}

impl Query {
    pub fn parse(query: &str, config: &QueryConfig) -> Result<Self, QueryError> {
        let url_schemes = ["http://", "https://", "ssh://", "git://", "ftp://"];
        let mut visited_scheme = HashSet::new();

        let original_query = query.to_string();
        let mut query = query.to_string();
        loop {
            if url_schemes.iter().any(|scheme| query.starts_with(scheme)) {
                // URL detected, no need to expand
                let url = Url::parse(&query).map_err(|e| QueryError::InvalidUrl {
                    original_query: original_query.clone(),
                    expanded_query: query.clone(),
                    source: e,
                })?;
                return Ok(Self {
                    original_query,
                    url,
                });
            }

            if let Some((scheme, rest)) = query.split_once(':') {
                if visited_scheme.contains(scheme) {
                    return Err(QueryError::CircularAlias {
                        original_query,
                        expanded_query: query,
                    });
                }
                visited_scheme.insert(scheme.to_owned());

                // scheme alias
                if let Some(scheme) = config.scheme_alias.get(scheme) {
                    query = format!("{}:{}", scheme, rest);
                    continue;
                }

                // custom scheme
                if let Some(template) = config.custom_scheme.get(scheme) {
                    query = template.expand(&HashMap::from_iter([("path", rest)]));
                    continue;
                }

                // unknown scheme, assume it's a scp-like syntax
                query = format!("ssh://{}/{}", scheme, rest);
                continue;
            }

            // no scheme, add default scheme
            if let Some(scheme) = &config.default_scheme {
                query = format!("{}:{}", scheme, query);
                continue;
            }

            return Err(QueryError::NoSchemeSpecified {
                original_query,
                expanded_query: query,
            });
        }
    }

    pub fn original_query(&self) -> &str {
        &self.original_query
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scheme() {
        assert!(Scheme::from_str("foo123").is_ok());
        assert!(Scheme::from_str("fo").is_ok());
        assert!(Scheme::from_str("fo+.-").is_ok());
        assert!(Scheme::from_str("foo_bar").is_err());
        assert!(Scheme::from_str("23foo").is_err());
    }

    #[test]
    fn test_parse_with_empty_config() {
        let config = QueryConfig {
            default_scheme: None,
            scheme_alias: HashMap::new(),
            custom_scheme: HashMap::new(),
        };

        let query = Query::parse("ssh://github.com/gifnksm/souko.git", &config).unwrap();
        assert_eq!(query.url.as_str(), "ssh://github.com/gifnksm/souko.git");

        let query = Query::parse("https://github.com/gifnksm/souko.git", &config).unwrap();
        assert_eq!(query.url.as_str(), "https://github.com/gifnksm/souko.git");

        let query = Query::parse("git@github.com:gifnksm/souko.git", &config).unwrap();
        assert_eq!(query.url.as_str(), "ssh://git@github.com/gifnksm/souko.git");

        let err = Query::parse("gifnksm/souko", &config).unwrap_err();
        assert_eq!(err.to_string(), "no scheme specified `gifnksm/souko`");
    }

    #[test]
    fn test_parse_with_default_config() {
        let config = QueryConfig {
            default_scheme: Some("gh".parse().unwrap()),
            scheme_alias: HashMap::from_iter([
                ("gh".parse().unwrap(), "github".parse().unwrap()),
                ("gl".parse().unwrap(), "gitlab".parse().unwrap()),
            ]),
            custom_scheme: HashMap::from_iter([
                (
                    "github".parse().unwrap(),
                    "https://github.com/{path}.git".parse().unwrap(),
                ),
                (
                    "gitlab".parse().unwrap(),
                    "https://gitlab.com/{path}.git".parse().unwrap(),
                ),
            ]),
        };

        let query = Query::parse("ssh://github.com/gifnksm/souko.git", &config).unwrap();
        assert_eq!(query.url.as_str(), "ssh://github.com/gifnksm/souko.git");

        let query = Query::parse("https://github.com/gifnksm/souko.git", &config).unwrap();
        assert_eq!(query.url.as_str(), "https://github.com/gifnksm/souko.git");

        let query = Query::parse("git@github.com:gifnksm/souko.git", &config).unwrap();
        assert_eq!(query.url.as_str(), "ssh://git@github.com/gifnksm/souko.git");

        let query = Query::parse("gh:gifnksm/souko", &config).unwrap();
        assert_eq!(query.url.as_str(), "https://github.com/gifnksm/souko.git");

        let query = Query::parse("gl:gifnksm/souko", &config).unwrap();
        assert_eq!(query.url.as_str(), "https://gitlab.com/gifnksm/souko.git");

        let query = Query::parse("gifnksm/souko", &config).unwrap();
        assert_eq!(query.url.as_str(), "https://github.com/gifnksm/souko.git");
    }

    #[test]
    fn test_parse_with_cyclic_config() {
        let config = QueryConfig {
            default_scheme: Some("gh".parse().unwrap()),
            scheme_alias: HashMap::from_iter([
                ("c1".parse().unwrap(), "c2".parse().unwrap()),
                ("c2".parse().unwrap(), "c3".parse().unwrap()),
                ("c3".parse().unwrap(), "c1".parse().unwrap()),
                ("d1".parse().unwrap(), "d2".parse().unwrap()),
                ("d3".parse().unwrap(), "d4".parse().unwrap()),
            ]),
            custom_scheme: HashMap::from_iter([
                ("d2".parse().unwrap(), "d3:x{path}".parse().unwrap()),
                ("d4".parse().unwrap(), "d1:y{path}".parse().unwrap()),
            ]),
        };

        let err = Query::parse("c1:test", &config).unwrap_err();
        assert_eq!(err.to_string(), "invalid config: circular alias `c1:test`");

        let err = Query::parse("d4:test", &config).unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid config: circular alias `d4:test` (expanded to `d4:xytest`)"
        );
    }
}
