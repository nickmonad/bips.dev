#[macro_use]
extern crate lazy_static;

use crossbeam::channel;
use crossbeam::sync::WaitGroup;
use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::thread;
mod bip;

const WORKERS: usize = 10;
lazy_static! {
    static ref BIP_NUMBER: Regex =
        Regex::new(r"^bips/bip-([0-9]+)\.mediawiki$").expect("error parsing regex");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().collect::<io::Result<Vec<String>>>()?;
    let bips = lines
        .iter()
        .filter_map(|path| {
            // parse the bip number from the path
            // and map into tuple (number, path)
            BIP_NUMBER
                .captures(&path)?
                .get(1)?
                .as_str()
                .parse::<u32>()
                .map_or(None, |n| Some((n, path.clone())))
        })
        .collect::<Vec<(u32, String)>>();

    // Create a bounded channel for worker threads.
    let (tx, rx) = channel::bounded::<(u32, String)>(WORKERS);

    // Setup the worker pool.
    // This thread pushes work onto the "queue" (channel), and each worker thread reads
    // one job from the queue. All threads reference the same WaitGroup. This thread will
    // continue once all jobs have been pushed onto the queue, and the worker threads will
    // terminate once the channel has been marked as empty and disconnected.
    let wg = WaitGroup::new();

    // Spawn worker threads
    for _ in 0..WORKERS {
        let wg = wg.clone();
        let rx = rx.clone();

        thread::spawn(move || loop {
            match rx.recv() {
                Ok((number, path)) => match bip::process(number, path.as_str()) {
                    Err(e) => println!("error processing BIP {}: {:?}", number, e),
                    _ => {}
                },
                Err(_) => {
                    drop(wg);
                    break;
                }
            }
        });
    }

    for bip in bips {
        tx.send(bip).unwrap();
    }

    // Signal no more work
    drop(tx);
    // Wait for workers to finish in-flight jobs
    wg.wait();

    Ok(())
}
