use std::collections::HashMap;

use toml::Table;

use crate::{
    block::{Block, BlockMeta},
    comment::{Comment, CommentType},
    error::Error,
    util, BANG_COMMENT, ROOT_KEY, TAG,
};

#[derive(Debug, Clone)]
pub struct SectionSchema {
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: String,
    pub docs: String,
}

impl SectionSchema {
    pub fn into_comment(self) -> Comment {
        let SectionSchema {
            wrap_type,
            inner_type,
            inner_default,
            docs,
        } = self;
        Comment {
            define_docs: docs,
            field_docs: String::new(),
            wrap_type,
            inner_type,
            inner_default,
            type_: CommentType::Section,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SectionMeta {
    pub key: String,
    pub schema: SectionSchema,
    pub blocks: Vec<BlockMeta>,
    pub docs: String,
}

impl SectionMeta {
    pub fn into_section(self, id: usize) -> Section {
        let SectionMeta {
            key,
            schema,
            docs,
            blocks: blocks_meta,
        } = self;
        let mut comment = schema.into_comment();
        if key == ROOT_KEY {
            comment.type_ = CommentType::Root;
        }
        comment.field_docs = docs;
        let mut blocks = Vec::new();
        let section_id = id;
        for (i, block) in blocks_meta.into_iter().enumerate() {
            blocks.append(&mut block.into_blocks(i, section_id));
        }
        let type_ = if comment.wrap_type == "Vec" {
            SectionType::Array
        } else if key == ROOT_KEY {
            SectionType::Root
        } else {
            SectionType::Table
        };
        Section {
            id,
            key,
            comment: Some(comment),
            type_,
            hide: false,
            blocks,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SectionType {
    Root,
    Table,
    Array,
}

impl SectionType {
    pub fn tag(&self) -> (String, String) {
        match self {
            Self::Root => ("".to_string(), "".to_string()),
            Self::Table => ("[".to_string(), "]".to_string()),
            Self::Array => ("[[".to_string(), "]]".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub id: usize,
    pub key: String,
    pub blocks: Vec<Block>,
    pub type_: SectionType,
    pub comment: Option<Comment>,
    pub hide: bool,
}

impl Section {
    pub fn new(id: usize) -> Self {
        Section {
            id,
            key: String::new(),
            blocks: Vec::new(),
            type_: SectionType::Table,
            comment: None,
            hide: false,
        }
    }

    pub fn is_root(&self) -> bool {
        self.key == ROOT_KEY
    }


    pub fn is_empty_comment(&self) -> bool {
        match &self.comment {
            None => true,
            Some(comment) => comment.is_empty()
        }
    }

    // pub fn from_str(text: &str) -> Result<Vec<Section>, Error> {
    //     // for root comment
    //     let mut sections = vec![Section::new(0)];
    //     let mut section_key = String::new();
    //     let mut section_id = 0;
    //     let mut block_id = 0;
    //     for line in text.lines() {
    //         let n = line.trim().len();
    //         if line.starts_with("[") {
    //             section_id += 1;
    //             let off = if line.starts_with("[[") { 2 } else { 1 };
    //             section_key = line[off..(n - off)].to_string();
    //             let mut section = Section::new(section_id);
    //             section.key = section_key.clone();
    //             sections.push(section);
    //         } else if n > 0 {
    //             let off = line.find("=").expect("toml format bug");
    //             let ident = line[0..off].trim();
    //             let value = line[off + 1..].trim();
    //             let block_key = section_key.clone() + TAG + ident;
    //             let mut block = Block::new(block_id, section_id);
    //             block.key = block_key;
    //             block.ident = ident.to_string();
    //             block.value = value.to_string();
    //             let n = sections.len();
    //             let section = &mut sections[n];
    //             section.blocks.push(block);
    //             block_id += 1;
    //         }
    //     }
    //     Ok(sections)
    // }

    pub fn from_table(key: String, table: Table, nest_stop: bool) -> Result<Vec<Section>, Error> {
        let mut section = Section::new(0);
        section.key = key.clone();
        let mut sections = Vec::new();
        for (ident, sub_value) in table {
            let sub_key = format!("{key}{TAG}{ident}");
            let (mut sub_sections, mut sub_blocks) =
                Block::from_value(ident, sub_key, sub_value, nest_stop).unwrap();
            section.blocks.append(&mut sub_blocks);
            sections.append(&mut sub_sections);
        }
        sections.insert(0, section);
        Ok(sections)
    }

    pub fn merge_value(&mut self, other: Section) -> Result<(), Error> {
        let Section { blocks: source, .. } = other;
        let mut map: HashMap<_, _> = self
            .blocks
            .iter_mut()
            .map(|block| (block.key.clone(), block))
            .collect();
        for block in source {
            let dest = map.get_mut(&block.key).unwrap();
            dest.value = block.value;
        }
        Ok(())
    }

    pub fn render(&self) -> Result<String, Error> {
        let mut text = String::new();
        if let Some(comment) = &self.comment {
            text = comment.render()?;
        }
        if text.trim().len() > 0 {
            util::append_line(&mut text);
        }
        let (left, right) = self.type_.tag();
        let key = util::remove_prefix_tag(&self.key);
        text = format!("{text}{left}{key}{right}");
        for block in &self.blocks {
            text.extend(["\n".to_string(), block.render()?]);
        }
        if self.hide {
            text = util::prefix_lines(&text, BANG_COMMENT);
        }
        Ok(text)
    }
}
