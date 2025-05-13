use crate::TomlValue;
use crate::comment::Comment;
use crate::config::TomlConfig;
use crate::util;
use crate::value::PrimValue;
use crate::{block::Block, section::Section};

#[derive(Debug, Clone, Default)]
pub struct Meta {
    pub config: TomlConfig,
    pub defined_docs: String,
    pub valued_docs: String,
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: PrimValue,
    pub is_array: bool,
}

impl Meta {
    pub fn comment(&self) -> Comment {
        Comment {
            config: self.config.clone(),
            defined_docs: self.defined_docs.clone(),
            valued_docs: self.valued_docs.clone(),
            inner_type: self.inner_type.clone(),
            inner_default: self.inner_default.clone(),
            ..Default::default()
        }
    }

    pub fn is_option_type(&self) -> bool {
        self.wrap_type == "Option"
    }
}

#[derive(Debug, Clone, Default)]
pub struct VariantSchema {
    pub docs: String,
    pub value: PrimValue,
    pub config: TomlConfig,
}

#[derive(Debug, Clone, Default)]
pub struct PrimSchema {
    pub meta: Meta,
    pub variants: Vec<VariantSchema>,
}

impl PrimSchema {
    pub fn flatten(self) -> Section {
        let PrimSchema { meta, variants } = self;
        let array_index = if meta.is_array { Some(0) } else { None };
        let block = Block {
            meta,
            variants,
            ..Default::default()
        };
        Section {
            array_index,
            blocks: vec![block],
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TableSchema {
    pub meta: Meta,
    pub fields: Vec<FieldSchema>,
}

impl TableSchema {
    pub fn flatten(self) -> Vec<Section> {
        let TableSchema { meta, fields } = self;
        let mut sections = Vec::new();
        let table_skip_none = meta.config.option_style.is_skip_none();
        for field in fields {
            if (table_skip_none || field.config.option_style.is_skip_none())
                && field.schema.meta().is_option_type()
            {
                continue;
            }
            sections.append(&mut field.flatten());
        }
        Section::reduce(&mut sections);
        for section in &mut sections {
            if section.is_root() {
                section.meta = meta.clone();
                section.array_index = if meta.is_array { Some(0) } else { None };
            }
        }
        sections
    }
}

#[derive(Debug, Clone, Default)]
pub struct FieldSchema {
    pub ident: String,
    pub docs: String,
    pub flat: bool,
    pub schema: Schema,
    pub config: TomlConfig,
}

impl FieldSchema {
    pub fn flatten(self) -> Vec<Section> {
        let FieldSchema {
            ident,
            docs,
            flat,
            schema,
            config,
        } = self;
        let mut sections = schema.flatten();
        if !flat {
            for section in &mut sections {
                section.meta.valued_docs = docs.clone();
                section.meta.config = config.clone();
                if section.is_value() {
                    for block in &mut section.blocks {
                        block.meta.valued_docs = docs.clone();
                        block.meta.config.merge_parent(&config);
                        block.key = ident.clone();
                        block.ident = ident.clone();
                    }
                } else {
                    util::increase_key(&mut section.key, &ident);
                    for block in &mut section.blocks {
                        util::increase_key(&mut block.key, &ident);
                    }
                }
            }
        }
        sections
    }

    pub fn set_inner_default(&mut self, raw: TomlValue) {
        let meta = self.schema.meta_mut();
        meta.inner_default.raw = Some(raw);
    }
}

#[derive(Debug, Clone)]
pub enum Schema {
    Prim(PrimSchema),
    Table(TableSchema),
}

impl Default for Schema {
    fn default() -> Self {
        Schema::Prim(PrimSchema::default())
    }
}

impl Schema {
    pub fn flatten(self) -> Vec<Section> {
        match self {
            Schema::Prim(prim) => vec![prim.flatten()],
            Schema::Table(table) => table.flatten(),
        }
    }
    pub fn new_table() -> Schema {
        Schema::Table(TableSchema::default())
    }

    pub fn is_prim(&self) -> bool {
        matches!(&self, Schema::Prim(_))
    }

    pub fn is_table(&self) -> bool {
        matches!(&self, Schema::Table(_))
    }

    pub fn meta_mut(&mut self) -> &mut Meta {
        match self {
            Schema::Prim(schema) => &mut schema.meta,
            Schema::Table(schema) => &mut schema.meta,
        }
    }

    pub fn meta(&self) -> &Meta {
        match self {
            Schema::Prim(schema) => &schema.meta,
            Schema::Table(schema) => &schema.meta,
        }
    }

    pub fn set_wrap_type(&mut self, new: String) -> String {
        let meta = self.meta_mut();
        std::mem::replace(&mut meta.wrap_type, new)
    }
}
