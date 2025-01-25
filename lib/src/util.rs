use serde::Serialize;

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

pub fn append_line(text: &mut String) {
    if !text.trim().is_empty() {
        text.push_str("\n");
    }
}