use serde::Serialize;
use strum_macros::{AsRefStr, EnumIter};
#[derive(EnumIter, AsRefStr, Debug, Serialize, Default)]
enum TestEnum {
    A,
    #[default]
    B,
    C2,
}

impl toml_input::TomlInput for TestEnum {
    fn schema() -> Result<toml_input::Schema, toml_input::Error> {
        use strum::IntoEnumIterator;
        use toml;
        use toml_input::config::EnumStyle;
        use toml_input::schema;
        let default = <TestEnum as Default>::default();
        let mut prim_schema = schema::PrimSchema::default();
        let mut meta = schema::Meta::default();
        meta.wrap_type = "".to_string();
        meta.inner_type = "TestEnum".to_string();
        let tag = default.as_ref().to_string();
        let raw = toml::Value::try_from(default)?;
        meta.inner_default = toml_input::PrimValue {
            tag,
            raw: Some(raw),
        };
        meta.defined_docs = " this is comment of enum".to_string();
        meta.config.enum_style = Some(EnumStyle::Expand);
        prim_schema.meta = meta;
        let mut variant_iter = TestEnum::iter();
        prim_schema.variants = Vec::new();
        let mut variant = schema::VariantSchema::default();
        variant.docs = "".to_string();
        let value = variant_iter.next().ok_or(toml_input::Error::EnumEmpty)?;
        let tag = std::convert::AsRef::as_ref(&value).to_string();
        let raw = toml::Value::try_from(value)?;
        let prim_value = toml_input::PrimValue {
            tag,
            raw: Some(raw),
        };
        variant.value = prim_value;
        variant.config.enum_style = Some(EnumStyle::Expand);
        prim_schema.variants.push(variant);
        let mut variant = schema::VariantSchema::default();
        variant.docs = "".to_string();
        let value = variant_iter.next().ok_or(toml_input::Error::EnumEmpty)?;
        let tag = std::convert::AsRef::as_ref(&value).to_string();
        let raw = toml::Value::try_from(value)?;
        let prim_value = toml_input::PrimValue {
            tag,
            raw: Some(raw),
        };
        variant.value = prim_value;
        variant.config.enum_style = Some(EnumStyle::Expand);
        prim_schema.variants.push(variant);
        let mut variant = schema::VariantSchema::default();
        variant.docs = "".to_string();
        let value = variant_iter.next().ok_or(toml_input::Error::EnumEmpty)?;
        let tag = std::convert::AsRef::as_ref(&value).to_string();
        let raw = toml::Value::try_from(value)?;
        let prim_value = toml_input::PrimValue {
            tag,
            raw: Some(raw),
        };
        variant.value = prim_value;
        variant.config.enum_style = Some(EnumStyle::Expand);
        prim_schema.variants.push(variant);
        Ok(schema::Schema::Prim(prim_schema))
    }
    fn into_value(self) -> Result<toml_input::Value, toml_input::Error> {
        let tag = self.as_ref().to_string();
        let raw = toml::Value::try_from(self)?;
        let prim = toml_input::PrimValue {
            tag,
            raw: Some(raw),
        };
        Ok(toml_input::Value::Prim(prim))
    }
}
