use std::fmt;

use crate::config;
use crate::statusblock::StatusBlock;

pub struct StatusBar {
    delimiter: String,
    blocks:    Vec<StatusBlock>,
}

impl StatusBar {
    pub fn new() -> Self {
        StatusBar {
            delimiter: config::delimiter().to_string(),
            blocks:    config::blocks(),
        }
    }

    pub fn get_blocks(&self) -> &Vec<StatusBlock> {
        &self.blocks
    }

    fn get_delimiter_for_index(&self, i: &usize) -> String {
        match (1..self.blocks.len()).contains(i) {
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
                    self.get_delimiter_for_index(&i),
                    block.evaluate(),
                )
                .as_str(),
            );
        }

        write!(f, "{}", out)
    }
}
