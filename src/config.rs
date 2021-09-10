use std::process::Command;

use crate::statusblock::StatusBlock;

/// Defines the delimiter that will be used to split status blocks.
pub const fn delimiter() -> &'static str {
    " | "
}

/// Defines the array of status blocks that will appear.
pub fn blocks() -> Vec<StatusBlock> {
    vec![
        StatusBlock::new("string_block", &|| {
            "Status blocks contain a closure that returns a String.".to_string()
        }),
        StatusBlock::new("function_block", &|| function_example()),
        StatusBlock::new("shell_block", &|| shell_example()),
    ]
}

fn function_example() -> String {
    "It's arguably neater to wrap that closure in a function.".to_string()
}

fn shell_example() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo You can also use the shell.")
        .output()
        .expect("Error in shell command!")
        .stdout;

    String::from_utf8(output).unwrap()
}
