#![crate_id = "weather-server"]

/// Weather update server
/// Binds PUB socket to tcp://*:5556 and ipc://weather.ipc
/// Publishes random weather updates

extern crate zmq;

use std::rand::Rng;

fn main() {
    let mut context = zmq::Context::new();
    let mut publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://weather.ipc").is_ok());

    let mut rng = std::rand::weak_rng();

    loop {
        let zipcode     = rng.gen_range(0i, 100000i);
        let temperature = rng.gen_range(-80i, 135i);
        let relhumidity = rng.gen_range(10i, 60i);

        // this is slower than C because the current format! implementation is
        // very, very slow. Several orders of magnitude slower than glibc's
        // sprintf
        let update = format!("{:05d} {:d} {:d}", zipcode, temperature, relhumidity);
        publisher.send(update.as_bytes(), 0).unwrap();
    }

    // note: destructors mean no explicit cleanup necessary
}
