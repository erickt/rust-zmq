#![crate_name = "mtserver"]

use std::thread;
use std::time::Duration;

fn worker_routine(context: &zmq2::Context) {
    let receiver = context.socket(zmq2::REP).unwrap();
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
    let context = zmq2::Context::new();
    let clients = context.socket(zmq2::ROUTER).unwrap();
    let workers = context.socket(zmq2::DEALER).unwrap();

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
    zmq2::proxy(&clients, &workers).expect("failed proxying");
}
