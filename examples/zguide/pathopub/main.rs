// Pathological publisher
// Sends out 1,000 topics and then one random update per second
extern crate zmq;
extern crate rand;

use std::env;
use std::thread::sleep;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    let args: Vec<_> = env::args().collect();
    let address = if args.len() == 2 { args[1].as_str() } else { "tcp://*:5556" };
    publisher.bind(&address).expect("could not bind publisher socket");

    // Ensure subscriber connection has time to complete
    sleep(Duration::from_millis(1000));

    // Send out all 1,000 topic messages
    for topic_nbr in 0..1000  {
        publisher.send(&format!("{:03}", topic_nbr), zmq::SNDMORE).unwrap();
        publisher.send("Save Roger", 0).unwrap();
    }
    // Send one random update per second
    let mut rng = rand::thread_rng();
    let topic_range = Range::new(0, 1000);
    loop {
        sleep(Duration::from_millis(1000));
        publisher.send(&format!("{:03}", topic_range.ind_sample(&mut rng)), zmq::SNDMORE).unwrap();
        publisher.send("Off with his head!", 0).unwrap();
    }
}
