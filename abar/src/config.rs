use std::process::Command;
use std::time::Duration;

use abar::{StatusBar, StatusBlock};

/// The number of blocks that can update concurrently. Most people won't need to
/// change this, but bumping it up can cause a noticable difference in the
/// initial load time if you have a lot of blocks. A value of 1 will disable
/// concurrency, which is probably ok for most people.
pub const NUM_THREADS: u8 = 2;

/// Definition of the StatusBar
pub fn bar() -> StatusBar
{
    // All fields are optional; default refresh rate is 1hz
    StatusBar::new()
        .blocks(blocks())
        .refresh_rate(Duration::from_millis(10))
        .delimiter(" | ")
        .left_buffer(" >>> ")
        .right_buffer(" <<< ")
}

/// This is the thing that you probably want to edit. A StatusBar is made up of
/// a number of blocks, each with a unique name, a closure that returns a
/// String, and an optional update interval. If you haven't used Rust much
/// before, I'd recommend copying the example syntax.
fn blocks() -> Vec<StatusBlock>
{
    use crate::utils::run;

    // You can use this wrapper to invoke shell commands.
    let run_example = StatusBlock::new()
        .name("run_example")
        .command(|| run("echo hello"))
        .min_size(8);

    // Alternatively, you can use the built-in interface,
    let shell_example = StatusBlock::new()
        .name("shell_example")
        .command(|| shell_example())
        .poll_interval(Duration::from_secs(2));

    // or use vanilla Rust exclusively for the fastest bar out there.
    let vanilla_example = StatusBlock::new()
        .name("vanilla_example")
        .command(|| rand_example())
        .poll_interval(Duration::from_millis(10))
        .size(6);

    // Slow blocks can be offloaded to the background if using worker threads.
    let slow_example = StatusBlock::new()
        .name("slow_example")
        .command(|| slow_example())
        .poll_interval(Duration::from_secs(3))
        .size(12)
        .update_in_background(true); // try setting this to false

    // Finally, an example using a closure:
    let closure_example = StatusBlock::new()
        .name("closure_example")
        .command(|| {
            let output = "hello from a closure";
            output.to_string()
        })
        .max_size(18);

    vec![
        run_example,
        shell_example,
        closure_example,
        slow_example,
        vanilla_example,
    ]
}

/// Example showing how you can combine vanilla Rust with the shell. This
/// example displays the number of running processes.
fn shell_example() -> String
{
    // this is essentially what the `run()` function looks like.
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps -A --no-headers | wc -l")
        .output()
        .expect("Error in shell command!")
        .stdout;

    // Convert the output into a String and remove trailing whitespace.
    let output = String::from_utf8(output).unwrap().trim().to_string();

    // The output can now be used as a regular string.
    format!("processes: {}", output)
}

/// One of the biggest perks of using Rust is the `cargo` dependency manager.
/// This example uses the external `rand` crate to display random numbers.
/// Additional dependencies can be defined as-needed in Cargo.toml
fn rand_example() -> String
{
    use rand::random;

    format!("{}", random::<u16>())
}

/// This is very slow.
fn slow_example() -> String
{
    use std::thread;

    use rand::random;

    thread::sleep(Duration::from_secs(1));
    format!("slow: {}", random::<u16>())
}
