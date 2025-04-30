use serde::Serialize;

#[derive(Debug, Default, Serialize)]
/// this is comment of struct
struct TestStruct {
    /// this is comment of field
    a: i32,
    /// optional field
    b: Option<u32>,
}

impl toml_input::TomlInput for TestStruct {
    fn schema() -> Result<toml_input::Schema, toml_input::Error> {
        use std::str::FromStr;
        use toml;
        use toml_input::schema;
        let default = <TestStruct as Default>::default();
        let mut table = schema::TableSchema::default();
        let mut meta = schema::Meta::default();
        meta.wrap_type = "".to_string();
        meta.inner_type = "TestStruct".to_string();
        let raw = toml::Value::try_from(default)?;
        meta.inner_default = toml_input::PrimValue::new(raw);
        meta.defined_docs = " this is comment of struct".to_string();
        table.meta = meta;
        table.fields = Vec::new();
        let mut field = schema::FieldSchema::default();
        field.ident = "a".to_string();
        field.docs = " this is comment of field".to_string();
        field.flat = false;
        field.schema = <i32 as toml_input::TomlInput>::schema()?;
        table.fields.push(field);
        let mut field = schema::FieldSchema::default();
        field.ident = "b".to_string();
        field.docs = " optional field".to_string();
        field.flat = false;
        field.schema = <Option<u32> as toml_input::TomlInput>::schema()?;
        let value =
            u32::from_str("1").map_err(|err| toml_input::Error::FromStrError(err.to_string()))?;
        let raw = toml::Value::try_from(value)?;
        field.set_inner_default(raw);
        table.fields.push(field);
        Ok(schema::Schema::Table(table))
    }
    fn into_value(self) -> Result<toml_input::Value, toml_input::Error> {
        let mut table = toml_input::TableValue::default();
        let mut field = toml_input::FieldValue::default();
        field.ident = "a".to_string();
        field.flat = false;
        field.value = self.a.into_value()?;
        table.fields.push(field);
        let mut field = toml_input::FieldValue::default();
        field.ident = "b".to_string();
        field.flat = false;
        field.value = self.b.into_value()?;
        table.fields.push(field);
        Ok(toml_input::Value::Table(table))
    }
}
