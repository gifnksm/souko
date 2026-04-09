use std::{
    any,
    collections::HashMap,
    fmt::{self, Display, Write},
    str::FromStr,
};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "String")]
pub(crate) struct Template {
    parts: Vec<Parts>,
}

#[derive(Debug, Error)]
pub(crate) enum TemplateValidateError {
    #[error("unknown template variable: {0:?}")]
    UnknownTemplateVariable(String),
}

impl Template {
    pub(crate) fn validate_and_expand<C>(
        &self,
        context: &C,
    ) -> Result<String, TemplateValidateError>
    where
        C: TemplateContext,
    {
        self.validate::<C>()?;
        Ok(self.expand(context))
    }

    pub(crate) fn validate<C>(&self) -> Result<(), TemplateValidateError>
    where
        C: TemplateContext,
    {
        let valid_variables = C::default().to_hashmap();
        for part in &self.parts {
            match part {
                Parts::Text(_) => {}
                Parts::Variable(name) => {
                    if !valid_variables.contains_key(name) {
                        return Err(TemplateValidateError::UnknownTemplateVariable(name.clone()));
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn expand<C>(&self, context: &C) -> String
    where
        C: TemplateContext,
    {
        let mut result = String::new();
        let variables = context.to_hashmap();
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

pub(crate) trait TemplateContext: Serialize + Default + fmt::Debug {
    fn to_hashmap(&self) -> HashMap<String, String> {
        let Ok(Value::Object(obj)) = serde_json::to_value(self) else {
            panic!(
                "TemplateContext invariant violated: context must serialize to a JSON object (type: {})",
                any::type_name::<Self>()
            );
        };
        obj.into_iter()
            .map(|(k, v)| {
                let Some(v) = v.as_str() else {
                    panic!(
                        "TemplateContext invariant violated: all template values must serialize to JSON string (type: {}, key: {})",
                        any::type_name::<Self>(),
                        k
                    );
                };
                (k, v.to_owned())
            })
            .collect()
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
                return Err(ParseError::UnexpectedChar('}'));
            }
            assert!(s.starts_with('{'));
            if let Some(end) = s.find('}') {
                let variable = s[1..end].trim();
                if !is_valid_variable(variable) {
                    return Err(ParseError::InvalidVariable(variable.to_string()));
                }
                s = &s[end + 1..];
                parts.push(Parts::variable(variable));
            } else {
                return Err(ParseError::NoClosingBrace);
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
        Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap());
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

    #[derive(Debug, Default, Serialize)]
    struct TestTemplateContext {
        food: String,
        frequency: String,
        drink: String,
    }

    impl TestTemplateContext {
        fn new(food: &str, frequency: &str, drink: &str) -> Self {
            Self {
                food: food.to_owned(),
                frequency: frequency.to_owned(),
                drink: drink.to_owned(),
            }
        }
    }

    impl TemplateContext for TestTemplateContext {}

    #[test]
    fn test_is_valid_variable() {
        assert!(is_valid_variable("foo123"));
        assert!(is_valid_variable("foo_bar"));
        assert!(is_valid_variable("foo_bar_baz"));
        assert!(is_valid_variable("a"));
        assert!(is_valid_variable("_"));
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
    fn test_validate_ok() {
        let temp = Template::from_str("I like {food} very much").unwrap();
        assert!(temp.validate::<TestTemplateContext>().is_ok());

        let temp = Template::from_str("{food} {frequency} {drink}").unwrap();
        assert!(temp.validate::<TestTemplateContext>().is_ok());
    }

    #[test]
    fn test_validate_err_unknown_template_variable() {
        let temp = Template::from_str("I like {food} and {dessert}").unwrap();
        let err = temp.validate::<TestTemplateContext>().unwrap_err();
        assert!(matches!(
            err,
            TemplateValidateError::UnknownTemplateVariable(s) if s == "dessert"
        ));
    }

    #[test]
    fn test_validate_and_expand() {
        let temp = Template::from_str("{food} {frequency}").unwrap();
        assert_eq!(
            temp.validate_and_expand(&TestTemplateContext::new("sushi", "everyday", "tea"))
                .unwrap(),
            "sushi everyday"
        );
    }

    #[test]
    fn test_expand() {
        let temp = Template::from_str("I like {food} very much").unwrap();
        assert_eq!(
            temp.expand(&TestTemplateContext::new("sushi", "", "")),
            "I like sushi very much"
        );
        assert_eq!(
            temp.expand(&TestTemplateContext::new("ramen", "", "")),
            "I like ramen very much"
        );

        let temp = Template::from_str("no variable template").unwrap();
        assert_eq!(
            temp.expand(&TestTemplateContext::new("sushi", "", "green tea")),
            "no variable template"
        );

        let temp =
            Template::from_str("{food} is my favorite food. I eat {food} {frequency}").unwrap();
        assert_eq!(
            temp.expand(&TestTemplateContext::new("sushi", "everyday", "")),
            "sushi is my favorite food. I eat sushi everyday"
        );
    }
}
