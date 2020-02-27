use arplib::*;
use hotpot_db::*;
use serde::{Deserialize, Serialize};
// use std::io;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    timestamp: i64,
    devices: Vec<(String, String)>,
}

fn get_ms_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as i64
}

fn track_devices() {
    let mut pot = HotPot::new();

    // lets make a new collection
    pot.create_collection("arp_history").unwrap();

    // the first value is the index of the network interface
    // the second value is an upper limit for IPs to scan - less is faster
    let res = process(2, 20);

    let entry = Entry {
        timestamp: get_ms_time(),
        devices: res,
    };

    pot.insert::<Entry>("arp_history", &entry).unwrap();
    println!("Last update: {}", entry.timestamp);
}

fn main() {
    println!("Starting tracker");

    loop {
        let _x = track_devices();
        thread::sleep(Duration::from_millis(1000 * 60 * 2));
    }
}
