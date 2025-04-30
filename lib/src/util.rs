use serde::Serialize;

use crate::{COMMENT, TAG};

pub fn value_to_string<T: Serialize>(value: &T) -> Result<String, toml::ser::Error> {
    let mut ser_value = String::new();
    let serializer = toml::ser::ValueSerializer::new(&mut ser_value);
    value.serialize(serializer)?;
    Ok(ser_value)
}

pub fn prefix_lines(text: &str, prefix: &str) -> String {
    let lines: Vec<String> = text.lines().map(|line| prefix.to_string() + line).collect();
    lines.join("\n")
}

pub fn comment_lines(text: &str) -> String {
    let lines: Vec<String> = text
        .lines()
        .map(|line| COMMENT.to_string() + line)
        .collect();
    lines.join("\n")
}

pub fn append_line(text: &mut String) {
    if !text.trim().is_empty() {
        text.push_str("\n");
    }
}

pub fn remove_prefix_tag(key: &str) -> String {
    let key = key.trim();
    if key.starts_with(".") {
        return key[1..].to_string();
    }
    key.to_string()
}

pub fn increase_key(key: &mut String, ident: impl AsRef<str>) {
    if key.is_empty() {
        *key = ident.as_ref().to_string();
    } else {
        *key = format!("{}{}{}", ident.as_ref(), TAG, key);
    }
}

pub fn key_parent(key: &str) -> String {
    let mut idents: Vec<_> = key.split(TAG).collect();
    idents.pop();
    idents.join(TAG)
}
