use toml::Value;

use crate::{
    comment::{Comment, CommentType},
    error::Error,
    schema::{PrimaryType, TomlConfig, UnitEnum},
    section::Section,
    util, BANG_COMMENT,
};

#[derive(Debug, Clone)]
pub enum BlockValueSchema {
    UnitEnum(UnitEnum),
    Primary(PrimaryType),
}

#[derive(Debug, Clone)]
pub struct BlockSchema {
    pub ident: String,
    pub docs: String,
    pub hide: bool,
    pub value: BlockValueSchema,
    pub config: TomlConfig,
}

impl BlockSchema {
    pub fn into_blocks(self) -> Vec<Block> {
        let BlockSchema {
            ident,
            docs,
            value,
            hide,
            config: field_config,
        } = self;
        let mut block = Block::new(0, 0);
        block.ident = ident;
        block.hide = hide;
        let mut comment = Comment::default();
        comment.field_docs = docs;
        match value {
            BlockValueSchema::Primary(pt) => {
                comment.define_docs = pt.docs;
                comment.wrap_type = pt.wrap_type;
                comment.inner_type = pt.inner_type;
                comment.inner_default = pt.inner_default.clone();
                comment.type_ = CommentType::BlockField;
                block.comment = Some(comment);
                block.type_ = BlockType::FieldValue;
                block.value = pt.inner_default;
                return vec![block];
            }
            BlockValueSchema::UnitEnum(ut) => {
                let UnitEnum {
                    wrap_type,
                    inner_type,
                    inner_default,
                    docs,
                    variants,
                    config: _,
                } = ut;
                comment.define_docs = docs;
                comment.wrap_type = wrap_type;
                comment.inner_type = inner_type;
                comment.inner_default = inner_default;
                comment.type_ = CommentType::BlockVariant;
                let mut blocks = vec![];
                for variant in variants {
                    let mut block1 = block.clone();
                    let mut comment1 = comment.clone();
                    comment1.define_docs = variant.docs;
                    if comment1.inner_default.parse::<isize>().is_ok() {
                        let value = format!("{}", variant.value);
                        block1.hide = hide || comment.inner_default != value;
                        block1.value = value;
                    } else {
                        block1.hide = hide || comment.inner_default != variant.tag;
                        block1.value = variant.tag;
                    }
                    block1.comment = Some(comment1);
                    block1.type_ = BlockType::FieldVariant;
                    if !field_config.enum_expand {
                        block1.skip = true;
                    }
                    blocks.push(block1);
                }
                return blocks;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockMeta {
    pub key: String,
    pub schema: BlockSchema,
}

impl BlockMeta {
    pub fn into_blocks(self, id: usize, section_id: usize) -> Vec<Block> {
        let BlockMeta { key, schema } = self;
        let mut blocks = schema.into_blocks();
        for block in &mut blocks {
            block.id = id;
            block.section_id = section_id;
            block.key = key.clone();
        }
        blocks
    }
}

#[derive(Debug, Clone)]
pub enum BlockType {
    Unknown,
    FieldValue,
    FieldVariant,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: usize,
    pub section_id: usize,
    pub type_: BlockType,
    pub key: String,
    pub value: String,
    pub ident: String,
    pub comment: Option<Comment>,
    pub hide: bool,
    pub skip: bool,
    pub config: TomlConfig,
}

impl Block {
    pub fn new(id: usize, section_id: usize) -> Self {
        Block {
            id,
            section_id,
            key: String::new(),
            value: String::new(),
            ident: String::new(),
            type_: BlockType::Unknown,
            comment: None,
            hide: false,
            skip: false,
            config: TomlConfig::default(),
        }
    }

    pub fn map_key(&self) -> String {
        match self.type_ {
            BlockType::FieldVariant => format!("{}-{}", self.key, self.value),
            _ => format!("{}", self.key),
        }
    }

    pub fn render(&self) -> Result<String, Error> {
        if self.skip {
            return Ok(String::new())
        }
        let mut text = String::new();
        if let Some(comment) = &self.comment {
            text = comment.render()?;
            if !text.is_empty() {
                text = text + "\n";
            }
        }
        let mut block = format!("{} = {}", self.ident, self.value);
        if self.hide {
            block = util::prefix_lines(&format!("{block}"), BANG_COMMENT)
        }
        Ok(format!("{text}{block}"))
    }

    pub fn from_value(
        ident: String,
        key: String,
        value: Value,
        nest_stop: bool,
    ) -> Result<(Vec<Section>, Vec<Block>), Error> {
        match value {
            Value::Table(table) if !nest_stop => {
                let sections = Section::from_table(key.clone(), table, false)?;
                Ok((sections, Vec::new()))
            }
            Value::Array(array) if !nest_stop && array.iter().all(|value| value.is_table()) => {
                let mut sections = Vec::new();
                let nest_stop = true;
                for value in array {
                    if let Value::Table(table) = value {
                        let mut sub_sections = Section::from_table(key.clone(), table, nest_stop)?;
                        sections.append(&mut sub_sections);
                    } else {
                        unreachable!()
                    }
                }
                Ok((sections, Vec::new()))
            }
            _ => {
                let mut block = Block::new(0, 0);
                block.key = key;
                block.ident = ident;
                block.value = util::value_to_string(&value).unwrap();
                Ok((Vec::new(), vec![block]))
            }
        }
    }
}
