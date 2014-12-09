// A port of the simplistic benchmark from
//
//    http://github.com/PaulKeeble/ScalaVErlangAgents
//
// I *think* it's the same, more or less.

#![crate_name = "msgsend"]

extern crate time;
extern crate zmq;

use std::task::TaskBuilder;
use std::comm;
use std::os;

fn server(mut pull_socket: zmq::Socket, mut push_socket: zmq::Socket, mut workers: uint) {
    let mut count = 0u;
    let mut msg = zmq::Message::new().unwrap();

    while workers != 0 {
        match pull_socket.recv(&mut msg, 0) {
            Err(e) => panic!(e.to_string()),
            Ok(()) => {
                let s = msg.as_str().unwrap();
                if s.is_empty() {
                    workers -= 1;
                } else {
                    count += from_str::<uint>(s).unwrap();
                }
            }
        }
    }

    match push_socket.send_str(count.to_string().as_slice(), 0) {
        Ok(()) => { }
        Err(e) => panic!(e.to_string()),
    }
}

fn spawn_server(ctx: &mut zmq::Context, workers: uint) -> comm::Sender<()> {
    let mut pull_socket = ctx.socket(zmq::PULL).unwrap();
    let mut push_socket = ctx.socket(zmq::PUSH).unwrap();

    pull_socket.bind("inproc://server-pull").unwrap();
    push_socket.bind("inproc://server-push").unwrap();

    // Spawn the server.
    let (ready_tx, ready_rx) = comm::channel();
    let (start_tx, start_rx) = comm::channel();

    // Mutable sockets cannot be implicitly captured.
    let pull_socket = pull_socket;
    let push_socket = push_socket;

    TaskBuilder::new().spawn(proc() {
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

fn worker(mut push_socket: zmq::Socket, count: uint) {
    for _ in range(0, count) {
        push_socket.send_str(100u.to_string().as_slice(), 0).unwrap();
    }

    // Let the server know we're done.
    push_socket.send_str("", 0).unwrap();
}

fn spawn_worker(ctx: &mut zmq::Context, count: uint) -> comm::Receiver<()> {
    let mut push_socket = ctx.socket(zmq::PUSH).unwrap();

    push_socket.connect("inproc://server-pull").unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();

    // Mutable sockets cannot be implicitly captured.
    let push_socket = push_socket;

    // Spawn the worker.
    let (tx, rx) = comm::channel();
    TaskBuilder::new().spawn(proc() {
        // Let the main thread we're ready.
        tx.send(());

        worker(push_socket, count);

        tx.send(());
    });

    // Wait for the worker to start.
    rx.recv();

    rx
}

fn run(ctx: &mut zmq::Context, size: uint, workers: uint) {
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
    let result = match pull_socket.recv_msg(0) {
        Ok(msg) => from_str::<uint>(msg.as_str().unwrap()).unwrap(),
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
    } else if args.len() <= 1u {
        vec!("".to_string(), "10000".to_string(), "4".to_string())
    } else {
        args
    };

    let size: uint = from_str(args[1].as_slice()).unwrap();
    let workers: uint = from_str(args[2].as_slice()).unwrap();

    let mut ctx = zmq::Context::new();

    run(&mut ctx, size, workers);
}
