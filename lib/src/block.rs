use crate::{
    comment::{Comment, CommentType},
    schema::{Meta, VariantSchema},
    util,
    value::BlockValue,
    Error, BANG_COMMENT,
};

#[derive(Debug, Clone, Default)]
pub struct Block {
    pub key: String,
    pub ident: String,
    pub meta: Meta,
    pub value: Option<BlockValue>,
    pub variants: Vec<VariantSchema>,
}

impl Block {
    pub fn is_enum(&self) -> bool {
        self.variants.len() > 0
    }

    pub fn is_value(&self) -> bool {
        self.ident.is_empty()
    }

    pub fn is_field(&self) -> bool {
        !self.is_value()
    }

    pub fn render(&self) -> Result<String, Error> {
        let mut lines = Vec::new();
        let mut block_value = self.meta.inner_default.clone().flatten();
        let mut commented = self.meta.config.block_comment;
        if let Some(value) = self.value.clone() {
            block_value = value;
            commented = false;
        }
        let raw_value;
        if let Some(raw) = block_value.value {
            raw_value = raw;
        } else {
            return Ok(String::new());
        }
        if self.is_enum() {
            for variant in &self.variants {
                let comment = util::comment_lines(&variant.docs);
                if variant.value.tag == block_value.tag {
                    let line = format!("{} = {}", self.ident, raw_value);
                    lines.push(comment);
                    lines.push(line);
                } else if self.meta.config.enum_expand {
                    if let Some(value) = &variant.value.raw {
                        lines.push(comment);
                        let line = format!("{}{} = {}", BANG_COMMENT, self.ident, value);
                        lines.push(line)
                    }
                }
            }
        } else {
            let comment = self.comment();
            let text = comment.render()?;
            if !text.is_empty() {
                lines.push(text);
            }
            let line = if commented {
                format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
            } else {
                format!("{} = {}", self.ident, raw_value)
            };
            lines.push(line);
        }
        Ok(lines.join("\n"))
    }

    pub fn comment(&self) -> Comment {
        let mut comment = self.meta.comment();
        if self.variants.is_empty() {
            comment.comment_type = CommentType::BlockField;
        } else {
            comment.comment_type = CommentType::BlockVariant;
        }
        comment
    }
}
