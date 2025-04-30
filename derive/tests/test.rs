use serde::Serialize;
use strum_macros::{AsRefStr, EnumIter};
use toml_input_derive::TomlInput;

#[test]
fn test_struct() {
    #[derive(TomlInput, Debug, Default, Serialize)]
    /// this is comment of struct
    struct TestStruct {
        /// this is comment of field
        a: i32,
        /// optional field
        #[toml_input(inner_default = "1")]
        b: Option<u32>,
    }

    use toml_input::TomlInput;
    dbg!(TestStruct::schema().unwrap());
}

#[test]
fn test_enum() {
    /// this is comment of enum
    #[derive(EnumIter, AsRefStr, TomlInput, Debug, Serialize)]
    #[toml_input(enum_style = "expand")]
    #[derive(Default)]
    enum TestEnum {
        A,
        #[default]
        B,
        C2,
    }
    
    use toml_input::TomlInput;
    dbg!(TestEnum::schema().unwrap());
}
