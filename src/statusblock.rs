pub struct StatusBlock {
    name:    String,
    command: Box<dyn Fn() -> String>,
}


impl StatusBlock {
    pub fn new(name: &str, command: &'static dyn Fn() -> String) -> Self {
        StatusBlock {
            name:    name.to_string(),
            command: Box::new(command),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn evaluate(&self) -> String {
        (self.command)()
    }
}
