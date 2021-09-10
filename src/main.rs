mod statusblock;

use clap::{load_yaml, App, AppSettings};
use statusblock::StatusBlock;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml)
        .set_term_width(0)
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DeriveDisplayOrder,
            // AppSettings::SubcommandRequiredElseHelp,
        ])
        .get_matches();

    let test_block =
        StatusBlock::new("test", || "this is a test block".to_string());

    print!("{}: {}", test_block.get_name(), test_block.evaluate());
}
