use std::time::Duration;
use std::{fmt, thread};

use crate::statusblock::StatusBlock;

pub struct StatusBar {
    delimiter:    String,
    blocks:       Vec<StatusBlock>,
    refresh_rate: Duration,
}

impl StatusBar {
    pub fn new() -> Self {
        StatusBar {
            delimiter:    String::new(),
            blocks:       Vec::new(),
            refresh_rate: Duration::from_secs(1),
        }
    }

    pub fn delimiter(mut self, delimiter: &str) -> Self {
        self.delimiter = delimiter.to_string();
        self
    }

    pub fn blocks(mut self, blocks: Vec<StatusBlock>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn refresh_rate(mut self, refresh_rate: Duration) -> Self {
        self.refresh_rate = refresh_rate;
        self
    }

    pub fn sleep(&self) {
        thread::sleep(self.refresh_rate)
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
