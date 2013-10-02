// A port of the simplistic benchmark from
//
//    http://github.com/PaulKeeble/ScalaVErlangAgents
//
// I *think* it's the same, more or less.

extern mod std;
extern mod extra;
extern mod zmq;

use std::comm;
use std::io;
use std::os;
use std::task;

fn server(pull_socket: zmq::Socket, push_socket: zmq::Socket, mut workers: uint) {
    let mut count = 0u;
    let mut msg = zmq::Message::new();

    while workers != 0 {
        match pull_socket.recv(&mut msg, 0) {
            Err(e) => fail!(e.to_str()),
            Ok(()) => {
                do msg.with_str |s| {
                    if s == "" {
                        workers -= 1;
                    } else {
                        count += from_str::<uint>(s).unwrap();
                    }
                }
            }
        }
    }

    match push_socket.send_str(count.to_str(), 0) {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }
}

fn spawn_server(ctx: &zmq::Context, workers: uint) -> comm::Chan<()> {
    let pull_socket = ctx.socket(zmq::PULL).unwrap();
    let push_socket = ctx.socket(zmq::PUSH).unwrap();

    pull_socket.bind("inproc://server-pull").unwrap();
    push_socket.bind("inproc://server-push").unwrap();
    //pull_socket.bind("tcp://127.0.0.1:3456").unwrap();
    //push_socket.bind("tcp://127.0.0.1:3457").unwrap();

    // Spawn the server.
    let (ready_po, ready_ch) = comm::stream();
    let (start_po, start_ch) = comm::stream();

    let mut task = task::task();
    task.sched_mode(task::SingleThreaded);
    do task.spawn_with((pull_socket, push_socket)) |(pull_socket, push_socket)| {
        // Let the main thread know we're ready.
        ready_ch.send(());

        // Wait until we need to start.
        start_po.recv();

        server(pull_socket, push_socket, workers);
    }

    // Wait for the server to start.
    ready_po.recv();

    start_ch
}

fn worker(push_socket: zmq::Socket, count: uint) {
    do count.times {
        push_socket.send_str(100u.to_str(), 0).unwrap();
    }

    // Let the server know we're done.
    push_socket.send_str("", 0).unwrap();
}

fn spawn_worker(ctx: &zmq::Context, count: uint) -> comm::Port<()> {
    let push_socket = ctx.socket(zmq::PUSH).unwrap();

    push_socket.connect("inproc://server-pull").unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();

    // Spawn the worker.
    let (po, ch) = comm::stream();
    let mut task = task::task();
    task.sched_mode(task::SingleThreaded);
    do task.spawn_with(push_socket) |push_socket| {
        // Let the main thread we're ready.
        ch.send(());

        worker(push_socket, count);

        ch.send(());
    }

    // Wait for the worker to start.
    po.recv();

    po
}

fn run(ctx: zmq::Context, size: uint, workers: uint) {
    let start_ch = spawn_server(&ctx, workers);

    // Create some command/control sockets.
    let push_socket = ctx.socket(zmq::PUSH).unwrap();
    let pull_socket = ctx.socket(zmq::PULL).unwrap();

    push_socket.connect("inproc://server-pull").unwrap();
    pull_socket.connect("inproc://server-push").unwrap();
    //push_socket.connect("tcp://127.0.0.1:3456").unwrap();
    //pull_socket.connect("tcp://127.0.0.1:3457").unwrap();

    // Spawn all the workers.
    let mut worker_results = ~[];
    for _ in range(0, workers) {
        worker_results.push(spawn_worker(&ctx, size / workers));
    }

    let start = extra::time::precise_time_s();

    start_ch.send(());

    // Block until all the workers finish.
    for po in worker_results.iter() {
        po.recv();
    }

    // Receive the final count.
    let result = match pull_socket.recv_msg(0) {
        Ok(msg) => msg.with_str(|s| from_str::<uint>(s).unwrap()),
        Err(e) => fail!(e.to_str()),
    };

    let end = extra::time::precise_time_s();
    let elapsed = end - start;

    io::println(format!("Count is {}", result));
    io::println(format!("Test took {} seconds", elapsed));
    let thruput = ((size / workers * workers) as float) / (elapsed as float);
    io::println(format!("Throughput={:f} per sec", thruput));
}

fn main() {
    let args = os::args();

    let args = if os::getenv("RUST_BENCH").is_some() {
        ~[~"", ~"1000000", ~"10000"]
    } else if args.len() <= 1u {
        ~[~"", ~"10000", ~"4"]
    } else {
        args
    };

    let size = from_str::<uint>(args[1]).unwrap();
    let workers = from_str::<uint>(args[2]).unwrap();

    let ctx = zmq::Context::new();

    run(ctx, size, workers);
}
