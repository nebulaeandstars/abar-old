pub struct StatusBlock<T>
where
    T: Fn() -> String,
{
    name:    String,
    command: T,
}


impl<T> StatusBlock<T>
where
    T: Fn() -> String,
{
    pub fn new(name: &str, command: T) -> Self {
        StatusBlock {
            name:    name.to_string(),
            command: command,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn evaluate(&self) -> String {
        (self.command)()
    }
}
