// A port of the simplistic benchmark from
//
//    http://github.com/PaulKeeble/ScalaVErlangAgents
//
// I *think* it's the same, more or less.

#![crate_name = "msgsend"]

extern crate zmq;

use std::env;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::{channel, Sender, Receiver};

fn server(pull_socket: zmq::Socket, push_socket: zmq::Socket, mut workers: u64) {
    let mut count = 0;
    let mut msg = zmq::Message::new();

    while workers != 0 {
        pull_socket.recv(&mut msg, 0).unwrap();
        let s = msg.as_str().unwrap();
        if s.is_empty() {
            workers -= 1;
        } else {
            count += s.parse::<u32>().unwrap();
        }
    }

    push_socket.send(&count.to_string(), 0).unwrap();
}

fn spawn_server(ctx: &mut zmq::Context, workers: u64) -> Sender<()> {
    let pull_socket = ctx.socket(zmq::PULL).unwrap();
    let push_socket = ctx.socket(zmq::PUSH).unwrap();

    pull_socket.bind("inproc://server-pull").unwrap();
    push_socket.bind("inproc://server-push").unwrap();

    // Spawn the server.
    let (ready_tx, ready_rx) = channel();
    let (start_tx, start_rx) = channel();

    thread::spawn(move|| {
        // Let the main thread know we're ready.
        ready_tx.send(()).unwrap();

        // Wait until we need to start.
        start_rx.recv().unwrap();

        server(pull_socket, push_socket, workers);
    });

    // Wait for the server to start.
    ready_rx.recv().unwrap();

    start_tx
}

fn worker(push_socket: zmq::Socket, count: u64) {
    for _ in 0 .. count {
        push_socket.send(&100.to_string(), 0).unwrap();
    }

    // Let the server know we're done.
    push_socket.send("", 0).unwrap();
}

fn spawn_worker(ctx: &mut zmq::Context, count: u64) -> Receiver<()> {
    let push_socket = ctx.socket(zmq::PUSH).unwrap();

    push_socket.connect("inproc://server-pull").unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();

    // Spawn the worker.
    let (tx, rx) = channel();
    thread::spawn(move|| {
        // Let the main thread we're ready.
        tx.send(()).unwrap();

        worker(push_socket, count);

        tx.send(()).unwrap();
    });

    // Wait for the worker to start.
    rx.recv().unwrap();

    rx
}

fn seconds(d: &Duration) -> f64 {
    d.as_secs() as f64 + (d.subsec_nanos() as f64 / 1e9)
}

fn run(ctx: &mut zmq::Context, size: u64, workers: u64) {
    let start_ch = spawn_server(ctx, workers);

    // Create some command/control sockets.
    let push_socket = ctx.socket(zmq::PUSH).unwrap();
    let pull_socket = ctx.socket(zmq::PULL).unwrap();

    push_socket.connect("inproc://server-pull").unwrap();
    pull_socket.connect("inproc://server-push").unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();
    //pull_socket.connect("tcp://127.0.0.1:3457").unwrap();

    // Spawn all the workers.
    let mut worker_results = Vec::new();
    for _ in 0 .. workers {
        worker_results.push(spawn_worker(ctx, size / workers));
    }

    let start = Instant::now();

    start_ch.send(()).unwrap();

    // Block until all the workers finish.
    for po in worker_results {
        po.recv().unwrap();
    }

    // Receive the final count.
    let msg = pull_socket.recv_msg(0).unwrap();
    let result = msg.as_str().unwrap().parse::<i32>().unwrap();

    let elapsed = seconds(&start.elapsed());

    println!("Count is {}", result);
    println!("Test took {} seconds", elapsed);
    let thruput = ((size / workers * workers) as f64) / elapsed;
    println!("Throughput={} per sec", thruput);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = if env::var("RUST_BENCH").is_ok() {
        vec!("".to_string(), "1000000".to_string(), "10000".to_string())
    } else if args.len() <= 1 {
        vec!("".to_string(), "10000".to_string(), "4".to_string())
    } else {
        args
    };

    let size = args[1].parse().unwrap();
    let workers = args[2].parse().unwrap();

    let mut ctx = zmq::Context::new();

    run(&mut ctx, size, workers);
}
