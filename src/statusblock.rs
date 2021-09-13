use std::time::{Duration, Instant};

pub struct StatusBlock {
    #[allow(dead_code)]
    name:          String,
    command:       Box<dyn Fn() -> String>,
    poll_interval: Option<Duration>,
    cache:         String,
    last_update:   Instant,
}

impl StatusBlock {
    pub fn new() -> Self {
        StatusBlock {
            name:          String::new(),
            command:       Box::new(|| "".to_string()),
            poll_interval: None,
            cache:         "".to_string(),
            last_update:   Instant::now(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn command(mut self, command: &'static dyn Fn() -> String) -> Self {
        self.command = Box::new(command);
        self.cache = (self.command)();
        self
    }

    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = Some(poll_interval);
        self
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn get_cache(&self) -> &String {
        &self.cache
    }

    /// Iff the StatusBlock needs to be updated, update it.
    pub fn update(&mut self) {
        match self.poll_interval {
            Some(interval) => {
                let now = Instant::now();
                if now.duration_since(self.last_update) >= interval {
                    self.cache = (self.command)();
                    self.last_update = now;
                }
            }
            None => (),
        };
    }
}
