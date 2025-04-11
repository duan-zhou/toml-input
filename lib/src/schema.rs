use std::path::PathBuf;

use serde::Serialize;

use crate::{error::Error, root::RootMeta, util};

#[derive(Debug, Clone)]
pub struct TomlConfig {
    pub enum_expand: bool,
}

impl Default for TomlConfig {
    fn default() -> Self {
        TomlConfig { enum_expand: true }
    }
}

#[derive(Debug, Clone)]
pub struct UnitEnum {
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: String,
    pub docs: String,
    pub variants: Vec<UnitVariant>,
    pub config: TomlConfig,
}

impl UnitEnum {
    pub fn empty() -> Self {
        UnitEnum {
            wrap_type: String::new(),
            inner_type: String::new(),
            inner_default: String::new(),
            docs: String::new(),
            variants: Vec::new(),
            config: TomlConfig::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitVariant {
    pub tag: String,
    pub docs: String,
    pub value: isize,
    pub config: TomlConfig,
}

impl UnitVariant {
    pub fn empty() -> UnitVariant {
        UnitVariant {
            tag: String::new(),
            docs: String::new(),
            value: 0,
            config: TomlConfig::default(),
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
    pub config: TomlConfig,
}

impl Struct {
    pub fn empty() -> Self {
        Struct {
            wrap_type: String::new(),
            inner_type: String::new(),
            inner_default: String::new(),
            docs: String::new(),
            fields: Vec::new(),
            config: TomlConfig::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: String,
    pub docs: String,
    pub flatten: bool,
    pub schema: Schema,
    pub config: TomlConfig,
}

impl StructField {
    pub fn empty() -> Self {
        StructField {
            ident: String::new(),
            docs: String::new(),
            flatten: false,
            schema: Schema::None,
            config: TomlConfig::default(),
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
}

impl Schema {
    pub fn set_wrap_type(&mut self, new: String) -> Option<String> {
        match self {
            Schema::None => None,
            Schema::Primary(ref mut data) => Some(std::mem::replace(&mut data.wrap_type, new)),
            Schema::Struct(ref mut data) => Some(std::mem::replace(&mut data.wrap_type, new)),
            Schema::UnitEnum(ref mut data) => Some(std::mem::replace(&mut data.wrap_type, new)),
        }
    }
}

impl Schema {
    pub fn new_struct() -> Schema {
        Schema::Struct(Struct::empty())
    }
}

pub trait TomlInput: Serialize + Sized {
    fn schema() -> Schema;
    fn schema_to_string() -> Result<String, Error> {
        let schema = Self::schema();
        if let Schema::Struct(st) = schema {
            let meta = RootMeta::from(st);
            let root = meta.into_root();
            return root.render();
        }
        return Ok(String::new());
    }
    fn value_to_string(self) -> Result<String, Error> {
        let schema = Self::schema();
        if let Schema::Struct(st) = schema {
            let meta = RootMeta::from(st);
            let mut root = meta.into_root();
            let value = toml::Value::try_from(self).unwrap();
            root.merge_value(value).unwrap();
            return root.render();
        }
        return Ok(String::new());
    }
}

macro_rules! impl_type_info_primary {
    ($t:ty, $name:expr) => {
        impl TomlInput for $t {
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

impl_type_info_primary!(bool, "bool");
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
impl_type_info_primary!(PathBuf, "path");

impl<T: TomlInput> TomlInput for Option<T> {
    fn schema() -> Schema {
        let mut schema = T::schema();
        schema.set_wrap_type("Option".to_string());
        schema
    }
}

impl<T: TomlInput> TomlInput for Vec<T> {
    fn schema() -> Schema {
        let mut schema = T::schema();
        schema.set_wrap_type("Vec".to_string());
        schema
    }
}
