use serde::Serialize;
use toml::Value;
use toml_comment::{group::SectionGroup, TomlSchema};

#[test]
fn test_simple() {
    /// comment `Test`
    #[derive(Debug, TomlSchema, Serialize, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: Option<usize>,
    }

    let test = Test { a: 1, b: Some(2) };
    let value = Value::try_from(test).unwrap();
    let left = r#"SectionGroup { sections: [Section { id: 0, key: "", blocks: [Block { id: 0, section_id: 0, type_: FieldValue, key: ".a", value: "1", ident: "a", comment: None, hide: false }, Block { id: 0, section_id: 0, type_: FieldValue, key: ".b", value: "2", ident: "b", comment: None, hide: false }], type_: Table, comment: None, hide: false }] }"#;
    let right = format!("{:?}", SectionGroup::from_value(value).unwrap());
    assert_eq!(left, right);
}

#[test]
fn test_array() {
    /// comment `Test`
    #[derive(Debug, TomlSchema, Serialize, Default)]
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
    let value = Value::try_from(test).unwrap();
    let right = format!("{:?}", SectionGroup::from_value(value).unwrap());
    let left = r#"SectionGroup { sections: [Section { id: 0, key: "", blocks: [Block { id: 0, section_id: 0, type_: FieldValue, key: ".a", value: "1", ident: "a", comment: None, hide: false }, Block { id: 0, section_id: 0, type_: FieldValue, key: ".b", value: "[2, 3, 4]", ident: "b", comment: None, hide: false }], type_: Table, comment: None, hide: false }] }"#;
    assert_eq!(left, right);

    let test = Test {
        a: 1,
        b: vec![2, 3, 4],
    };
    /// comment `Test1`
    #[derive(Debug, TomlSchema, Serialize, Default)]
    struct Test1 {
        /// comment `c`
        c: i32,
        /// comment `d`
        d: Vec<Test>,
    }
    let test1 = Test1 {
        c: 1,
        d: vec![test],
    };
    let value = Value::try_from(test1).unwrap();
    let right = format!("{:?}", SectionGroup::from_value(value).unwrap());
    let left = r#"SectionGroup { sections: [Section { id: 0, key: "", blocks: [Block { id: 0, section_id: 0, type_: FieldValue, key: ".c", value: "1", ident: "c", comment: None, hide: false }], type_: Table, comment: None, hide: false }, Section { id: 0, key: ".d", blocks: [Block { id: 0, section_id: 0, type_: FieldValue, key: ".d.a", value: "1", ident: "a", comment: None, hide: false }, Block { id: 0, section_id: 0, type_: FieldValue, key: ".d.b", value: "[2, 3, 4]", ident: "b", comment: None, hide: false }], type_: Table, comment: None, hide: false }] }"#;
    assert_eq!(left, right);

}
