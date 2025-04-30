#[derive(Debug, Clone, PartialEq)]
pub struct TomlConfig {
    pub enum_style: Option<EnumStyle>,
    pub block_comment: bool,
}

impl Default for TomlConfig {
    fn default() -> Self {
        TomlConfig {
            enum_style: None,
            block_comment: true,
        }
    }
}

impl TomlConfig {
    pub fn merge_parent(&mut self, parent: &TomlConfig) {
        if self.enum_style.is_none() {
            self.enum_style = parent.enum_style.clone();
        }
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
