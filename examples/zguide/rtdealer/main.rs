#![crate_name = "rtdealer"]

///  Router-to-dealer example

extern crate zmq;
extern crate rand;

use zmq::SNDMORE;
use rand::Rng;
use std::time::{Duration, Instant};
use std::thread;

fn worker_task() {
  let mut context = zmq::Context::new();
  let mut worker = context.socket(zmq::DEALER).unwrap();
  let mut rng = rand::thread_rng();
  let identity: String = (0..10).map(|_| rand::random::<u8>() as char).collect();
  worker.set_identity(identity.as_bytes()).unwrap();
  assert!(worker.connect("tcp://localhost:5671").is_ok());
  let mut total = 0;
  loop {
    // Tell the broker we're ready for work
    worker.send(b"", SNDMORE).unwrap();
    worker.send_str("Hi boss!", 0).unwrap();
    // Get workload from broker, until finished
    let mut envelope = zmq::Message::new().unwrap();
    let mut workload = zmq::Message::new().unwrap();
    worker.recv(&mut envelope, 0).unwrap(); // envelope delimiter
    worker.recv(&mut workload, 0).unwrap();
    let work = workload.as_str().unwrap();
    if work == "Fired!" {
      println!("Worker {} completed {} tasks", identity, total);
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
  let mut context = zmq::Context::new();
  let mut broker = context.socket(zmq::ROUTER).unwrap();
  assert!(broker.bind("tcp://*:5671").is_ok());

  // While this example runs in a single process, that is just to make
  // it easier to start and stop the example. Each thread has its own
  // context and conceptually acts as a separate process.
  let mut thread_pool = Vec::new();
  for _ in 0..worker_pool_size {
    let child = thread::spawn(move || {
      worker_task();
    });
    thread_pool.push(child);
  }

  // Run for five seconds and then tell workers to end
  let start_time = Instant::now();
  let mut workers_fired = 0;
  loop {
    // Next message gives us least recently used worker
    let mut identity = zmq::Message::new().unwrap();
    broker.recv(&mut identity, 0).unwrap();
    broker.send_msg(identity, SNDMORE).unwrap();

    let mut envelope = zmq::Message::new().unwrap();
    let mut workload = zmq::Message::new().unwrap();
    broker.recv(&mut envelope, 0).unwrap(); // Envelope
    broker.recv(&mut workload, 0).unwrap(); // Response from worker
    broker.send(b"", SNDMORE).unwrap();

    // Encourage workers until it's time to fire them
    if start_time.elapsed() < allowed_duration {
      broker.send_str("Work harder", 0).unwrap();
    } else {
      broker.send_str("Fired!", 0).unwrap();
      workers_fired += 1;
      if workers_fired >= worker_pool_size {
        break;
      }
    }
  }
}
