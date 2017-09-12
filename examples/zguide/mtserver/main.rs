#![crate_name = "mtserver"]

extern crate zmq;
use std::thread;
use std::time::Duration;

fn worker_routine(context: &zmq::Context) {
    let receiver = context.socket(zmq::REP).unwrap();
    receiver
        .connect("inproc://workers")
        .expect("failed to connect worker");
    loop {
        receiver
            .recv_string(0)
            .expect("worker failed receiving")
            .unwrap();
        thread::sleep(Duration::from_millis(1000));
        receiver.send("World", 0).unwrap();
    }
}

fn main() {
    let context = zmq::Context::new();
    let clients = context.socket(zmq::ROUTER).unwrap();
    let workers = context.socket(zmq::DEALER).unwrap();

    clients
        .bind("tcp://*:5555")
        .expect("failed to bind client router");
    workers
        .bind("inproc://workers")
        .expect("failed to bind worker dealer");

    for _ in 0..5 {
        let ctx = context.clone();
        thread::spawn(move || worker_routine(&ctx));
    }
    zmq::proxy(&clients, &workers).expect("failed proxying");

}
