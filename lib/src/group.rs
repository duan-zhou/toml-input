use std::collections::HashMap;

use toml::Value;

use crate::{block::{BlockMeta, BlockSchema, BlockValueSchema},  error::Error, schema::{Schema, Struct, StructField}, section::{Section, SectionMeta, SectionSchema}, ROOT_KEY};
use crate::TAG;

#[derive(Debug, Clone)]
pub struct SectionMetaGroup {
    pub sections: Vec<SectionMeta>,
}

impl SectionMetaGroup {
    pub fn into_section_group(self) -> SectionGroup {
        let sections: Vec<Section> = self
            .sections
            .into_iter()
            .enumerate()
            .map(|(i, section)| section.into_section(i))
            .collect();
        SectionGroup {sections}
    }
}

impl From<Struct> for SectionMetaGroup {
    fn from(value: Struct) -> Self {
        let Struct {
            wrap_type,
            inner_type,
            inner_default,
            docs,
            fields,
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
            docs: docs,
        };
        let meta = SectionMeta {
            key,
            schema,
            blocks,
            docs: String::new(),
        };
        sections.insert(0, meta);
        SectionMetaGroup { sections }
    }
}

fn meta_from_struct(st: Struct, key: String, flatten: bool) -> (Vec<SectionMeta>, Vec<BlockMeta>) {
    let Struct {
        wrap_type,
        inner_type,
        inner_default,
        docs,
        fields,
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
    } = field;
    let key = format!("{section_key}{TAG}{ident}");
    let fn_block_meta = |value: BlockValueSchema| {
        let schema = BlockSchema { ident, docs, value, hide: false };
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
        Schema::TupleEnum(tt) => {
            let value = BlockValueSchema::TupleEnum(tt);
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
pub struct SectionGroup {
    pub sections: Vec<Section>,
}

impl SectionGroup {
    pub fn render(&self) -> Result<String, Error> {
        let mut data = Vec::new();
        for section in &self.sections {
            data.push(section.render()?);
        }
        Ok(data.join("\n\n"))
    }
}


impl SectionGroup {
    pub fn from_value(value: Value) -> Result<SectionGroup, Error> {
        match value {
            Value::Table(table) => {
                let nest_stop = false;
                let sections = Section::from_table(ROOT_KEY.to_string(), table, nest_stop)?;
                Ok(SectionGroup { sections })
            }
            _ => panic!(),
        }
    }

    pub fn merge_value(&mut self, value: Value) -> Result<(), Error> {
        let mut map: HashMap<_, _> = self
            .sections
            .iter_mut()
            .map(|section| (section.key.clone(), (false, section)))
            .collect();
        let SectionGroup { sections: source } = SectionGroup::from_value(value).unwrap();
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
        self.sections.sort_by_key(|section| section.key.clone());
        Ok(())
    }
}