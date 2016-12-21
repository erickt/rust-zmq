#![crate_name = "lvcache"]

extern crate zmq;

use std::str::from_utf8;
use std::collections::HashMap;

fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::SUB).unwrap();
    frontend.connect("tcp://localhost:5557").expect("could not connect to frontend");
    let backend = context.socket(zmq::XPUB).unwrap();
    backend.bind("tcp://*:5558").expect("could not bind backend socket");

    //  Subscribe to every single topic from publisher
    frontend.set_subscribe(b"").unwrap();

    let mut cache = HashMap::new();

    loop {
        let mut items = [
            frontend.as_poll_item(zmq::POLLIN),
            backend.as_poll_item(zmq::POLLIN),
        ];
        if zmq::poll(&mut items, 1000).is_err() {
            break;              //  Interrupted
        }
        if items[0].is_readable() {
            let topic = frontend.recv_msg(0).unwrap();
            let current = frontend.recv_msg(0).unwrap();
            cache.insert(topic.to_vec(), current.to_vec());
            backend.send(topic, zmq::SNDMORE).unwrap();
            backend.send(current, 0).unwrap();
        }
        if items[1].is_readable() {
            // Event is one byte 0=unsub or 1=sub, followed by topic
            let event = backend.recv_msg(0).unwrap();
            if event[0] == 1 {
                let topic = &event[1..];
                println!("Sending cached topic {}", from_utf8(topic).unwrap());
                if let Some(previous) = cache.get(topic) {
                    backend.send(topic, zmq::SNDMORE).unwrap();
                    backend.send(previous, 0).unwrap();
                }
            }
        }
    }
}
