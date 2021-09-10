use std::process::Command;
use std::time::Duration;

use crate::statusbar::StatusBar;
use crate::statusblock::StatusBlock;

/// Defines the delimiter that will be used to split up the output.
pub const fn delimiter() -> &'static str {
    " | "
}

/// Defines the array of status blocks that will appear. This is the thing that
/// you probably want to edit. A StatusBar is made up of a number of blocks,
/// which each have a unique name, a closure that returns a String, and an
/// optional update interval. If you haven't used Rust much before, I'd
/// recommend defining unique functions for each block.
pub fn bar() -> StatusBar {
    use crate::utils::run;

    // This is of many ways to define your status bar. These examples
    // demonstrate some of the things that you can do with `abar`, without
    // assuming much prior Rust knowledge.
    let blocks = vec![
        // You can wrap shell commands using the `run()` helper function.
        StatusBlock::new(
            "shell_wrapper",
            &|| run(r"echo $USER"),
            None, // doesn't update
        ),
        // Alternatively, combine rust with the shell like this.
        StatusBlock::new(
            "processes",
            &|| shell_example(),
            Some(Duration::from_secs(10)), // updates every 10 seconds
        ),
        // Or use Rust entirely by itself to make the fastest bar out there.
        StatusBlock::new(
            "time",
            &|| time_example(),
            Some(Duration::from_secs(5)), // updates every 5 seconds
        ),
        // Raw closures are always an option as well.
        StatusBlock::new("hello", &|| "Hello, bar!".to_string(), None),
    ];

    StatusBar::new(delimiter().to_string(), blocks)
}

/// Example showing how you can combine vanilla Rust with the shell. Displays
/// the number of running processes.
fn shell_example() -> String {
    // Run `sh` with the command as an argument. Not best practice but
    // definitely easier to read.
    let output = Command::new("sh")
        .arg("-c")
        // r"foo" is a raw string (no escape sequences)
        .arg(r"ps -A --no-headers | wc -l")
        .output()
        .expect("Error in shell command!")
        .stdout;

    // Convert the output into a String and remove trailing whitespace.
    let output = String::from_utf8(output).unwrap().trim().to_string();

    // The output can now be used as a regular string.
    format!("processes: {}", output)
}

/// One of the biggest perks of using Rust: the `cargo` package manager!
/// Additional dependencies can be defined in Cargo.toml
fn time_example() -> String {
    use chrono::{Timelike, Utc};

    // return a formatted string representing the current time.
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    format!(
        "{:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    )
}
