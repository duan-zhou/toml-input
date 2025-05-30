use crate::{
    BANG_COMMENT, COMMENT, Error, TomlValue,
    comment::{Comment, CommentType},
    schema::{Meta, VariantSchema},
    util,
    value::BlockValue,
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

    pub fn is_comented(&self) -> bool {
        self.value.is_none() && self.meta.config.commented
    }

    pub fn is_none_skipped(&self) -> bool {
        self.meta.is_option_type() && self.meta.config.is_none_skipped() && self.value.is_none()
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
        if self.is_none_skipped() {
            return Ok(String::new());
        }
        let mut block_value = self.meta.inner_default.clone().flatten();
        let mut commented = self.meta.config.commented;
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
            if variant.value.tag != tag {
                continue;
            }
            let comment = util::comment_lines(&variant.docs);
            if !self.meta.config.is_comment_hidden() {
                lines.push(comment);
            }
            let line = if commented {
                format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
            } else {
                format!("{} = {}", self.ident, raw_value)
            };
            lines.push(line);
            break;
        }
        lines.retain(|line| !line.trim().is_empty());
        Ok(lines.join("\n"))
    }

    fn render_enum_expand(
        &self,
        commented: bool,
        tag: String,
        raw_value: TomlValue,
    ) -> Result<String, Error> {
        if !self.enum_is_expand() {
            return Err(Error::EnumStyleError("not enum_expand style".to_string()));
        }
        let mut lines = Vec::new();
        for variant in &self.variants {
            let comment = util::comment_lines(&variant.docs);
            if !self.meta.config.is_comment_hidden() {
                lines.push(comment);
            }
            if variant.value.tag == tag {
                let line = if commented {
                    format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
                } else {
                    format!("{} = {}", self.ident, raw_value)
                };
                lines.push(line);
            } else if let Some(value) = &variant.value.raw {
                let line = format!("{}{} = {}", BANG_COMMENT, self.ident, value);
                lines.push(line)
            }
        }
        lines.retain(|line| !line.trim().is_empty());
        Ok(lines.join("\n"))
    }

    fn render_enum_fold(
        &self,
        commented: bool,
        tag: String,
        raw_value: TomlValue,
    ) -> Result<String, Error> {
        if !self.enum_is_fold() {
            return Err(Error::EnumStyleError("not enum_fold style".to_string()));
        }
        let mut lines = Vec::new();
        let comment = self.comment();
        let text = comment.render()?;
        if !self.meta.config.is_comment_hidden() {
            lines.push(text);
        }
        let mut values = Vec::new();
        for variant in &self.variants {
            if variant.value.tag == tag {
                let line = if commented {
                    format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
                } else {
                    format!("{} = {}", self.ident, raw_value)
                };
                lines.push(line);
            }
            if let Some(value) = &variant.value.raw {
                values.push(format!("{value}"))
            }
        }
        if values.len() > 1 {
            lines.insert(
                1,
                format!("{} {} = {}", COMMENT, self.ident, values.join(" | ")),
            );
        }
        lines.retain(|line| !line.trim().is_empty());
        Ok(lines.join("\n"))
    }

    fn render_single(&self, commented: bool, raw_value: TomlValue) -> Result<String, Error> {
        if self.is_enum() {
            panic!()
        }
        let mut lines = Vec::new();
        let comment = self.comment();
        let text = comment.render()?;
        if !self.meta.config.is_comment_hidden() {
            lines.push(text);
        }
        let line = if commented {
            format!("{}{} = {}", BANG_COMMENT, self.ident, raw_value)
        } else {
            format!("{} = {}", self.ident, raw_value)
        };
        lines.push(line);
        lines.retain(|line| !line.trim().is_empty());
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
