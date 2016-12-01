#![crate_name = "tasksink"]

/// Task sink
/// Binds PULL socket to tcp://localhost:5558
/// Collects results from workers via that socket

extern crate zmq;

use std::io::{self, Write};
use std::time::Instant;

fn main() {
    //  Prepare our context and socket
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.bind("tcp://*:5558").is_ok());

    // Wait for start of batch
    receiver.recv_bytes(0).unwrap();

    //  Start our clock now
    let start = Instant::now();

    for task_nbr in 0..100 {
        receiver.recv_bytes(0).unwrap();

        if task_nbr % 10 == 0 {
            print!(":");
        } else {
            print!(".");
        }
        let _ = io::stdout().flush();
    }

    println!("\nTotal elapsed time: {:?}", start.elapsed());
}
