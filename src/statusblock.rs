use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Encapsulates a Fn() -> String closure.
///
/// Each StatusBlock has a unique name, some command that returns a string, and
/// a polling interval. The result of the command will be cached, and will be
/// updated iff the update() method is called *and* the time since the last
/// update is > the polling interval.
///
/// # Building
///
/// StatusBlocks follow the builder pattern for instantiation, so to make a new
/// one you might use something like this:
///
/// ```
/// let block = StatusBlock::new()
///     .name("example")
///     .command(&|| "hello".to_string())
///     .poll_interval(Duration::from_secs(5));
/// ```
pub struct StatusBlock {
    #[allow(dead_code)]
    name:          String,
    command:       Arc<dyn Fn() -> String + Send + Sync>,
    poll_interval: Option<Duration>,
    min_size:      Option<usize>,
    max_size:      Option<usize>,
    cache:         String,
    last_update:   Option<Instant>,
}

pub type Command = Arc<dyn Fn() -> String + Send + Sync>;

impl StatusBlock {
    /// Returns a new StatusBlock with default values. The defaults are:
    ///
    /// ```
    /// StatusBlock {
    ///     name:          String::new(),
    ///     command:       Box::new(|| String::new()),
    ///     poll_interval: None,
    ///     min_size:      None,
    ///     max_size:      None,
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            name:          String::new(),
            command:       Arc::new(String::new),
            poll_interval: None,
            min_size:      None,
            max_size:      None,
            cache:         String::new(),
            last_update:   None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn command(
        mut self,
        command: Arc<dyn Fn() -> String + Send + Sync>,
    ) -> Self {
        self.command = command;
        self
    }

    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = Some(poll_interval);
        self
    }

    pub fn min_size(mut self, min_size: usize) -> Self {
        self.min_size = Some(min_size);
        self
    }

    pub fn max_size(mut self, max_size: usize) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.min_size = Some(size);
        self.max_size = Some(size);
        self
    }

    pub fn needs_update(&self) -> bool {
        if let Some(last_update) = self.last_update {
            match self.poll_interval {
                Some(interval) =>
                    Instant::now().duration_since(last_update) >= interval,
                None => false,
            }
        } else {
            true
        }
    }

    /// Returns a reference to the name of the StatusBlock.
    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    /// Returns a reference to the StatusBlocks's cache.
    pub fn get_cache(&self) -> &String {
        &self.cache
    }

    pub fn get_command(&self) -> Command {
        Arc::clone(&self.command)
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Iff the StatusBlock needs to be updated, update it.
    pub fn update(&mut self) {
        if self.needs_update() {
            self.cache = (self.command)();
            self.last_update = Some(Instant::now());
        }
    }

    pub fn manual_update(&mut self, val: String) {
        self.cache = val;
        self.last_update = Some(Instant::now());
    }
}


impl fmt::Display for StatusBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = self.cache.to_string();

        if let Some(max) = self.max_size {
            out.truncate(max);
        }

        if let Some(min) = self.min_size {
            let diff = min - out.len();
            if diff > 0 {
                out.push_str(&" ".repeat(diff));
            }
        }

        write!(f, "{}", out)
    }
}

impl Default for StatusBlock {
    fn default() -> Self {
        Self::new()
    }
}
