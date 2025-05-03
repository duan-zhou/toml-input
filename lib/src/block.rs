use crate::{
    comment::{Comment, CommentType},
    schema::{Meta, VariantSchema},
    util,
    value::BlockValue,
    Error, TomlValue, BANG_COMMENT, COMMENT,
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
        !self.variants.is_empty()
    }

    pub fn is_value(&self) -> bool {
        self.ident.is_empty()
    }

    pub fn is_field(&self) -> bool {
        !self.is_value()
    }

    pub fn enum_is_expand(&self) -> bool {
        if !self.is_enum() {
            return false;
        }
        let style = self.meta.config.enum_style.unwrap_or_default();
        style.can_expand(self.variants.len())
    }

    pub fn enum_is_fold(&self) -> bool {
        if !self.is_enum() {
            return false;
        }
        let style = self.meta.config.enum_style.unwrap_or_default();
        style.can_fold(self.variants.len())
    }

    pub fn render(&self) -> Result<String, Error> {
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
        let tag = block_value.tag;
        let text;
        if self.enum_is_expand() {
            text = self.render_enum_expand(commented, tag, raw_value)?;
        } else if self.enum_is_fold() {
            text = self.render_enum_fold(commented, tag, raw_value)?;
        } else if self.is_enum() {
            text = self.render_enum_single(commented, tag, raw_value)?;
        } else {
            text = self.render_single(commented, raw_value)?;
        }
        Ok(text)
    }

    fn render_enum_single(
        &self,
        commented: bool,
        tag: String,
        raw_value: TomlValue,
    ) -> Result<String, Error> {
        let mut lines = Vec::new();
        for variant in &self.variants {
            let comment = util::comment_lines(&variant.docs);
            if variant.value.tag == tag {
                let line = if commented {
                    format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
                } else {
                    format!("{} = {}", self.ident, raw_value)
                };
                lines.push(comment);
                lines.push(line);
                break;
            }
        }
        Ok(lines.join("\n"))
    }

    fn render_enum_expand(
        &self,
        commented: bool,
        tag: String,
        raw_value: TomlValue,
    ) -> Result<String, Error> {
        if !self.enum_is_expand() {
            panic!()
        }
        let mut lines = Vec::new();
        for variant in &self.variants {
            let comment = util::comment_lines(&variant.docs);
            if variant.value.tag == tag {
                let line = if commented {
                    format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
                } else {
                    format!("{} = {}", self.ident, raw_value)
                };
                if comment.len() > 0 {
                    lines.push(comment);
                }
                lines.push(line);
            } else if let Some(value) = &variant.value.raw {
                if comment.len() > 0 {
                    lines.push(comment);
                }
                let line = format!("{}{} = {}", BANG_COMMENT, self.ident, value);
                lines.push(line)
            }
        }
        Ok(lines.join("\n"))
    }

    fn render_enum_fold(
        &self,
        commented: bool,
        tag: String,
        raw_value: TomlValue,
    ) -> Result<String, Error> {
        if !self.enum_is_fold() {
            panic!()
        }
        let mut lines = Vec::new();
        let mut values = Vec::new();
        for variant in &self.variants {
            let comment = util::comment_lines(&variant.docs);
            if variant.value.tag == tag {
                let line = if commented {
                    format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
                } else {
                    format!("{} = {}", self.ident, raw_value)
                };
                lines.push(comment);
                lines.push(line);
            }
            if let Some(value) = &variant.value.raw {
                values.push(format!("{value}"))
            }
        }
        if values.len() > 1 {
            lines.insert(
                0,
                format!("{} {} = {}", COMMENT, self.ident, values.join(" | ")),
            );
        }
        Ok(lines.join("\n"))
    }

    fn render_single(&self, commented: bool, raw_value: TomlValue) -> Result<String, Error> {
        if self.is_enum() {
            panic!()
        }
        let mut lines = Vec::new();
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
