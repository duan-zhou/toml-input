use std::path::PathBuf;

use crate::{
    error::Error, schema::{Meta, PrimSchema}, section::TomlContent, value::{ArrayValue, PrimValue}, Schema, TomlValue, Value
};
use serde::Serialize;

pub trait TomlInput: Serialize + Sized {
    fn schema() -> Result<Schema, Error>;
    fn into_value(self) -> Result<Value, Error>;
    fn schema_to_string() -> Result<String, Error> {
        let schema = Self::schema()?;
        let sections = schema.flatten();
        let mut content = TomlContent { sections };
        content.config_commented(false);
        content.render()
    }
    fn into_string(self) -> Result<String, Error> {
        let schema = Self::schema()?;
        let sections = schema.flatten();
        let mut content = TomlContent { sections };
        let value = self.into_value()?;
        content.merge_value(value);
        content.render()
    }
}

macro_rules! impl_type_info_primary {
    ($t:ty, $name:expr) => {
        impl TomlInput for $t {
            fn schema() -> Result<Schema, Error> {
                let default = <$t as Default>::default();
                let raw = TomlValue::try_from(default)?;
                let mut meta = Meta::default();
                meta.inner_type = $name.to_string();
                meta.inner_default = PrimValue::new(raw);
                let data = PrimSchema {
                    meta,
                    ..Default::default()
                };
                Ok(Schema::Prim(data))
            }
            fn into_value(self) -> Result<Value, Error> {
                let raw = TomlValue::try_from(self)?;
                Ok(Value::new_prim(raw))
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
    fn schema() -> Result<Schema, Error> {
        let mut schema = T::schema()?;
        schema.set_wrap_type("Option".to_string());
        Ok(schema)
    }
    fn into_value(self) -> Result<Value, Error> {
        if let Some(item) = self {
            item.into_value()
        } else {
            Ok(Value::Prim(PrimValue::default()))
        }
    }
}

impl<T: TomlInput> TomlInput for Vec<T> {
    fn schema() -> Result<Schema, Error> {
        let mut schema = T::schema()?;
        schema.set_wrap_type("Vec".to_string());
        schema.meta_mut().is_array = true;
        Ok(schema)
    }
    fn into_value(self) -> Result<Value, Error> {
        let schema = T::schema()?;
        let mut values = Vec::new();
        let mut as_prim = schema.is_prim();
        for item in self {
            let value = item.into_value()?;
            if as_prim || value.is_prim() || value.is_array() {
                as_prim = true;
                let prim = value.into_prim();
                values.push(Value::Prim(prim));
            } else {
                values.push(value)
            }
        }
        let array = ArrayValue { values };
        if as_prim {
            Ok(Value::Prim(array.into_prim()))
        } else {
            Ok(Value::Array(array))
        }
    }
}
