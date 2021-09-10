mod config;
mod statusbar;
mod statusblock;

use std::io;
use std::io::Write;
use std::time::Duration;

use clap::{load_yaml, App};

fn main() {
    let cli_settings = load_yaml!("cli.yml");
    let cli = App::from_yaml(cli_settings);

    let mut status = config::bar();

    loop {
        status.update();
        println!("{}", status);

        io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_secs(1))
    }
}
