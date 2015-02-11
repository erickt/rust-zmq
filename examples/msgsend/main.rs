// A port of the simplistic benchmark from
//
//    http://github.com/PaulKeeble/ScalaVErlangAgents
//
// I *think* it's the same, more or less.

#![crate_name = "msgsend"]

extern crate time;
extern crate zmq;

use std::thread::Builder;
use std::sync::mpsc;
use std::os;

fn server(mut pull_socket: zmq::Socket, mut push_socket: zmq::Socket, mut workers: usize) {
    let mut count = 0us;
    let mut msg = zmq::Message::new().ok().unwrap();

    while workers != 0 {
        match pull_socket.recv(&mut msg, 0) {
            Err(e) => panic!(e.to_string()),
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
        Err(e) => panic!(e.to_string()),
    }
}

fn spawn_server(ctx: &mut zmq::Context, workers: usize) -> mpsc::Sender<()> {
    let mut pull_socket = ctx.socket(zmq::PULL).ok().unwrap();
    let mut push_socket = ctx.socket(zmq::PUSH).ok().unwrap();

    pull_socket.bind("inproc://server-pull").ok().unwrap();
    push_socket.bind("inproc://server-push").ok().unwrap();

    // Spawn the server.
    let (ready_tx, ready_rx) = mpsc::channel();
    let (start_tx, start_rx) = mpsc::channel();

    Builder::new().spawn(move|| {
        // Let the main thread know we're ready.
        ready_tx.send(());

        // Wait until we need to start.
        start_rx.recv();

        server(pull_socket, push_socket, workers);
    });

    // Wait for the server to start.
    ready_rx.recv();

    start_tx
}

fn worker(mut push_socket: zmq::Socket, count: usize) {
    for _ in range(0, count) {
        push_socket.send_str(100us.to_string().as_slice(), 0).ok().unwrap();
    }

    // Let the server know we're done.
    push_socket.send_str("", 0).ok().unwrap();
}

fn spawn_worker(ctx: &mut zmq::Context, count: usize) -> mpsc::Receiver<()> {
    let mut push_socket = ctx.socket(zmq::PUSH).ok().unwrap();

    push_socket.connect("inproc://server-pull").ok().unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();

    // Spawn the worker.
    let (tx, rx) = mpsc::channel();
    Builder::new().spawn(move|| {
        // Let the main thread we're ready.
        tx.send(());

        worker(push_socket, count);

        tx.send(());
    });

    // Wait for the worker to start.
    rx.recv();

    rx
}

fn run(ctx: &mut zmq::Context, size: usize, workers: usize) {
    let start_ch = spawn_server(ctx, workers);

    // Create some command/control sockets.
    let mut push_socket = ctx.socket(zmq::PUSH).ok().unwrap();
    let mut pull_socket = ctx.socket(zmq::PULL).ok().unwrap();

    push_socket.connect("inproc://server-pull").ok().unwrap();
    pull_socket.connect("inproc://server-push").ok().unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();
    //pull_socket.connect("tcp://127.0.0.1:3457").unwrap();

    // Spawn all the workers.
    let mut worker_results = Vec::new();
    for _ in range(0, workers) {
        worker_results.push(spawn_worker(ctx, size / workers));
    }

    let start = time::precise_time_s();

    start_ch.send(());

    // Block until all the workers finish.
    for po in worker_results.iter() {
        po.recv();
    }

    // Receive the final count.
    let result: usize = match pull_socket.recv_msg(0) {
        Ok(msg) => msg.as_str().unwrap().parse().unwrap(),
        Err(e) => panic!(e.to_string()),
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
    } else if args.len() <= 1us {
        vec!("".to_string(), "10000".to_string(), "4".to_string())
    } else {
        args
    };

    let size: usize = args[1].as_slice().parse().unwrap();
    let workers: usize = args[2].as_slice().parse().unwrap();

    let mut ctx = zmq::Context::new();

    run(&mut ctx, size, workers);
}
