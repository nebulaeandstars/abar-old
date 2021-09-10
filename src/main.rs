mod statusblock;

use clap::{load_yaml, App};
use statusblock::StatusBlock;

fn main() {
    let cli_settings = load_yaml!("cli.yml");
    let cli_matches = App::from_yaml(cli_settings).get_matches();

    let test_block =
        StatusBlock::new("test", || "this is a test block".to_string());

    print!("{}: {}", test_block.get_name(), test_block.evaluate());
}
