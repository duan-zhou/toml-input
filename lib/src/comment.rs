use crate::{config::TomlConfig, error::Error, util, PrimValue};

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
    pub defined_docs: String,
    pub valued_docs: String,
    pub wrap_type: String,
    pub inner_type: String,
    pub inner_default: PrimValue,
    pub comment_type: CommentType,
    pub config: TomlConfig,
}

impl Comment {
    pub fn is_empty(&self) -> bool {
        self.defined_docs.trim().is_empty() && self.valued_docs.trim().is_empty()
    }

    pub fn render(&self) -> Result<String, Error> {
        let text = if self.valued_docs.len() > 0 {
            self.valued_docs.clone()
        } else {
            self.defined_docs.clone()
        };
        Ok(util::comment_lines(&text))
    }
}
