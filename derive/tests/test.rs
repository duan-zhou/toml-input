use serde::Serialize;
use toml_input_derive::TomlInput;

#[test]
fn test_struct() {
    #[derive(TomlInput, Debug, Default, Serialize)]
    /// this is comment of struct
    #[serde()]
    struct TestStruct {
        /// this is comment of field
        a: i32,
    }

    use toml_input::TomlInput;
    dbg!(TestStruct::schema());
}

#[test]
fn test_enum() {
    /// this is comment of enum
    #[derive(TomlInput, Debug, Serialize)]
    #[toml_input(enum_expand)]
    enum TestEnum {
        A,
        B,
        C2,
    }
    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::B
        }
    }
    use toml_input::TomlInput;
    dbg!(TestEnum::schema());
}
