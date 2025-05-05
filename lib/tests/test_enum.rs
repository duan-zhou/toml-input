use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};
use toml_input::TomlInput;

#[test]
fn test_schema() {
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize, Deserialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[allow(dead_code)]
    #[derive(Debug, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize, Default)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        #[default]
        B,
    }

    let text = Test::schema_to_string().unwrap();
    println!("{}", text);
    let res = r#"# comment `Test`

# comment `a`
a = 0
# comment `A`
#!b = "A"
# comment `B`
b = "B""#;
    assert_eq!(res, text);
    let _: Test = toml::from_str(res).unwrap();
}

#[test]
fn test_value() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[allow(dead_code)]
    #[derive(
        Debug, Clone, EnumIter, AsRefStr, TomlInput, Serialize, PartialEq, Deserialize, Default,
    )]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        #[default]
        B,
    }

    let test = Test {
        a: 1,
        b: TestEnum::A,
    };
    let text = test.clone().into_string().unwrap();
    // println!("{}", text);
    let res = r#"# comment `Test`

# comment `a`
a = 1
# comment `A`
b = "A"
# comment `B`
#!b = "B""#;
    assert_eq!(res, text);
    let test1: Test = toml::from_str(res).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_single() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, Default, PartialEq)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        #[toml_input(enum_style = "single")]
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[derive(Debug, Clone, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize, PartialEq)]
    #[allow(dead_code)]
    #[derive(Default)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        #[default]
        B,
    }

    let test = Test {
        a: 0,
        b: TestEnum::B,
    };
    let text = test.clone().into_string().unwrap();
    println!("{}", text);
    let res = r#"# comment `Test`

# comment `a`
a = 0
# comment `B`
b = "B""#;
    assert_eq!(res, text);
    let test1: Test = toml::from_str(res).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_fold() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, Default, PartialEq)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        #[toml_input(enum_style = "fold")]
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[derive(Debug, Clone, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize, PartialEq)]
    #[allow(dead_code)]
    #[derive(Default)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        #[default]
        B,
    }

    let test = Test {
        a: 0,
        b: TestEnum::B,
    };
    let text = test.clone().into_string().unwrap();
    println!("{}", text);
    let res = r#"# comment `Test`

# comment `a`
a = 0
# b = "A" | "B"
# comment `B`
b = "B""#;
    assert_eq!(res, text);
    let test1: Test = toml::from_str(res).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_tuple() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[derive(Debug, Clone, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize, PartialEq)]
    #[allow(dead_code)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        B(String),
    }
    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::B(String::new())
        }
    }
    let test = Test {
        a: 0,
        b: TestEnum::B("test B".to_string()),
    };
    let text = test.clone().into_string().unwrap();
    println!("{}", text);
    let res = r#"# comment `Test`

# comment `a`
a = 0
# comment `A`
#!b = "A"
# comment `B`
b = { B = "test B" }"#;
    assert_eq!(res, text);
    let test1: Test = toml::from_str(res).unwrap();
    assert_eq!(test, test1);
}

#[test]
fn test_struct() {
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[derive(Debug, Clone, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize, PartialEq)]
    #[allow(dead_code)]
    #[derive(Default)]
    enum TestEnum {
        /// comment `A`
        #[default]
        A,
        /// comment `B`
        B { c: usize, d: f64 },
    }

    let test = Test {
        a: 0,
        b: TestEnum::B { c: 2, d: 1.5 },
    };
    let text = test.clone().into_string().unwrap();
    println!("{}", text);
    let res = r#"# comment `Test`

# comment `a`
a = 0
# comment `A`
#!b = "A"
# comment `B`
b = { B = { c = 2, d = 1.5 } }"#;
    assert_eq!(res, text);
    let test1: Test = toml::from_str(res).unwrap();
    assert_eq!(test, test1);
}
