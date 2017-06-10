#![crate_name = "rtreq"]

//! Router-to-request example

extern crate zmq;
extern crate rand;

use zmq::SNDMORE;
use rand::Rng;
use std::time::{Duration, Instant};
use std::thread;

// Inefficient but terse base16 encoder
fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<_>>()
        .join("")
}

fn worker_task() {
    let context = zmq::Context::new();
    let worker = context.socket(zmq::REQ).unwrap();
    let mut rng = rand::thread_rng();
    let identity: Vec<_> = (0..10).map(|_| rand::random::<u8>()).collect();
    worker.set_identity(&identity).unwrap();
    assert!(worker.connect("tcp://localhost:5671").is_ok());

    let mut total = 0;
    loop {
        // Tell the broker we're ready for work
        worker.send("Hi boss!", 0).unwrap();

        // Get workload from broker, until finished
        let workload = worker.recv_string(0).unwrap().unwrap();
        if workload == "Fired!" {
            println!("Worker {} completed {} tasks", hex(&identity), total);
            break;
        }
        total += 1;

        // Do some random work
        thread::sleep(Duration::from_millis(rng.gen_range(1, 500)));
    }
}


fn main() {
    let worker_pool_size = 10;
    let allowed_duration = Duration::new(5, 0);
    let context = zmq::Context::new();
    let broker = context.socket(zmq::ROUTER).unwrap();
    assert!(broker.bind("tcp://*:5671").is_ok());

    // While this example runs in a single process, that is just to make
    // it easier to start and stop the example. Each thread has its own
    // context and conceptually acts as a separate process.
    let mut thread_pool = Vec::new();
    for _ in 0..worker_pool_size {
        let child = thread::spawn(move || { worker_task(); });
        thread_pool.push(child);
    }

    // Run for five seconds and then tell workers to end
    let start_time = Instant::now();
    let mut workers_fired = 0;
    loop {
        // Next message gives us least recently used worker
        let identity = broker.recv_bytes(0).unwrap();
        broker.send(&identity, SNDMORE).unwrap();

        broker.recv_bytes(0).unwrap(); // Envelope
        broker.recv_bytes(0).unwrap(); // Response from worker
        broker.send("", SNDMORE).unwrap();

        // Encourage workers until it's time to fire them
        if start_time.elapsed() < allowed_duration {
            broker.send("Work harder", 0).unwrap();
        } else {
            broker.send("Fired!", 0).unwrap();
            workers_fired += 1;
            if workers_fired >= worker_pool_size {
                break;
            }
        }
    }
}
