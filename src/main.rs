mod config;
mod statusblock;

use std::env;

use clap::{load_yaml, App};

fn main() {
    let cli_settings = load_yaml!("cli.yml");
    let cli = App::from_yaml(cli_settings);

    let mut output = String::new();
    let blocks = config::blocks();

    for arg in env::args() {
        for block in &blocks {
            if block.get_name() == arg {
                output.push_str(
                    format!("{}{}", config::delim(), &block.evaluate())
                        .as_str(),
                );
            }
        }
    }

    print!("{}", output);
}
