//! Pathological subscriber
//! Subscribes to one random topic and prints received messages

extern crate rand;
extern crate zmq;

use std::env;

use rand::distributions::{Distribution, Range};

fn main() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();

    let args: Vec<_> = env::args().collect();
    let address = if args.len() == 2 {
        args[1].as_str()
    } else {
        "tcp://localhost:5556"
    };
    subscriber
        .connect(&address)
        .expect("could not connect to publisher");

    let mut rng = rand::thread_rng();
    let topic_range = Range::new(0, 1000);
    let subscription = format!("{:03}", topic_range.sample(&mut rng)).into_bytes();
    subscriber.set_subscribe(&subscription).unwrap();

    loop {
        let topic = subscriber.recv_msg(0).unwrap();
        let data = subscriber.recv_msg(0).unwrap();
        assert_eq!(&topic[..], &subscription[..]);
        println!("{}", std::str::from_utf8(&data).unwrap());
    }
}
