#![crate_name = "msreader"]

// Reading from multiple sockets
// This version uses a simple recv loop

extern crate zmq;

use std::thread;

fn main() {
    let mut context = zmq::Context::new();
    
    // Connect to task ventilator
    let mut receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    // Connect to weather server
    let mut subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());
    let filter = "10001".to_string();
    assert!(subscriber.set_subscribe(filter.as_bytes()).is_ok());


    // Process messages from both sockets
    // We prioritize traffic from the task ventilator
    let mut msg = zmq::Message::new().unwrap();
    loop {

        loop {
            let resp = receiver.recv(&mut msg, zmq::DONTWAIT);
            match resp {
                Err(_) => { break },
                Ok(()) => {
                    // Process task
                }
            }
        }
        
        loop {
            let resp = subscriber.recv(&mut msg, zmq::DONTWAIT);
            match resp {
                Err(_) => { break },
                Ok(()) => {
                    // Process weather update
                }
            }
        }
        
        // No activity, so sleep for 1 msec
        thread::sleep_ms(1)
    }
}
