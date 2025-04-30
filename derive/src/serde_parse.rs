use convert_case::{Case, Casing};
use regex::Regex;
use syn::{Attribute, Meta, MetaList};

#[derive(Default)]
pub struct RenameRule {
    pub case: Option<Case<'static>>,
    pub alias: Option<String>,
}

impl RenameRule {
    pub fn new_case(case: Case<'static>) -> Self {
        RenameRule {
            case: Some(case),
            alias: None,
        }
    }

    pub fn case_to(&self, origin: String) -> String {
        if let Some(case) = self.case {
            origin.to_case(case)
        } else {
            origin
        }
    }

    pub fn alias(&self, origin: String) -> String {
        if let Some(alias) = &self.alias {
            alias.to_string()
        } else {
            origin
        }
    }
}

pub fn rename_rule(attrs: &[Attribute]) -> RenameRule {
    let mut rule = RenameRule::default();
    if let Some(text) = parse_serde_text(attrs) {
        let text = text.trim();
        let rename_text = parse_rename(text).unwrap_or("".to_string());
        rule = match rename_text.as_str() {
            "lowercase" => RenameRule::new_case(Case::Lower),
            "UPPERCASE" => RenameRule::new_case(Case::Upper),
            "PascalCase" => RenameRule::new_case(Case::Pascal),
            "camelCase" => RenameRule::new_case(Case::Camel),
            "snake_case" => RenameRule::new_case(Case::Snake),
            "SCREAMING_SNAKE_CASE" => RenameRule::new_case(Case::UpperSnake),
            "kebab-case" => RenameRule::new_case(Case::Kebab),
            "SCREAMING-KEBAB-CASE" => RenameRule::new_case(Case::UpperKebab),
            _ if text.is_empty() => RenameRule {
                alias: Some(text.to_string()),
                ..Default::default()
            },
            _ => RenameRule::default(),
        };
    }
    rule
}

pub fn flatten(attrs: &[Attribute]) -> bool {
    if let Some(text) = parse_serde_text(attrs) {
        let re = Regex::new(r",?s*flatten").unwrap();
        return re.is_match(&text);
    }
    false
}

fn parse_serde_text(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        if let Meta::List(MetaList { tokens, .. }) = attr.meta.clone() {
            return Some(tokens.to_string());
        }
    }
    None
}

fn parse_rename(text: &str) -> Option<String> {
    None.or(parse_rename_simple(text))
        .or(parse_rename_all(text))
        .or(parse_rename_serialize(text))
        .or(parse_rename_all_serialize(text))
}

// serde(rename="ser_name")
fn parse_rename_simple(text: &str) -> Option<String> {
    let re = Regex::new(r#"rename\s*=\s*"(?<value>.+)""#).unwrap();
    re.captures(text).map(|cap| cap["value"].to_string())
}

// rename(serialize = "ser_name")
fn parse_rename_serialize(text: &str) -> Option<String> {
    let re = Regex::new(r#"rename\s*(\(|.*,)\s*serialize\s*=\s*"(?<value>.+?)""#).unwrap();
    re.captures(text).map(|cap| cap["value"].to_string())
}

// serde(rename_all="xxx")
fn parse_rename_all(text: &str) -> Option<String> {
    let re = Regex::new(r#"rename_all\s*=\s*"(?<value>.+)""#).unwrap();
    re.captures(text).map(|cap| cap["value"].to_string())
}

// rename_all(serialize = "ser_name")
fn parse_rename_all_serialize(text: &str) -> Option<String> {
    let re = Regex::new(r#"rename_all\s*(\(|.*,)\s*serialize\s*=\s*"(?<value>.+?)""#).unwrap();
    re.captures(text).map(|cap| cap["value"].to_string())
}

#[test]
fn test_rename_simple() {
    let text = r#"rename="ser_name""#;
    assert_eq!("ser_name", parse_rename_simple(text).unwrap())
}

#[test]
fn test_rename_all() {
    let text = r#"rename_all="ser_name""#;
    assert_eq!("ser_name", parse_rename_all(text).unwrap())
}

#[test]
fn test_rename_serialize() {
    let text = r#"rename(serialize = "ser_name")"#;
    assert_eq!("ser_name", parse_rename_serialize(text).unwrap());
    let text = r#"rename(serialize = "ser_name", deserialize = "de_name")"#;
    assert_eq!("ser_name", parse_rename_serialize(text).unwrap());
    let text = r#"rename(deserialize = "de_name", serialize = "ser_name")"#;
    assert_eq!("ser_name", parse_rename_serialize(text).unwrap());
}

#[test]
fn test_rename_all_serialize() {
    let text = r#"rename_all(serialize = "ser_name")"#;
    assert_eq!("ser_name", parse_rename_all_serialize(text).unwrap());
    let text = r#"rename_all(serialize = "ser_name", deserialize = "de_name")"#;
    assert_eq!("ser_name", parse_rename_all_serialize(text).unwrap());
    let text = r#"rename_all(deserialize = "de_name", serialize = "ser_name")"#;
    assert_eq!("ser_name", parse_rename_all_serialize(text).unwrap());
}
