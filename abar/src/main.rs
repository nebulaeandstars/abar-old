mod config;
mod utils;

use clap::{load_yaml, App};

fn main() {
    let cli_settings = load_yaml!("cli.yml");
    let _ = App::from_yaml(cli_settings).get_matches();

    let mut status = config::bar();

    loop {
        std::process::Command::new("xsetroot")
            .arg("-name")
            .arg(status.to_string().as_str())
            .output()
            .unwrap();

        status.update();
        status.sleep();
    }
}
