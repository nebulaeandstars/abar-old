use clap::{load_yaml, App, AppSettings};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .set_term_width(0)
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DeriveDisplayOrder,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .get_matches();

    print!("Test");
}
