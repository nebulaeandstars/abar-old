use std::process::Command;
use std::time::Duration;

use abar::{StatusBar, StatusBlock};

/// This is the thing that you probably want to edit. A StatusBar is made up of
/// a number of blocks, each with a unique name, a closure that returns a
/// String, and an optional update interval. If you haven't used Rust much
/// before, I'd recommend copying the example syntax.
pub fn bar() -> StatusBar {
    use crate::utils::run;

    // You can use this wrapper to invoke shell commands.
    let run_example = StatusBlock::new()
        .name("run_example")
        .command(&|| run("echo hello"));

    // Alternatively, you can use the built-in interface,
    let shell_example = StatusBlock::new()
        .name("shell_example")
        .command(&|| shell_example())
        .poll_interval(Duration::from_secs(1));

    // or use vanilla Rust exclusively for the fastest bar out there.
    let vanilla_example = StatusBlock::new()
        .name("vanilla_example")
        .command(&|| time_example())
        .poll_interval(Duration::from_secs(5));

    // In case you were wondering how to use a closure:
    let closure_example = StatusBlock::new()
        .name("closure_example")
        .command(&|| {
            let output = "hello from a closure";
            output.to_string()
        })
        .poll_interval(Duration::from_secs(5));

    // I've defined all of the blocks as variables in advance, but feel free to
    // do whatever you want for your own bar. Make it yours.
    let blocks =
        vec![run_example, shell_example, closure_example, vanilla_example];

    // All fields are optional; default refresh rate is 1hz
    StatusBar::new()
        .blocks(blocks)
        .refresh_rate(Duration::from_millis(500))
        .delimiter(" | ")
        .left_buffer(" >>> ")
        .right_buffer(" <<< ")
}

/// Example showing how you can combine vanilla Rust with the shell. This
/// example displays the number of running processes.
fn shell_example() -> String {
    // this is essentially what the `run()` function looks like.
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

/// One of the biggest perks of using Rust is the `cargo` dependency manager.
/// This example uses the external `chrono` crate to display the current time as
/// GMT. Additional dependencies can be defined as-needed in Cargo.toml
fn time_example() -> String {
    use chrono::{Timelike, Utc};

    // return a formatted string representing the current time (GMT).
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