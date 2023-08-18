use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{Display, Write},
    hash::Hash,
    str::FromStr,
};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "String")]
pub(crate) struct Template {
    parts: Vec<Parts>,
}

impl Template {
    pub(crate) fn expand<K, V>(&self, variables: &HashMap<K, V>) -> String
    where
        K: Borrow<str> + Eq + Hash,
        V: Display,
    {
        let mut result = String::new();
        for part in &self.parts {
            match part {
                Parts::Variable(name) => {
                    if let Some(value) = variables.get(name) {
                        write!(&mut result, "{value}").unwrap();
                    }
                }
                Parts::Text(text) => result.push_str(text),
            }
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Parts {
    Text(String),
    Variable(String),
}

impl Parts {
    fn text(s: impl Display) -> Self {
        Self::Text(s.to_string())
    }
    fn variable(s: impl Display) -> Self {
        Self::Variable(s.to_string())
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    #[error("unexpected character: {0:?}")]
    UnexpectedChar(char),
    #[error("no closing brace '}}' found")]
    NoClosingBrace,
    #[error("invalid variable: {0}")]
    InvalidVariable(String),
}

impl FromStr for Template {
    type Err = ParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut parts = vec![];

        while let Some(idx) = s.find(['{', '}']) {
            let (text, brace) = s.split_at(idx);
            push_str(&mut parts, text);
            s = brace;
            if let Some(rest) = s.strip_prefix("{{") {
                s = rest;
                push_char(&mut parts, '{');
                continue;
            }
            if let Some(rest) = s.strip_prefix("}}") {
                s = rest;
                push_char(&mut parts, '}');
                continue;
            }
            if s.starts_with('}') {
                bail!(ParseError::UnexpectedChar('}'));
            }
            assert!(s.starts_with('{'));
            if let Some(end) = s.find('}') {
                let variable = s[1..end].trim();
                if !is_valid_variable(variable) {
                    bail!(ParseError::InvalidVariable(variable.to_string()));
                }
                s = &s[end + 1..];
                parts.push(Parts::variable(variable));
            } else {
                bail!(ParseError::NoClosingBrace);
            }
        }
        push_str(&mut parts, s);
        Ok(Self { parts })
    }
}

fn push_char(parts: &mut Vec<Parts>, ch: char) {
    if let Some(Parts::Text(last)) = parts.last_mut() {
        last.push(ch)
    } else {
        parts.push(Parts::text(ch));
    }
}

fn push_str(parts: &mut Vec<Parts>, s: &str) {
    if s.is_empty() {
        return;
    }
    if let Some(Parts::Text(last)) = parts.last_mut() {
        last.push_str(s)
    } else {
        parts.push(Parts::text(s));
    }
}

fn is_valid_variable(s: &str) -> bool {
    static VARIABLE_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]+$").unwrap());
    VARIABLE_RE.is_match(s)
}

impl TryFrom<String> for Template {
    type Error = ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(s.as_str())
    }
}

impl TryFrom<&str> for Template {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_variable() {
        assert!(is_valid_variable("foo123"));
        assert!(is_valid_variable("foo_bar"));
        assert!(is_valid_variable("foo_bar_baz"));
        assert!(!is_valid_variable(""));
        assert!(!is_valid_variable("foo bar"));
        assert!(!is_valid_variable("foo-bar"));
        assert!(!is_valid_variable("foo_bar-baz"));
        assert!(!is_valid_variable("123foo"));
    }

    fn t(s: impl Display) -> Parts {
        Parts::text(s)
    }
    fn v(s: impl Display) -> Parts {
        Parts::variable(s)
    }

    #[test]
    fn test_from_str_ok() {
        assert_eq!(Template::from_str("foo").unwrap().parts, vec![t("foo")]);
        assert_eq!(Template::from_str("").unwrap().parts, vec![]);
        assert_eq!(Template::from_str("{var}").unwrap().parts, vec![v("var")]);
        assert_eq!(
            Template::from_str("{ var2 }").unwrap().parts,
            vec![v("var2")]
        );
        assert_eq!(
            Template::from_str("xxx#$%%{{)*)_*}}").unwrap().parts,
            vec![t("xxx#$%%{)*)_*}")]
        );
        assert_eq!(
            Template::from_str("{{{var}}}").unwrap().parts,
            vec![t("{"), v("var"), t("}")]
        );
        assert_eq!(
            Template::from_str("{{{var}}} }}{var}{{").unwrap().parts,
            vec![t("{"), v("var"), t("} }"), v("var"), t("{")]
        );
    }

    #[test]
    fn test_from_str_err() {
        assert!(matches!(
            Template::from_str("{var").unwrap_err(),
            ParseError::NoClosingBrace
        ));
        assert!(matches!(
            Template::from_str("{{var}").unwrap_err(),
            ParseError::UnexpectedChar('}')
        ));
        assert!(matches!(
            Template::from_str("{2var}").unwrap_err(),
            ParseError::InvalidVariable(s) if s == "2var"
        ));
        assert!(matches!(
            Template::from_str("{v ar}").unwrap_err(),
            ParseError::InvalidVariable(s) if s == "v ar"
        ));
    }

    #[test]
    fn test_expand() {
        let temp = Template::from_str("I like {food} very much").unwrap();
        assert_eq!(
            temp.expand(&HashMap::from_iter([("food", "sushi")])),
            "I like sushi very much"
        );
        assert_eq!(
            temp.expand(&HashMap::from_iter([("food", "ramen")])),
            "I like ramen very much"
        );

        let temp = Template::from_str("no variable template").unwrap();
        assert_eq!(
            temp.expand(&HashMap::from_iter([
                ("food", "sushi"),
                ("drink", "green tea")
            ])),
            "no variable template"
        );

        let temp =
            Template::from_str("{food} is my favorite food. I eat {food} {frequency}").unwrap();
        assert_eq!(
            temp.expand(&HashMap::from_iter([
                ("food", "sushi"),
                ("frequency", "everyday")
            ])),
            "sushi is my favorite food. I eat sushi everyday"
        );
    }
}
