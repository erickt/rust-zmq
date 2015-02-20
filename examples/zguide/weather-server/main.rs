#![crate_name = "weather-server"]

/// Weather update server
/// Binds PUB socket to tcp://*:5556 and ipc://weather.ipc
/// Publishes random weather updates

extern crate zmq;
extern crate rand;

use rand::Rng;

fn main() {
    let mut context = zmq::Context::new();
    let mut publisher = context.socket(zmq::PUB).ok().unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://weather.ipc").is_ok());

    let mut rng = rand::weak_rng();

    loop {
        let zipcode     = rng.gen_range(0is, 100000is);
        let temperature = rng.gen_range(-80is, 135is);
        let relhumidity = rng.gen_range(10is, 60is);

        // this is slower than C because the current format! implementation is
        // very, very slow. Several orders of magnitude slower than glibc's
        // sprintf
        let update = format!("{:05} {} {}", zipcode, temperature, relhumidity);
        publisher.send(update.as_bytes(), 0).ok().unwrap();
    }

    // note: destructors mean no explicit cleanup necessary
}
