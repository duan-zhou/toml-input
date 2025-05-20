#[derive(Debug, Clone, PartialEq)]
pub struct TomlConfig {
    pub enum_style: Option<EnumStyle>,
    pub option_style: Option<OptionStyle>,
    pub commented: bool,
    pub comment_style: Option<CommentStyle>,
}

impl Default for TomlConfig {
    fn default() -> Self {
        TomlConfig {
            enum_style: None,
            option_style: None,
            commented: true,
            comment_style: None,
        }
    }
}

impl TomlConfig {
    pub fn merge_parent(&mut self, parent: &TomlConfig) {
        if self.enum_style.is_none() {
            self.enum_style = parent.enum_style;
        }
        if self.option_style.is_none() {
            self.option_style = parent.option_style;
        }
        if self.comment_style.is_none() {
            self.comment_style = parent.comment_style;
        }
    }

    pub fn is_none_skipped(&self) -> bool {
        if let Some(style) = self.option_style {
            style.is_skip_none()
        } else {
            false
        }
    }

    pub fn is_comment_hidden(&self) -> bool {
        matches!(self.comment_style, Some(CommentStyle::Hide))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnumStyle {
    Single,
    Expand,
    Fold,
    Flex(usize),
}

impl Default for EnumStyle {
    fn default() -> Self {
        EnumStyle::Flex(4)
    }
}

impl EnumStyle {
    pub fn can_expand(&self, variants_len: usize) -> bool {
        use EnumStyle::*;
        match self {
            Expand => true,
            Flex(limit) => variants_len <= *limit,
            _ => false,
        }
    }

    pub fn can_fold(&self, variants_len: usize) -> bool {
        use EnumStyle::*;
        match self {
            Fold => true,
            Flex(limit) => variants_len > *limit,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum OptionStyle {
    SkipNone,
    #[default]
    ExpandNone,
}

impl OptionStyle {
    pub fn is_skip_none(&self) -> bool {
        matches!(self, OptionStyle::SkipNone)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CommentStyle {
    #[default]
    Show,
    Hide,
}
