use serde::{Deserialize, Serialize};
use toml_input::TomlInput;

#[test]
fn test_skip_none() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        #[toml_input(option_style = "skip_none")]
        b: Option<usize>,
    }
    let test = Test { a: 2, b: None };
    let res = test.clone().into_string().unwrap();
    let text = "# comment `Test`

# comment `a`
a = 2"
        .to_string();
    assert_eq!(res, text);
    let test1: Test = toml::from_str(&text).unwrap();
    assert_eq!(test, test1);

    let test = Test { a: 2, b: Some(3) };
    let res = test.clone().into_string().unwrap();
    let text = "# comment `Test`

# comment `a`
a = 2
# comment `b`
b = 3"
        .to_string();
    assert_eq!(res, text);
    let test1: Test = toml::from_str(&text).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_expand_none() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        #[toml_input(option_style = "expand_none")]
        b: Option<usize>,
    }
    let test = Test { a: 2, b: None };
    let res = test.clone().into_string().unwrap();
    let text = "# comment `Test`

# comment `a`
a = 2
# comment `b`
#!b = 0"
        .to_string();
    assert_eq!(res, text);
    let test1: Test = toml::from_str(&text).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_comment_style() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Option<usize>,
    }
    let test = Test { a: 2, b: None };
    let mut content = test.clone().into_content().unwrap();
    content.config_comment_style_hide();
    let res = content.render().unwrap();
    let text = "a = 2
#!b = 0"
        .to_string();
    assert_eq!(res, text);
    let test1: Test = toml::from_str(&text).unwrap();
    assert_eq!(test, test1);
}
