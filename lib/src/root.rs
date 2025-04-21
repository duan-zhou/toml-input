use std::collections::HashMap;

use crate::block::Block;
use crate::schema::{PrimSchema, StructSchema};
use crate::value::{KeyValue, PrimValue};
use crate::{
    block::BlockMeta,
    error::Error,
    schema::{Schema, StructField},
    section::{Section, SectionMeta},
};
use crate::{util, TomlConfig, Value, TAG};

#[derive(Debug, Clone, Default)]
pub struct RootMeta {
    pub prim: Option<PrimSchema>,
    pub blocks: Vec<BlockMeta>,
    pub sections: Vec<SectionMeta>,
}

impl RootMeta {
    pub fn into_root(self) -> Root {
        let RootMeta {
            prim,
            blocks: meta_blocks,
            sections: meta_sections,
        } = self;
        let mut blocks = Vec::new();
        for (i, block) in meta_blocks.into_iter().enumerate() {
            blocks.append(&mut block.into_blocks(i, 0))
        }

        let mut sections = Vec::new();
        for (i, section) in meta_sections.into_iter().enumerate() {
            sections.push(section.into_section(i + 1))
        }
        Root {
            blocks,
            sections,
            ..Default::default()
        }
    }
}

impl From<Schema> for RootMeta {
    fn from(value: Schema) -> Self {
        match value {
            Schema::Prim(prim_schema) => prim_schema.into(),
            Schema::Struct(struct_schema) => struct_schema.into(),
        }
    }
}

impl From<PrimSchema> for RootMeta {
    fn from(value: PrimSchema) -> Self {
        RootMeta {
            prim: Some(value),
            ..Default::default()
        }
    }
}
impl From<StructSchema> for RootMeta {
    fn from(value: StructSchema) -> Self {
        let StructSchema { meta, fields } = value;
        let prim = PrimSchema {
            meta,
            ..Default::default()
        };
        let mut root = RootMeta {
            prim: Some(prim),
            ..Default::default()
        };
        for field in fields {
            let RootMeta {
                mut blocks,
                mut sections,
                ..
            } = field.into();
            root.blocks.append(&mut blocks);
            root.sections.append(&mut sections);
        }
        root
    }
}

impl From<StructField> for RootMeta {
    fn from(mut value: StructField) -> Self {
        if value.schema.is_prim() {
            let block = BlockMeta {
                field: value,
                ..Default::default()
            };
            return RootMeta {
                blocks: vec![block],
                ..Default::default()
            };
        }
        let schema = std::mem::replace(&mut value.schema, Schema::default());
        let RootMeta {
            prim,
            mut blocks,
            mut sections,
        } = schema.into();
        if let Some(prim) = prim {
            value.schema = Schema::Prim(prim);
            let block = BlockMeta {
                field: value,
                ..Default::default()
            };
            return RootMeta {
                blocks: vec![block],
                ..Default::default()
            };
        }
        if !value.flat {
            for block in &mut blocks {
                block.increase_key(&value.ident);
            }
            for section in &mut sections {
                section.increase_key(&value.ident);
            }
        }
        return RootMeta {
            blocks,
            sections,
            ..Default::default()
        };
    }
}

#[derive(Debug, Clone, Default)]
pub struct Root {
    pub prim: Option<PrimValue>,
    pub blocks: Vec<Block>,
    pub sections: Vec<Section>,
    pub config: TomlConfig,
}

impl Root {
    pub fn new_value(value: impl Into<String>) -> Self {
        Root {
            sections: vec![Section::new_value(0, value)],
            ..Default::default()
        }
    }
    pub fn hide_blocks(&mut self, hide: bool) {
        for section in &mut self.sections {
            section.hide = false;
            for block in &mut section.blocks {
                block.hide = hide
            }
        }
    }

    pub fn render(&self) -> Result<String, Error> {
        let mut data = Vec::new();
        for section in &self.sections {
            let text = section.render()?;
            if !text.is_empty() {
                data.push(text);
            }
        }
        Ok(data.join("\n\n"))
    }

    pub fn merge_value(&mut self, value: Value) -> Result<(), Error> {
        match self {
            Value::Prim(prim) => self.prim = Some(prim),
            Value::Table(table) => {
                for KeyValue {key, value} in table.flatten() {
                    for block in self.blocks {
                        if block.key == key {
                            block.value = value;
                        }
                    }
                }

            }
        }
        // if let Value::Prim(prim) = value {
        //     for block in &mut self.blocks  {
        //         if block.key == prim.key {
        //             block.value = util::value_to_string(&prim.value).unwrap();
        //             break
        //         }
        //     }
        // } else if let Value::Table()
        // self.hide_blocks(true);
        // let mut map: HashMap<_, _> = self
        //     .sections
        //     .iter_mut()
        //     .map(|section| (section.key.clone(), (false, section)))
        //     .collect();
        // let Root {
        //     sections: source, ..
        // } = Root::from_value(value).unwrap();
        // let mut added = Vec::new();
        // for section in source {
        //     dbg!(&section.key);
        //     let (visited, dest) = map.get_mut(&section.key).unwrap();
        //     if !*visited {
        //         dest.merge_value(section).unwrap();
        //         *visited = true;
        //     } else {
        //         let mut dest = dest.clone();
        //         dest.merge_value(section).unwrap();
        //         added.push(dest);
        //     }
        // }
        // self.sections.append(&mut added);
        // self.sections.sort_by_key(|section| section.id);
        Ok(())
    }
}
