use serde::Serialize;
use toml_comment::TomlSchema;

#[test]
fn test_struct() {
    #[derive(Default, Serialize, TomlSchema)]
    /// this is comment of struct
    struct Test {
        /// this is comment of field
        a: i32,
        b: Option<usize>,
    }
    let schema = Test::schema();
    let left = r#"Struct(Struct { wrap_type: "", inner_type: "Test", inner_default: "{ a = 0 }", docs: " this is comment of struct", fields: [StructField { ident: "a", docs: " this is comment of field", flatten: false, schema: Primary(PrimaryType { wrap_type: "", inner_type: "i32", inner_default: "0", docs: "" }) }, StructField { ident: "b", docs: "", flatten: false, schema: Primary(PrimaryType { wrap_type: "Option", inner_type: "usize", inner_default: "0", docs: "" }) }] })"#;
    let right = format!("{:?}", schema);
    assert_eq!(left, right);
}


#[test]
fn test_enum() {
    #[derive(Serialize, TomlSchema)]
    /// comment Test
    enum Test {
        /// comment A
        A,
        /// comment B
        B,
    }

    impl Default for Test {
        fn default() -> Self {
            Test::A
        }
    }
    let schema = Test::schema();
    let left = r#"UnitEnum(UnitEnum { wrap_type: "", inner_type: "Test", inner_default: "\"A\"", docs: " comment Test", variants: [UnitVariant { tag: "A", docs: " comment A", value: 0 }, UnitVariant { tag: "B", docs: " comment B", value: 1 }] })"#;
    let right = format!("{:?}", schema);
    assert_eq!(left, right);
}
