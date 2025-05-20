use crate::{Error, Value, config::CommentStyle, section::Section, util};

#[derive(Debug, Clone)]
pub struct TomlContent {
    pub sections: Vec<Section>,
}

impl TomlContent {
    pub fn merge_value(&mut self, value: Value) {
        let values = value.flatten();
        for value in &values {
            if value.array_index.is_some() {
                let section_key = util::key_parent(&value.key);
                let mut new_section = None;
                for section in &mut self.sections {
                    if section.key != section_key {
                        continue;
                    }
                    if section.array_index == value.array_index {
                        new_section = None;
                        break;
                    }
                    if new_section.is_none() {
                        let mut section = section.clone();
                        section.array_index = value.array_index;
                        new_section = Some(section)
                    }
                }
                if let Some(section) = new_section {
                    self.sections.push(section);
                }
            }
        }
        for value in values {
            'f0: for section in &mut self.sections {
                if section.array_index != value.array_index {
                    continue;
                }
                for block in &mut section.blocks {
                    if block.key == value.key && value.value.is_some() {
                        block.value = Some(value);
                        break 'f0;
                    }
                }
            }
        }
    }

    pub fn config_commented(&mut self, commented: bool) {
        for section in &mut self.sections {
            section.meta.config.commented = commented;
            for block in &mut section.blocks {
                block.meta.config.commented = commented;
            }
        }
    }

    pub fn config_comment_style_hide(&mut self) {
        let style = CommentStyle::Hide;
        for section in &mut self.sections {
            section.meta.config.comment_style = Some(style);
            for block in &mut section.blocks {
                block.meta.config.comment_style = Some(style);
            }
        }
    }

    pub fn render(&self) -> Result<String, Error> {
        let mut lines = Vec::new();
        for section in &self.sections {
            let line = section.render()?;
            if !line.trim().is_empty() {
                lines.push(line);
            }
        }
        Ok(lines.join("\n\n").trim().to_string())
    }
}
