mod config;
mod utils;

fn main() {
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
