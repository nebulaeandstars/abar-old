#![feature(destructuring_assignment)]

mod config;
mod utils;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use abar::Command;
use spmc;

fn work(
    jobs_rx: spmc::Receiver<(usize, Command)>,
    results_tx: mpsc::Sender<(usize, String)>,
) {
    // listen for a new job, then send back the result.
    loop {
        let (i, job) = jobs_rx.recv().unwrap();
        results_tx.send((i, (job)())).unwrap();
    }
}

fn main() {
    let statusbar = config::bar();

    // get bar parameters now, before it gets locked behind the mutex.
    let mut status = statusbar.to_string();
    let refresh_rate: Duration = statusbar.get_refresh_rate();
    let (jobs_rx, results_tx) = statusbar.get_channels();

    // protect the status bar to make it thread-safe.
    let statusbar = Arc::new(Mutex::new(statusbar));

    // if there's supposed to be more than one worker thread, spawn them.
    for _ in 1..config::NUM_THREADS {
        let jobs_rx = jobs_rx.clone();
        let results_tx = results_tx.clone();
        thread::spawn(move || work(jobs_rx, results_tx));
    }

    let mut new_status: String;
    loop {
        let mut sb = statusbar.lock().unwrap();

        // if there are no worker threads, clear all pending asyncronous jobs
        if config::NUM_THREADS <= 1 {
            loop {
                if let Ok((i, job)) = jobs_rx.try_recv() {
                    results_tx.send((i, (job)())).unwrap();
                } else {
                    break;
                }
            }
        }

        sb.update();

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
