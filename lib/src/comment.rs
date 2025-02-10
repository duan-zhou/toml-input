use crate::{error::Error, util};

#[derive(Debug, Clone, PartialEq)]
pub enum CommentType {
    None,
    Root,
    Section,
    BlockField,
    BlockVariant,
}

impl Default for CommentType {
    fn default() -> Self {
        CommentType::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct Comment {
    pub define_docs: String,
    pub field_docs: String,
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: String,
    pub type_: CommentType,
}

impl Comment {
    pub fn is_empty(&self) -> bool {
        self.define_docs.trim().is_empty() && self.field_docs.trim().is_empty()
    }

    pub fn render(&self) -> Result<String, Error> {
        let text = if self.type_ == CommentType::BlockVariant || self.type_ == CommentType::Root {
            self.define_docs.clone()
        } else {
            self.field_docs.clone()
        };
        Ok(util::comment_lines(&text))
    }
}
