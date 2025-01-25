use regex::Regex;
use syn::{Attribute, Meta, MetaList};

use crate::case::RenameRule;

pub fn rename_rule(attrs: &[Attribute]) -> RenameRule {
    let mut rule = RenameRule::None;
    if let Some(text) = parse_serde_text(attrs) {
        let rename_text = parse_rename(&text).unwrap_or("".to_string());
        rule = RenameRule::new(&rename_text);
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
