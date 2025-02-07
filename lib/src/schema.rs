use serde::Serialize;

use crate::{
    block::{BlockSchema, BlockValueSchema},
    group::SectionMetaGroup,
    util, TAG,
};

#[derive(Debug, Clone)]
pub struct TupleEnum {
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: String,
    pub docs: String,
    pub variants: Vec<TupleVariant>,
}

impl TupleEnum {
    pub fn into_block_schemas(self, block_ident: &str) -> Vec<BlockSchema> {
        let TupleEnum {
            wrap_type,
            inner_type,
            inner_default,
            docs,
            variants,
        } = self;
        let mut data = Vec::new();
        for variant in variants {
            let TupleVariant {
                ident: variant_ident,
                docs: variant_docs,
                value: variant_value,
            } = variant;
            let value_schema = match variant_value {
                TupleEnumValue::Primary(pt) => BlockValueSchema::Primary(pt),
                TupleEnumValue::UnitEnum(ut) => BlockValueSchema::UnitEnum(ut),
            };
            let schema = BlockSchema {
                ident: block_ident.to_string() + TAG + &variant_ident,
                docs: variant_docs,
                value: value_schema,
                hide: false,
            };
            data.push(schema);
        }
        return data;
    }
}
#[derive(Debug, Clone)]
pub struct TupleVariant {
    pub ident: String,
    pub docs: String,
    pub value: TupleEnumValue,
}

#[derive(Debug, Clone)]
pub enum TupleEnumValue {
    Primary(PrimaryType),
    UnitEnum(UnitEnum),
}

#[derive(Debug, Clone)]
pub struct UnitEnum {
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: String,
    pub docs: String,
    pub variants: Vec<UnitVariant>,
}

impl UnitEnum {
    pub fn empty() -> Self {
        UnitEnum {
            wrap_type: String::new(),
            inner_type: String::new(),
            inner_default: String::new(),
            docs: String::new(),
            variants: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitVariant {
    pub tag: String,
    pub docs: String,
    pub value: isize,
}

impl UnitVariant {
    pub fn empty() -> UnitVariant {
        UnitVariant {
            tag: String::new(),
            docs: String::new(),
            value: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: String,
    pub docs: String,
    pub fields: Vec<StructField>,
}

impl Struct {
    pub fn empty() -> Self {
        Struct {
            wrap_type: String::new(),
            inner_type: String::new(),
            inner_default: String::new(),
            docs: String::new(),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: String,
    pub docs: String,
    pub flatten: bool,
    pub schema: Schema,
}

impl StructField {
    pub fn empty() -> Self {
        StructField {
            ident: String::new(),
            docs: String::new(),
            flatten: false,
            schema: Schema::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimaryType {
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: String,
    pub docs: String,
}

impl PrimaryType {
    pub fn empty() -> Self {
        PrimaryType {
            wrap_type: String::new(),
            inner_type: String::new(),
            inner_default: String::new(),
            docs: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Schema {
    None,
    Primary(PrimaryType),
    Struct(Struct),
    UnitEnum(UnitEnum),
    TupleEnum(TupleEnum),
}

impl Schema {
    pub fn set_wrap_type(&mut self, new: String) -> Option<String> {
        match self {
            Schema::None => None,
            Schema::Primary(ref mut data) => Some(std::mem::replace(&mut data.wrap_type, new)),
            Schema::Struct(ref mut data) => Some(std::mem::replace(&mut data.wrap_type, new)),
            Schema::UnitEnum(ref mut data) => Some(std::mem::replace(&mut data.wrap_type, new)),
            Schema::TupleEnum(ref mut data) => Some(std::mem::replace(&mut data.wrap_type, new)),
        }
    }
}

impl Schema {
    pub fn new_struct() -> Schema {
        Schema::Struct(Struct::empty())
    }
}

pub trait TomlSchema: Serialize + Sized {
    fn schema() -> Schema;
    fn schema_to_string() -> String {
        let schema = Self::schema();
        if let Schema::Struct(st) = schema {
            let meta = SectionMetaGroup::from(st);
            // println!("{:?}", meta);
            let group = meta.into_section_group();
            // println!("{:?}", group);
            return group.render().unwrap();
        }
        return String::new();
    }
    fn value_to_string(self) -> String {
        let schema = Self::schema();
        if let Schema::Struct(st) = schema {
            let meta = SectionMetaGroup::from(st);
            println!("{:?}", meta);
            let mut group = meta.into_section_group();
            println!("{:?}", group);
            let value = toml::Value::try_from(self).unwrap();
            group.merge_value(value).unwrap();
            return group.render().unwrap();
        }
        return String::new();
    }
}

macro_rules! impl_type_info_primary {
    ($t:ty, $name:expr) => {
        impl TomlSchema for $t {
            fn schema() -> Schema {
                let default = <$t as Default>::default();
                let mut data = PrimaryType::empty();
                data.inner_type = $name.to_string();
                data.inner_default = util::value_to_string(&default).unwrap();
                Schema::Primary(data)
            }
        }
    };
}

impl_type_info_primary!(String, "string");
impl_type_info_primary!(i8, "i8");
impl_type_info_primary!(i16, "i16");
impl_type_info_primary!(i32, "i32");
impl_type_info_primary!(i64, "i64");
impl_type_info_primary!(isize, "isize");
impl_type_info_primary!(u8, "u8");
impl_type_info_primary!(u16, "u16");
impl_type_info_primary!(u32, "u32");
impl_type_info_primary!(u64, "u64");
impl_type_info_primary!(usize, "usize");
impl_type_info_primary!(f32, "f32");
impl_type_info_primary!(f64, "f64");

impl<T: TomlSchema> TomlSchema for Option<T> {
    fn schema() -> Schema {
        let mut schema = T::schema();
        schema.set_wrap_type("Option".to_string());
        schema
    }
}

impl<T: TomlSchema> TomlSchema for Vec<T> {
    fn schema() -> Schema {
        let mut schema = T::schema();
        schema.set_wrap_type("Vec".to_string());
        schema
    }
}
