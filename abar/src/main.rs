mod config;
mod utils;

fn main() {
    let mut statusbar = config::bar();
    let mut status = statusbar.to_string();

    loop {
        let new_status = statusbar.to_string();

        if status != new_status {
            std::process::Command::new("xsetroot")
                .arg("-name")
                .arg(new_status.as_str())
                .output()
                .unwrap();

            status = new_status;
        }

        statusbar.update();
        statusbar.sleep();
    }
}
