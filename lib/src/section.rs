use std::collections::HashMap;

use crate::{
    BANG_COMMENT, ROOT_KEY,
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

    pub fn is_none_skipped(&self) -> bool {
        let mut skipped = self.meta.is_option_type() && self.meta.config.is_none_skipped();
        for block in &self.blocks {
            skipped = skipped && block.is_none_skipped();
        }
        skipped
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
        if self.is_none_skipped() {
            return Ok(String::new());
        }
        let comment = self.comment();
        let text = comment.render()?;
        let mut lines = Vec::new();
        if !self.meta.config.is_comment_hidden() {
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
            let line = block.render()?;
            if !line.is_empty() {
                lines.push(line);
            }
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
