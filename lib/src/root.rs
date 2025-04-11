use std::collections::HashMap;

use toml::Value;

use crate::schema::TomlConfig;
use crate::TAG;
use crate::{
    block::{BlockMeta, BlockSchema, BlockValueSchema},
    error::Error,
    schema::{Schema, Struct, StructField},
    section::{Section, SectionMeta, SectionSchema},
    ROOT_KEY,
};

#[derive(Debug, Clone)]
pub struct RootMeta {
    pub config: TomlConfig,
    pub sections: Vec<SectionMeta>,
}

impl RootMeta {
    pub fn into_root(self) -> Root {
        let RootMeta { config, sections } = self;
        let sections: Vec<Section> = sections
            .into_iter()
            .enumerate()
            .map(|(i, section)| section.into_section(i))
            .collect();
        Root { sections, config }
    }
}

impl From<Struct> for RootMeta {
    fn from(value: Struct) -> Self {
        let Struct {
            wrap_type,
            inner_type,
            inner_default,
            docs,
            fields,
            config,
        } = value;
        let key = ROOT_KEY.to_string();
        let mut sections = Vec::new();
        let mut blocks = Vec::new();
        for field in fields {
            let (mut sub_sections, mut sub_blocks) = meta_from_field(field, &key);
            sections.append(&mut sub_sections);
            blocks.append(&mut sub_blocks);
        }
        let schema = SectionSchema {
            wrap_type,
            inner_type,
            inner_default,
            docs,
            config: config.clone(),
        };
        let meta = SectionMeta {
            key,
            docs: String::new(),
            schema,
            blocks,
        };
        sections.insert(0, meta);
        RootMeta { sections, config }
    }
}

fn meta_from_struct(st: Struct, key: String, flatten: bool) -> (Vec<SectionMeta>, Vec<BlockMeta>) {
    let Struct {
        wrap_type,
        inner_type,
        inner_default,
        docs,
        fields,
        config,
    } = st;
    let mut sections = Vec::new();
    let mut blocks = Vec::new();
    for field in fields {
        let (mut sub_sections, mut sub_blocks) = meta_from_field(field, &key);
        sections.append(&mut sub_sections);
        blocks.append(&mut sub_blocks);
    }
    if flatten {
        return (sections, blocks);
    }
    let schema = SectionSchema {
        wrap_type,
        inner_type,
        inner_default,
        docs,
        config,
    };
    let meta = SectionMeta {
        key,
        docs: String::new(),
        schema,
        blocks,
    };
    sections.insert(0, meta);
    (sections, Vec::new())
}

fn meta_from_field(field: StructField, section_key: &str) -> (Vec<SectionMeta>, Vec<BlockMeta>) {
    let StructField {
        ident,
        docs,
        flatten,
        schema,
        config,
    } = field;
    let key = format!("{section_key}{TAG}{ident}");
    let fn_block_meta = |value: BlockValueSchema| {
        let schema = BlockSchema {
            ident,
            docs,
            value,
            hide: false,
            config,
        };
        BlockMeta {
            key: key.clone(),
            schema,
        }
    };
    let mut sections = Vec::new();
    let mut blocks = Vec::new();
    match schema {
        Schema::None => {}
        Schema::Primary(pt) => {
            let value = BlockValueSchema::Primary(pt);
            blocks.push(fn_block_meta(value));
        }
        Schema::UnitEnum(ut) => {
            let value = BlockValueSchema::UnitEnum(ut);
            blocks.push(fn_block_meta(value));
        }
        Schema::Struct(st) => {
            let (mut sub_sections, mut sub_blocks) = meta_from_struct(st, key, flatten);
            sections.append(&mut sub_sections);
            blocks.append(&mut sub_blocks);
        }
    }
    (sections, blocks)
}

#[derive(Debug, Clone)]
pub struct Root {
    pub sections: Vec<Section>,
    pub config: TomlConfig,
}

impl Root {
    pub fn hide_blocks(&mut self) {
        for section in &mut self.sections {
            for block in &mut section.blocks {
                block.hide = true
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

    pub fn from_value(value: Value) -> Result<Root, Error> {
        match value {
            Value::Table(table) => {
                let nest_stop = false;
                let sections = Section::from_table(ROOT_KEY.to_string(), table, nest_stop)?;
                Ok(Root {
                    sections,
                    config: TomlConfig::default(),
                })
            }
            _ => panic!(),
        }
    }

    pub fn merge_value(&mut self, value: Value) -> Result<(), Error> {
        self.hide_blocks();
        let mut map: HashMap<_, _> = self
            .sections
            .iter_mut()
            .map(|section| (section.key.clone(), (false, section)))
            .collect();
        let Root {
            sections: source, ..
        } = Root::from_value(value).unwrap();
        let mut added = Vec::new();
        for section in source {
            let (visited, dest) = map.get_mut(&section.key).unwrap();
            if !*visited {
                dest.merge_value(section).unwrap();
                *visited = true;
            } else {
                let mut dest = dest.clone();
                dest.merge_value(section).unwrap();
                added.push(dest);
            }
        }
        self.sections.append(&mut added);
        self.sections.sort_by_key(|section| section.id);
        Ok(())
    }
}
