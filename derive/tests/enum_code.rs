use serde::Serialize;
use strum_macros::{AsRefStr, EnumIter};
#[derive(EnumIter, AsRefStr, Debug, Serialize)]
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
impl toml_input::TomlInput for TestEnum {
    fn schema() -> Result<toml_input::Schema, toml_input::Error> {
        use strum::IntoEnumIterator;
        use toml;
        use toml_input::schema;
        let default = <TestEnum as Default>::default();
        let mut prim_schema = schema::PrimSchema::default();
        let mut meta = schema::Meta::default();
        meta.wrap_type = "".to_string();
        meta.inner_type = "TestEnum".to_string();
        let tag = default.as_ref().to_string();
        let raw = toml::Value::try_from(default).unwrap();
        meta.inner_default = toml_input::PrimValue { tag, raw: Some(raw) };
        meta.valued_docs = " this is comment of enum".to_string();
        meta.config.enum_expand = true;
        prim_schema.meta = meta;
        let mut variant_iter = TestEnum::iter();
        prim_schema.variants = Vec::new();
        let mut variant = schema::VariantSchema::default();
        variant.docs = "".to_string();
        let value = variant_iter.next().unwrap();
        let tag: &str = std::convert::AsRef::as_ref(&value);
        let prim_value = toml_input::PrimValue {
            tag: tag.into(),
            ..Default::default()
        };
        variant.value = prim_value;
        prim_schema.variants.push(variant);
        let mut variant = schema::VariantSchema::default();
        variant.docs = "".to_string();
        let value = variant_iter.next().unwrap();
        let tag: &str = std::convert::AsRef::as_ref(&value);
        let prim_value = toml_input::PrimValue {
            tag: tag.into(),
            ..Default::default()
        };
        variant.value = prim_value;
        prim_schema.variants.push(variant);
        let mut variant = schema::VariantSchema::default();
        variant.docs = "".to_string();
        let value = variant_iter.next().unwrap();
        let tag: &str = std::convert::AsRef::as_ref(&value);
        let prim_value = toml_input::PrimValue {
            tag: tag.into(),
            ..Default::default()
        };
        variant.value = prim_value;
        prim_schema.variants.push(variant);
        Ok(schema::Schema::Prim(prim_schema))
    }
    fn into_value(self) -> Result<toml_input::Value, toml_input::Error> {
        let tag = self.as_ref().to_string();
        let raw = toml::Value::try_from(self).unwrap();
        let prim = toml_input::PrimValue { tag, raw: Some(raw) };
        Ok(toml_input::Value::Prim(prim))
    }
}
