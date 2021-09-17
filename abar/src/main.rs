mod config;
mod utils;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut statusbar = config::bar();

    let mut status = statusbar.to_string();
    let refresh_rate: Duration = statusbar.get_refresh_rate();

    let statusbar = Arc::new(Mutex::new(statusbar));

    let sb = Arc::clone(&statusbar);
    thread::spawn(move || {
        let statusbar = sb;
        let refresh_rate = refresh_rate;

        loop {
            {
                let mut sb = statusbar.lock().unwrap();
                sb.update()
            }

            thread::sleep(refresh_rate);
        }
    });

    let mut new_status: String;
    loop {
        {
            let sb = statusbar.lock().unwrap();
            new_status = sb.to_string();
        }

        if status != new_status {
            std::process::Command::new("xsetroot")
                .arg("-name")
                .arg(new_status.as_str())
                .output()
                .unwrap();

            status = new_status;
        }

        thread::sleep(refresh_rate);
    }
}
