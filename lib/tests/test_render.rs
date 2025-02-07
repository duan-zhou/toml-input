use serde::Serialize;
use toml_comment::TomlSchema;

#[test]
fn test_struct() {
    /// comment `Test`
    #[derive(Debug, TomlSchema, Serialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Option<usize>,
    }

    let text = Test::schema_to_string();
    println!("{}", text);

    let test = Test {
        a: 2,
        b: None,
    };
    println!("{}", test.value_to_string());
}


#[test]
fn test_enum() {
    /// comment `Test`
    #[derive(Debug, TomlSchema, Serialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }

    /// comment `Test`
    #[derive(Debug, TomlSchema, Serialize)]
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

    let text = Test::schema_to_string();
    println!("{}", text);
}


#[test]
fn test_enum_tuple() {
    /// comment `Test`
    #[derive(Debug, TomlSchema, Serialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }

    /// comment `Test`
    #[derive(Debug, TomlSchema, Serialize)]
    enum TestEnum {
        /// comment `A`
        A(String),
        /// comment `B`
        B(i32),
    }

    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::B(1)
        }
    }

    let text = Test::schema_to_string();
    println!("test_enum_typle:\n{}", text);
}
