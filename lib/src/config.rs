#[derive(Debug, Clone, PartialEq)]
pub struct TomlConfig {
    pub enum_expand: bool,
    pub block_comment: bool,
}

impl Default for TomlConfig {
    fn default() -> Self {
        TomlConfig {
            enum_expand: true,
            block_comment: true,
        }
    }
}

impl TomlConfig {
    pub fn merge_parent(&mut self, parent: &TomlConfig) {
        self.enum_expand = self.enum_expand && parent.enum_expand
    }
}
