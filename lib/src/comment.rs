use crate::{error::Error, COMMENT};

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
    pub fn render(&self) -> Result<String, Error> {
        let text = if self.type_ == CommentType::BlockVariant || self.type_ == CommentType::Root {
            format!("{}{}", COMMENT, self.define_docs)
        } else {
            format!("{}{}", COMMENT, self.field_docs)
        };
        Ok(text)
    }
}
