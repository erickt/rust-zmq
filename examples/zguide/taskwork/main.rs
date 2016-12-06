#![crate_name = "taskwork"]

/// Task worker
/// Connects PULL socket to tcp://localhost:5557
/// Collects workloads from ventilator via that socket
/// Connects PUSH socket to tcp://localhost:5558
/// Sends results to sink via that socket

extern crate zmq;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn atoi(s: &str) -> u64 {
    s.parse().unwrap()
}

fn main() {
    let context = zmq::Context::new();

    // socket to receive messages on
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    //  Socket to send messages to
    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect("tcp://localhost:5558").is_ok());

    loop {
        let string = receiver.recv_string(0).unwrap().unwrap();

        // Show progress
        print!(".");
        let _ = io::stdout().flush();

        // Do the work
        thread::sleep(Duration::from_millis(atoi(&string)));

        // Send results to sink
        sender.send("", 0).unwrap();
     }

}
