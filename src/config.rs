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
/// which have a unique name, a closure that returns a String, and an optional
/// update interval. If you haven't used Rust much before, I'd recommend
/// copying the example syntax, defining unique functions for each block.
pub fn bar() -> StatusBar {
    let blocks = vec![
        // block that shows the number of processes (10 second update time)
        StatusBlock::new(
            "processes",
            &|| shell_example(),
            Some(Duration::from_secs(10)),
        ),
        // block that shows the current time (5 second update time)
        StatusBlock::new(
            "time",
            &|| time_example(),
            Some(Duration::from_secs(5)),
        ),
        // block that says hello demonstrating closures (never updates)
        StatusBlock::new("hello", &|| "Hello, bar!".to_string(), None),
    ];

    StatusBar::new(delimiter().to_string(), blocks)
}

/// Example showing how you can combine Rust with the shell.
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
