// A port of the simplistic benchmark from
//
//    http://github.com/PaulKeeble/ScalaVErlangAgents
//
// I *think* it's the same, more or less.

#![crate_name = "msgsend"]

#![feature(core)]

extern crate time;
extern crate zmq;

use std::os;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

fn server(mut pull_socket: zmq::Socket, mut push_socket: zmq::Socket, mut workers: u64) {
    let mut count = 0;
    let mut msg = zmq::Message::new().unwrap();

    while workers != 0 {
        match pull_socket.recv(&mut msg, 0) {
            Err(e) => panic!(e),
            Ok(()) => {
                let s = msg.as_str().unwrap();
                if s.is_empty() {
                    workers -= 1;
                } else {
                    count += s.parse().unwrap();
                }
            }
        }
    }

    match push_socket.send_str(count.to_string().as_slice(), 0) {
        Ok(()) => { }
        Err(e) => panic!(e),
    }
}

fn spawn_server(ctx: &mut zmq::Context, workers: u64) -> Sender<()> {
    let mut pull_socket = ctx.socket(zmq::PULL).unwrap();
    let mut push_socket = ctx.socket(zmq::PUSH).unwrap();

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

fn worker(mut push_socket: zmq::Socket, count: u64) {
    for _ in 0 .. count {
        push_socket.send_str(100.to_string().as_slice(), 0).unwrap();
    }

    // Let the server know we're done.
    push_socket.send_str("", 0).unwrap();
}

fn spawn_worker(ctx: &mut zmq::Context, count: u64) -> Receiver<()> {
    let mut push_socket = ctx.socket(zmq::PUSH).unwrap();

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

fn run(ctx: &mut zmq::Context, size: u64, workers: u64) {
    let start_ch = spawn_server(ctx, workers);

    // Create some command/control sockets.
    let mut push_socket = ctx.socket(zmq::PUSH).unwrap();
    let mut pull_socket = ctx.socket(zmq::PULL).unwrap();

    push_socket.connect("inproc://server-pull").unwrap();
    pull_socket.connect("inproc://server-push").unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();
    //pull_socket.connect("tcp://127.0.0.1:3457").unwrap();

    // Spawn all the workers.
    let mut worker_results = Vec::new();
    for _ in 0 .. workers {
        worker_results.push(spawn_worker(ctx, size / workers));
    }

    let start = time::precise_time_s();

    start_ch.send(()).unwrap();

    // Block until all the workers finish.
    for po in worker_results {
        po.recv().unwrap();
    }

    // Receive the final count.
    let result: i32 = match pull_socket.recv_msg(0) {
        Ok(msg) => {
            let msg_str: &str = msg.as_str().unwrap();
            msg_str.parse().unwrap()
        },
        Err(e) => panic!(e),
    };

    let end = time::precise_time_s();
    let elapsed = end - start;

    println!("Count is {}", result);
    println!("Test took {} seconds", elapsed);
    let thruput = ((size / workers * workers) as f64) / elapsed;
    println!("Throughput={} per sec", thruput);
}

fn main() {
    let args = os::args();

    let args = if os::getenv("RUST_BENCH").is_some() {
        vec!("".to_string(), "1000000".to_string(), "10000".to_string())
    } else if args.len() <= 1 {
        vec!("".to_string(), "10000".to_string(), "4".to_string())
    } else {
        args
    };

    let size = args[1].as_slice().parse().unwrap();
    let workers = args[2].as_slice().parse().unwrap();

    let mut ctx = zmq::Context::new();

    run(&mut ctx, size, workers);
}
