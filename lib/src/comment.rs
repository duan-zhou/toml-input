use crate::{PrimValue, config::TomlConfig, error::Error, util};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CommentType {
    #[default]
    None,
    Root,
    Section,
    BlockField,
    BlockVariant,
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
        let text = if !self.valued_docs.is_empty() {
            self.valued_docs.clone()
        } else {
            self.defined_docs.clone()
        };
        Ok(util::comment_lines(&text))
    }
}
