#![crate_name = "tasksink"]

///  Task sink
///  Binds PULL socket to tcp://localhost:5558
///  Collects results from workers via that socket

extern crate zmq;
use std::time::Instant;

fn main() {
    //  Prepare our context and socket
    let mut context = zmq::Context::new();
    let mut receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.bind("tcp://*:5558").is_ok());
    
    // Wait for start of batch
    let mut msg = zmq::Message::new().unwrap();

    receiver.recv(&mut msg, 0).unwrap();

    //  Start our clock now
    let start = Instant::now();

    for i in 1..101 {
        receiver.recv(&mut msg, 0).unwrap();

        if i % 10 == 0 {
            print!(":");
        } else {
            print!(".");
        }
    }
    
    println!("\nTotal elapsed time: {:?}", start.elapsed());
}
