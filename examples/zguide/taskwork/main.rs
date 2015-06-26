#![crate_name = "taskwork"]

///  Task worker
///  Connects PULL socket to tcp://localhost:5557
///  Collects workloads from ventilator via that socket
///  Connects PUSH socket to tcp://localhost:5558
///  Sends results to sink via that socket

extern crate zmq;

use std::thread;
use std::io::{self,Write};

fn main() {
    let mut context = zmq::Context::new();

    // socket to receive messages on
    let mut receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    //  Socket to send messages to
    let mut sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect("tcp://localhost:5558").is_ok());

    let mut msg = zmq::Message::new().unwrap();

    loop {
        receiver.recv(&mut msg, 0).unwrap();

        let work: u8 =  msg.as_str().unwrap().bytes().last().unwrap();

        // Show progress
        print!(".");
        let _ = io::stdout().flush();

        // Do the work
        thread::sleep_ms(work as u32);

        // Send results to sink
        sender.send(b"",0).unwrap();
     }

}
