mod config;
mod statusbar;
mod statusblock;

use clap::{load_yaml, App};

use crate::statusbar::StatusBar;

fn main() {
    let cli_settings = load_yaml!("cli.yml");
    let cli = App::from_yaml(cli_settings);

    let status = StatusBar::new();

    print!("{}", status);
}
