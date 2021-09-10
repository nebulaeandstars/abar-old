use std::time::{Duration, Instant};

pub struct StatusBlock {
    #[allow(dead_code)]
    name:          String,
    command:       Box<dyn Fn() -> String>,
    cache:         String,
    poll_interval: Option<Duration>,
    last_update:   Instant,
}

impl StatusBlock {
    pub fn new(
        name: &str,
        command: &'static dyn Fn() -> String,
        poll_interval: Option<Duration>,
    ) -> Self {
        StatusBlock {
            name:          name.to_string(),
            command:       Box::new(command),
            cache:         command(),
            poll_interval: poll_interval,
            last_update:   Instant::now(),
        }
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
