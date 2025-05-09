use std::collections::HashMap;

use crate::{
    BANG_COMMENT, ROOT_KEY, Value,
    block::Block,
    comment::{Comment, CommentType},
    error::Error,
    schema::Meta,
    util,
};

#[derive(Debug, Clone)]
pub struct Section {
    pub key: String,
    pub meta: Meta,
    pub array_index: Option<usize>,
    pub blocks: Vec<Block>,
}

impl Default for Section {
    fn default() -> Self {
        Section {
            key: ROOT_KEY.to_string(),
            meta: Meta::default(),
            array_index: None,
            blocks: Vec::new(),
        }
    }
}

impl Section {
    pub fn is_root(&self) -> bool {
        self.key == ROOT_KEY && !self.blocks.is_empty()
    }
    pub fn is_value(&self) -> bool {
        self.key == ROOT_KEY && self.blocks.len() == 1 && self.blocks[0].is_value()
    }

    pub fn is_commented(&self) -> bool {
        let mut commented = self.meta.config.commented;
        for block in &self.blocks {
            commented = commented && block.is_comented();
        }
        commented
    }

    pub fn assigned_to(&mut self, ident: impl AsRef<str>) {
        if self.is_value() {
            for block in &mut self.blocks {
                block.key = ident.as_ref().to_string();
                block.ident = ident.as_ref().to_string();
            }
        } else {
            util::increase_key(&mut self.key, &ident);
            for block in &mut self.blocks {
                util::increase_key(&mut block.key, &ident);
            }
        }
    }

    pub fn reduce(sections: &mut Vec<Section>) {
        let mut map: HashMap<String, &mut Section> = HashMap::new();
        let mut dup = Vec::new();
        for (i, section) in sections.iter_mut().enumerate() {
            if let Some(s) = map.get_mut(&section.key) {
                s.blocks.append(&mut section.blocks);
                dup.push(i);
            } else {
                map.insert(section.key.clone(), section);
            }
        }
        for (i, index) in dup.iter().enumerate() {
            sections.remove(index - i);
        }
    }

    pub fn render(&self) -> Result<String, Error> {
        let comment = self.comment();
        let text = comment.render()?;
        let mut lines = Vec::new();
        if !text.is_empty() {
            lines.push(text);
        }
        let (left, right) = if self.is_root() {
            ("".to_string(), "".to_string())
        } else if self.meta.is_array {
            ("[[".to_string(), "]]".to_string())
        } else {
            ("[".to_string(), "]".to_string())
        };
        let bang = if self.is_commented() && (self.key != ROOT_KEY) {
            BANG_COMMENT
        } else {
            ""
        };
        lines.push(format!("{bang}{}{}{}", left, self.key, right));
        for block in &self.blocks {
            lines.push(block.render()?)
        }
        Ok(lines.join("\n"))
    }

    pub fn comment(&self) -> Comment {
        let mut comment = self.meta.comment();
        comment.comment_type = if self.is_root() {
            CommentType::Root
        } else {
            CommentType::Section
        };
        comment
    }
}

#[derive(Debug, Clone)]
pub struct TomlContent {
    pub sections: Vec<Section>,
}

impl TomlContent {
    pub fn merge_value(&mut self, value: Value) {
        let values = value.flatten();
        for value in &values {
            if value.array_index.is_some() {
                let section_key = util::key_parent(&value.key);
                let mut new_section = None;
                for section in &mut self.sections {
                    if section.key != section_key {
                        continue;
                    }
                    if section.array_index == value.array_index {
                        new_section = None;
                        break;
                    }
                    if new_section.is_none() {
                        let mut section = section.clone();
                        section.array_index = value.array_index;
                        new_section = Some(section)
                    }
                }
                if let Some(section) = new_section {
                    self.sections.push(section);
                }
            }
        }
        for value in values {
            'f0: for section in &mut self.sections {
                if section.array_index != value.array_index {
                    continue;
                }
                for block in &mut section.blocks {
                    if block.key == value.key && value.value.is_some() {
                        block.value = Some(value);
                        break 'f0;
                    }
                }
            }
        }
    }

    pub fn config_commented(&mut self, commented: bool) {
        for section in &mut self.sections {
            section.meta.config.commented = commented;
            for block in &mut section.blocks {
                block.meta.config.commented = commented;
            }
        }
    }

    pub fn render(&self) -> Result<String, Error> {
        let mut lines = Vec::new();
        for section in &self.sections {
            lines.push(section.render()?);
        }
        Ok(lines.join("\n\n"))
    }
}
