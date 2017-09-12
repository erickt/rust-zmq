#![crate_name = "taskvent"]

//! Task ventilator
//! Binds PUSH socket to tcp://localhost:5557
//! Sends batch of tasks to workers via that socket

extern crate zmq;
extern crate rand;

use std::io::{self, BufRead};
use rand::Rng;

fn main() {
    let context = zmq::Context::new();

    // Socket to send messages on
    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.bind("tcp://*:5557").is_ok());

    //  Socket to send start of batch message on
    let sink = context.socket(zmq::PUSH).unwrap();
    assert!(sink.connect("tcp://localhost:5558").is_ok());

    println!("Press Enter when the workers are ready: ");
    let stdin = io::stdin();
    stdin.lock().lines().next();

    println!("Sending tasks to workers...");
    //  The first message is "0" and signals start of batch
    sink.send("0", 0).unwrap();

    let mut rng = rand::thread_rng();

    // Send 100 tasks
    let mut total_msec: u32 = 0;
    for _ in 0..100 {
        //  Random workload from 1 to 100 msecs
        let workload: u32 = rng.gen_range(1, 101);

        total_msec += workload;

        let workload_str = format!("{}", workload);
        sender.send(&workload_str, 0).unwrap();
     }

    println!("Total expected cost: {} msec", total_msec)
}
