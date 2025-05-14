use serde::{Deserialize, Serialize};
use toml_input::TomlInput;

#[test]
fn test_schema() {
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize, Deserialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Option<usize>,
    }
    let res = Test::schema_to_string().unwrap();
    let text = "# comment `Test`

# comment `a`
a = 0
# comment `b`
b = 0"
        .to_string();
    assert_eq!(res, text);
    let _: Test = toml::from_str(&text).unwrap();
}

#[test]
fn test_value() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
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
fn test_value_array() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Vec<usize>,
    }

    let test = Test {
        a: 1,
        b: vec![2, 3, 4],
    };
    let res = test.clone().into_string().unwrap();
    println!("{res}");
    let text = "# comment `Test`

# comment `a`
a = 1
# comment `b`
b = [2, 3, 4]"
        .to_string();
    assert_eq!(res, text);
    let test1: Test = toml::from_str(&text).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_array() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Vec<usize>,
    }
    /// comment `Test1`
    #[derive(Debug, Clone, TomlInput, Serialize, Default, Deserialize, PartialEq)]
    struct Test1 {
        /// comment `c`
        c: i32,
        /// comment `d`
        d: Vec<Test>,
    }
    let test2 = Test {
        a: 1,
        b: vec![2, 3, 4],
    };
    let test3 = Test {
        a: 5,
        b: vec![6, 7, 8],
    };
    let test1 = Test1 {
        c: 1,
        d: vec![test2, test3],
    };
    let res = test1.clone().into_string().unwrap();
    println!("{res}");
    let text = "# comment `Test1`

# comment `c`
c = 1

# comment `d`
[[d]]
# comment `a`
a = 1
# comment `b`
b = [2, 3, 4]

# comment `d`
[[d]]
# comment `a`
a = 5
# comment `b`
b = [6, 7, 8]"
        .to_string();
    assert_eq!(res, text);
    let test2: Test1 = toml::from_str(&text).unwrap();
    assert_eq!(test1, test2);
}

#[test]
fn test_array_empty() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Vec<usize>,
    }

    let test = Test {
        a: 1,
        b: vec![],
    };
    let res = test.clone().into_string().unwrap();
    println!("{res}");
    let text = "# comment `Test`

# comment `a`
a = 1
# comment `b`
b = []"
        .to_string();
    assert_eq!(res, text);
    let test1: Test = toml::from_str(&text).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_nested() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Vec<usize>,
    }
    /// comment `Test1`
    #[derive(Debug, Clone, TomlInput, Serialize, Default, Deserialize, PartialEq)]
    struct Test1 {
        /// comment `c`
        c: i32,
        /// comment `d`
        d: Test,
        /// comment `e`
        e: Option<String>,
    }
    let test2 = Test {
        a: 1,
        b: vec![2, 3, 4],
    };
    let test1 = Test1 {
        c: 1,
        d: test2,
        e: None,
    };
    let res = test1.clone().into_string().unwrap();
    let text = r#"# comment `Test1`

# comment `c`
c = 1
# comment `e`
#!e = ""

# comment `d`
[d]
# comment `a`
a = 1
# comment `b`
b = [2, 3, 4]"#
        .to_string();
    assert_eq!(res, text);
    let test2: Test1 = toml::from_str(&text).unwrap();
    assert_eq!(test1, test2);
}

#[test]
fn test_section_comment() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Option<Test1>,
    }
    /// comment `Test1`
    #[derive(Debug, Clone, TomlInput, Serialize, Default, Deserialize, PartialEq)]
    struct Test1 {
        /// comment `c`
        c: i32,
    }
    let test = Test { a: 1, b: None };
    let res = test.clone().into_string().unwrap();
    println!("{res}");
    let text = r#"# comment `Test`

# comment `a`
a = 1

# comment `b`
#![b]
# comment `c`
#!c = 0"#
        .to_string();
    assert_eq!(res, text);
    let test1: Test = toml::from_str(&text).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_skip() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq)]
    #[serde(default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Option<usize>,
        /// skipped
        #[serde(skip)]
        c: i32,
    }

    impl Default for Test {
        fn default() -> Self {
            Test {
                a: 0,
                b: Some(1),
                c: 2,
            }
        }
    }
    let test = Test {
        a: 2,
        b: Some(3),
        c: 2,
    };
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
