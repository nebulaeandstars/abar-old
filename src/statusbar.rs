use std::fmt;

use crate::statusblock::StatusBlock;

pub struct StatusBar {
    delimiter: String,
    blocks:    Vec<StatusBlock>,
}

impl StatusBar {
    pub fn new(delimiter: String, blocks: Vec<StatusBlock>) -> Self {
        StatusBar { delimiter, blocks }
    }

    pub fn update(&mut self) {
        for block in &mut self.blocks {
            block.update()
        }
    }

    fn get_delimiter_at_index(&self, i: usize) -> String {
        match i >= 1 {
            true => self.delimiter.clone(),
            false => String::new(),
        }
    }
}

impl fmt::Display for StatusBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        for (i, block) in self.blocks.iter().enumerate() {
            out.push_str(
                format!(
                    "{}{}",
                    self.get_delimiter_at_index(i),
                    block.get_cache(),
                )
                .as_str(),
            );
        }

        write!(f, "{}", out)
    }
}
