#![feature(destructuring_assignment)]

mod config;
mod utils;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use abar::{Command, StatusBar};
use spmc;

fn work(statusbar: Arc<Mutex<StatusBar>>) {
    let results_tx: mpsc::Sender<(usize, String)>;
    let jobs_rx: spmc::Receiver<(usize, Command)>;

    // temporarily gain control of the status bar to get a copy of the channels
    let sb = statusbar.lock().unwrap();
    (jobs_rx, results_tx) = sb.get_channels();
    drop(sb);

    loop {
        let (i, job) = jobs_rx.recv().unwrap();
        results_tx.send((i, (job)())).unwrap();
    }
}

fn main() {
    let statusbar = config::bar();

    let mut status = statusbar.to_string();
    let refresh_rate: Duration = statusbar.get_refresh_rate();

    let statusbar = Arc::new(Mutex::new(statusbar));

    for _ in 0..config::NUM_WORKERS {
        let sb = Arc::clone(&statusbar);
        thread::spawn(move || work(sb));
    }

    let mut new_status: String;
    loop {
        let mut sb = statusbar.lock().unwrap();
        sb.update_async();
        new_status = sb.to_string();
        drop(sb);

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
