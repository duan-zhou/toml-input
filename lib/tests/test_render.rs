use serde::Serialize;
use toml_input::TomlInput;

#[test]
fn test_struct() {
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize, Default)]
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
    let test = Test { a: 2, b: None };
    let res = test.value_to_string().unwrap();
    let text = "# comment `Test`

# comment `a`
a = 2
# comment `b`
#! b = 0"
        .to_string();
    assert_eq!(res, text);
}

#[test]
fn test_array() {
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize, Default)]
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
    let res = test.value_to_string().unwrap();
    println!("{res}");
    let text = "# comment `Test`

# comment `a`
a = 1
# comment `b`
b = [2, 3, 4]".to_string();
    assert_eq!(res, text);
}

#[test]
fn test_nested() {
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Vec<usize>,
    }
    /// comment `Test1`
    #[derive(Debug, TomlInput, Serialize, Default)]
    struct Test1 {
        /// comment `c`
        c: i32,
        /// comment `d`
        d: Vec<Test>,
    }
    let test = Test {
        a: 1,
        b: vec![2, 3, 4],
    };
    let test1 = Test1 {
        c: 1,
        d: vec![test],
    };
    let res = test1.value_to_string().unwrap();
    println!("{res}");
    let text = "# comment `Test1`

# comment `c`
c = 1

[[d]]
# comment `a`
a = 1
# comment `b`
b = [2, 3, 4]".to_string();
    assert_eq!(res, text);

}

#[test]
fn test_enum() {
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }
    /// comment `Test`
    #[allow(dead_code)]
    #[derive(Debug, TomlInput, Serialize)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        B,
    }
    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::B
        }
    }
    let text = Test::schema_to_string().unwrap();
    let res = r#"# comment `Test`

# comment `a`
a = 0
# comment `A`
#! b = "A"
# comment `B`
b = "B""#;
    assert_eq!(res, text);
}

#[test]
fn test_enum_expand() {
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        #[toml_input(enum_expand = false)]
        b: TestEnum,
    }
    /// comment `Test`
    #[derive(Debug, TomlInput, Serialize)]
    #[allow(dead_code)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        B,
    }
    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::B
        }
    }
    let test = Test {
        a: 0,
        b: TestEnum::B,
    };
    let text = test.value_to_string().unwrap();
    let res = r#"# comment `Test`

# comment `a`
a = 0
# comment `B`
b = "B""#;
    assert_eq!(res, text);
}
