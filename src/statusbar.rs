use std::sync::mpsc;
use std::time::Duration;
use std::{fmt, thread};

use spmc;

use crate::statusblock::{Command, StatusBlock};

/// Encapsulates a number of StatusBlocks.
///
/// Contains information re. how StatusBlocks should be formatted, delimited,
/// rendered, etc. as well as methods that operate across all blocks at once.
///
/// # Building
///
/// StatusBars follow the builder pattern for instantiation, so to make a new
/// one you might use something like this:
///
/// ```
/// let blocks: Vec<StatusBlock> = vec![];
///
/// let status = StatusBar::new()
///     .blocks(blocks)
///     .refresh_rate(Duration::from_millis(500))
///     .delimiter(" | ")
///     .left_buffer(" >>> ")
///     .left_buffer(" <<< ");
/// ```
pub struct StatusBar {
    delimiter:          String,
    blocks:             Vec<StatusBlock>,
    refresh_rate:       Duration,
    left_buffer:        String,
    right_buffer:       String,
    hide_empty_modules: bool,
    jobs_channel: (
        spmc::Sender<(usize, Command)>,
        spmc::Receiver<(usize, Command)>,
    ),
    results_channel: (
        mpsc::Sender<(usize, String)>,
        mpsc::Receiver<(usize, String)>,
    ),
}

impl StatusBar {
    /// Returns a new StatusBar with default values. The defaults are:
    ///
    /// ```
    /// StatusBar {
    ///     blocks:             Vec::new(),
    ///     refresh_rate:       Duration::from_secs(1),
    ///     delimiter:          String::new(),
    ///     left_buffer:        String::new(),
    ///     right_buffer:       String::new(),
    ///     hide_empty_modules: false,
    /// }
    /// ```
    pub fn new() -> Self {
        StatusBar {
            blocks:             Vec::new(),
            refresh_rate:       Duration::from_secs(1),
            delimiter:          String::new(),
            left_buffer:        String::new(),
            right_buffer:       String::new(),
            hide_empty_modules: false,

            jobs_channel:    spmc::channel(),
            results_channel: mpsc::channel(),
        }
    }

    pub fn blocks(mut self, blocks: Vec<StatusBlock>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn refresh_rate(mut self, refresh_rate: Duration) -> Self {
        self.refresh_rate = refresh_rate;
        self
    }

    pub fn delimiter(mut self, delimiter: &str) -> Self {
        self.delimiter = delimiter.to_string();
        self
    }

    pub fn left_buffer(mut self, left_buffer: &str) -> Self {
        self.left_buffer = left_buffer.to_string();
        self
    }

    pub fn right_buffer(mut self, right_buffer: &str) -> Self {
        self.right_buffer = right_buffer.to_string();
        self
    }

    pub fn hide_empty_modules(mut self, hide_empty_modules: bool) -> Self {
        self.hide_empty_modules = hide_empty_modules;
        self
    }

    /// Puts the current thread to sleep for an amount of time defined by the
    /// StatusBar's refresh_rate.
    pub fn sleep(&self) {
        thread::sleep(self.refresh_rate)
    }

    pub fn get_refresh_rate(&self) -> Duration {
        self.refresh_rate
    }

    pub fn get_channels(
        &self,
    ) -> (
        spmc::Receiver<(usize, Command)>,
        mpsc::Sender<(usize, String)>,
    ) {
        let (_, jobs_rx) = &self.jobs_channel;
        let (results_tx, _) = &self.results_channel;

        (jobs_rx.clone(), results_tx.clone())
    }

    /// Tells all blocks in the StatusBar to update themselves if needed.
    pub fn update(&mut self) {
        for block in &mut self.blocks {
            block.update()
        }
    }

    pub fn update_async(&mut self) {
        let (jobs_tx, _) = &mut self.jobs_channel;
        let (_, results_rx) = &self.results_channel;
        let mut num_jobs = 0;

        for (index, block) in &mut self.blocks.iter().enumerate() {
            if block.needs_update() {
                // println!("\"{}\" needs update", block.get_name());
                jobs_tx.send((index, block.get_command())).unwrap();
                num_jobs += 1;
            }
        }

        for _ in 0..num_jobs {
            match results_rx.try_recv() {
                Ok((index, value)) => {
                    // println!("\"{}\" updated",
                    // self.blocks[index].get_name());
                    self.blocks[index].manual_update(value);
                }
                Err(_) => break,
            }
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
            if !self.hide_empty_modules || !block.is_empty() {
                out.push_str(&format!(
                    "{}{}",
                    self.get_delimiter_at_index(i),
                    block,
                ));
            }
        }

        write!(f, "{}{}{}", self.left_buffer, out, self.right_buffer)
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}
