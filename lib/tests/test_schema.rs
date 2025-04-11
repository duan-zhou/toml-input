use serde::Serialize;
use toml_input::{schema::{PrimaryType, Schema, Struct, StructField, UnitEnum, UnitVariant}, TomlInput};

#[test]
fn test_struct() {
    #[derive(Default, Serialize, TomlInput)]
    /// this is comment of struct
    struct Test {
        /// this is comment of field a
        a: i32,
        b: Option<usize>,
    }
    let res = Test::schema();
    // field a
    let mut pt0 = PrimaryType::empty();
    pt0.inner_type = "i32".to_string();
    pt0.inner_default = "0".to_string();
    let mut fd0 = StructField::empty();
    fd0.docs = " this is comment of field a".to_string();
    fd0.ident = "a".to_string();
    fd0.schema = Schema::Primary(pt0);
    // field b
    let mut pt1 = PrimaryType::empty();
    pt1.wrap_type = "Option".to_string();
    pt1.inner_type = "usize".to_string();
    pt1.inner_default = "0".to_string();
    let mut fd1 = StructField::empty();
    fd1.ident = "b".to_string();
    fd1.schema = Schema::Primary(pt1);
    // schema
    let fields = vec![fd0, fd1];
    let mut stt = Struct::empty();
    stt.docs = " this is comment of struct".to_string();
    stt.fields = fields;
    stt.inner_type = "Test".to_string();
    stt.inner_default = "{ a = 0 }".to_string();
    let sch = Schema::Struct(stt);
    assert_eq!(format!("{:?}", res), format!("{:?}", sch));
}


#[test]
fn test_enum() {
    #[derive(Serialize, TomlInput)]
    /// comment Test
    #[allow(dead_code)]
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
    let res = Test::schema();
    // variant a
    let mut uv0 = UnitVariant::empty();
    uv0.docs = " comment A".to_string();
    uv0.tag = "\"A\"".to_string();
    uv0.value = 0;
    // variant b
    let mut uv1 = UnitVariant::empty();
    uv1.docs = " comment B".to_string();
    uv1.tag = "\"B\"".to_string();
    uv1.value = 1;
    // schema
    let variants = vec![uv0, uv1];
    let mut uem = UnitEnum::empty();
    uem.docs = " comment Test".to_string();
    uem.variants = variants;
    uem.inner_type = "Test".to_string();
    uem.inner_default = "\"A\"".to_string();
    let sch = Schema::UnitEnum(uem);
    assert_eq!(format!("{:?}", res), format!("{:?}", sch));
}

