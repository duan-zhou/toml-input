use toml::{map::Map, Value as TomlValue};

use crate::util;
#[derive(Debug, Clone)]
pub struct BlockValue {
    pub key: String,
    pub tag: String,
    pub value: Option<TomlValue>,
    pub array_index: Option<usize>,
}

impl Default for BlockValue {
    fn default() -> Self {
        BlockValue {
            key: String::default(),
            tag: String::default(),
            value: None,
            array_index: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrimValue {
    pub tag: String,
    pub raw: Option<TomlValue>,
}

impl PrimValue {
    pub fn new(raw: TomlValue) -> Self {
        PrimValue {
            raw: Some(raw),
            ..Default::default()
        }
    }
    pub fn flatten(self) -> BlockValue {
        let PrimValue { tag, raw } = self;
        BlockValue {
            tag,
            value: raw,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ArrayValue {
    pub values: Vec<Value>,
}

impl ArrayValue {
    pub fn into_prim(self) -> PrimValue {
        let mut raws = Vec::new();
        for value in self.values {
            let PrimValue { raw, .. } = value.into_prim();
            if let Some(raw) = raw {
                raws.push(raw);
            }
        }
        PrimValue {
            tag: String::new(),
            raw: Some(TomlValue::Array(raws)),
        }
    }

    pub fn flatten(self) -> Vec<BlockValue> {
        let ArrayValue { values, .. } = self;
        let mut blocks = Vec::new();
        for (i, value) in values.into_iter().enumerate() {
            let tmp = value.flatten();
            for mut block in tmp {
                block.array_index = Some(i);
                blocks.push(block);
            }
        }
        blocks
    }
}

#[derive(Debug, Clone, Default)]
pub struct TableValue {
    pub fields: Vec<FieldValue>,
}

impl TableValue {
    pub fn into_prim(self) -> PrimValue {
        let mut map = Map::new();
        for field in self.fields {
            for (key, value) in field.into_map() {
                map.insert(key, value);
            }
        }
        PrimValue {
            tag: String::new(),
            raw: Some(TomlValue::Table(map)),
        }
    }

    pub fn flatten(self) -> Vec<BlockValue> {
        let TableValue { fields, .. } = self;
        let mut values = Vec::new();
        for field in fields {
            values.append(&mut field.flatten());
        }
        values
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Prim(PrimValue),
    Array(ArrayValue),
    Table(TableValue),
}

impl Default for Value {
    fn default() -> Self {
        Value::Prim(PrimValue::default())
    }
}

impl Value {
    pub fn new_prim(raw: TomlValue) -> Self {
        let prim = PrimValue::new(raw);
        Value::Prim(prim)
    }

    pub fn into_prim(self) -> PrimValue {
        match self {
            Value::Prim(prim) => prim,
            Value::Array(array) => array.into_prim(),
            Value::Table(table) => table.into_prim(),
        }
    }

    pub fn is_prim(&self) -> bool {
        matches!(self, Value::Prim(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn is_table(&self) -> bool {
        matches!(self, Value::Table(_))
    }

    pub fn flatten(self) -> Vec<BlockValue> {
        match self {
            Value::Prim(prim) => vec![prim.flatten()],
            Value::Array(array) => array.flatten(),
            Value::Table(table) => table.flatten(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FieldValue {
    pub ident: String,
    pub value: Value,
    pub flat: bool,
}

impl FieldValue {
    pub fn into_map(self) -> Map<String, TomlValue> {
        let FieldValue { ident, value, flat } = self;
        let PrimValue { raw, .. } = value.into_prim();
        let mut map = Map::new();
        if flat {
            if let Some(TomlValue::Table(map)) = raw {
                return map;
            }
        }
        if let Some(raw) = raw {
            map.insert(ident, raw);
        }
        map
    }
    pub fn flatten(self) -> Vec<BlockValue> {
        let FieldValue { ident, value, flat } = self;
        let mut blocks = value.flatten();
        if !flat {
            for block in &mut blocks {
                util::increase_key(&mut block.key, &ident);
            }
        }
        blocks
    }
}
