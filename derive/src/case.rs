use self::RenameRule::*;

#[derive(Clone)]
pub enum RenameRule {
    /// Don't apply a default rename rule.
    None,
    /// field alias
    Alias(String),
    /// Rename direct children to "lowercase" style.
    LowerCase,
    /// Rename direct children to "UPPERCASE" style.
    UpperCase,
    /// Rename direct children to "PascalCase" style, as typically used for
    /// enum variants.
    PascalCase,
    /// Rename direct children to "camelCase" style.
    CamelCase,
    /// Rename direct children to "snake_case" style, as commonly used for
    /// fields.
    SnakeCase,
    /// Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly
    /// used for constants.
    ScreamingSnakeCase,
    /// Rename direct children to "kebab-case" style.
    KebabCase,
    /// Rename direct children to "SCREAMING-KEBAB-CASE" style.
    ScreamingKebabCase,
}

impl RenameRule {
    pub fn new(text: &str) -> Self {
        match text {
            "lowercase" => LowerCase,
            "UPPERCASE" => UpperCase,
            "PascalCase" => PascalCase,
            "camelCase" => CamelCase,
            "snake_case" => SnakeCase,
            "SCREAMING_SNAKE_CASE" => ScreamingSnakeCase,
            "kebab-case" => KebabCase,
            "SCREAMING-KEBAB-CASE" => ScreamingKebabCase,
            _ if text.trim().is_empty()=> None,
            _ => Alias(text.to_string()),
        }
    }
    // use in `rename`
    pub fn rename(&self, name: &str) -> String {
        match self {
            Alias(alias) => alias.to_string(),
            _ => name.to_string(),
        }
    }
    // use in `rename_all`
    pub fn rename_all(&self, name: &str) -> String {
        match self {
            UpperCase => name.to_ascii_uppercase(),
            PascalCase => {
                let mut pascal = String::new();
                let mut capitalize = true;
                for ch in name.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        pascal.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        pascal.push(ch);
                    }
                }
                pascal
            }
            CamelCase => {
                let pascal = PascalCase.rename_all(name);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            ScreamingSnakeCase => name.to_ascii_uppercase(),
            KebabCase => name.replace('_', "-"),
            ScreamingKebabCase => ScreamingSnakeCase.rename_all(name).replace('_', "-"),
            None | LowerCase | SnakeCase => name.to_string(),
            Alias(_) => name.to_string()
        }
    }
}

#[test]
fn rename() {
    for &(original, upper, pascal, camel, screaming, kebab, screaming_kebab) in &[
        (
            "outcome", "OUTCOME", "Outcome", "outcome", "OUTCOME", "outcome", "OUTCOME",
        ),
        (
            "very_tasty",
            "VERY_TASTY",
            "VeryTasty",
            "veryTasty",
            "VERY_TASTY",
            "very-tasty",
            "VERY-TASTY",
        ),
        ("a", "A", "A", "a", "A", "a", "A"),
        ("z42", "Z42", "Z42", "z42", "Z42", "z42", "Z42"),
    ] {
        assert_eq!(None.rename_all(original), original);
        assert_eq!(UpperCase.rename_all(original), upper);
        assert_eq!(PascalCase.rename_all(original), pascal);
        assert_eq!(CamelCase.rename_all(original), camel);
        assert_eq!(SnakeCase.rename_all(original), original);
        assert_eq!(ScreamingSnakeCase.rename_all(original), screaming);
        assert_eq!(KebabCase.rename_all(original), kebab);
        assert_eq!(ScreamingKebabCase.rename_all(original), screaming_kebab);
    }
}
