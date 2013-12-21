/// Weather update server
/// Binds PUB socket to tcp://*:5556 and ipc://weather.ipc
/// Publishes random weather updates

extern mod zmq;

use std::rand::random;

fn main() {
    let mut context = zmq::Context::new();
    let mut publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://weather.ipc").is_ok());

    loop {
        let zipcode     = random::<int>() % 100000;
        let temperature = (random::<int>() % 215) - 80;
        let relhumidity = (random::<int>() % 50) + 10;

        // this is slower than C because the current format! implementation is
        // very, very slow. Several orders of magnitude slower than glibc's
        // sprintf
        let update = format!("{:05d} {:d} {:d}", zipcode, temperature, relhumidity);
        publisher.send(update.as_bytes(), 0);
    }

    // note: destructors mean no explicit cleanup necessary
}
