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
use std::uint;

fn server(ctx: zmq::Context, ch: &comm::Chan<()>, workers: uint) {
    let mut workers = workers;

    let mut pull_socket = match ctx.socket(zmq::PULL) {
        Ok(socket) => socket,
        Err(e) => fail!(e.to_str()),
    };

    let mut push_socket = match ctx.socket(zmq::PUSH) {
        Ok(socket) => socket,
        Err(e) => fail!(e.to_str()),
    };

    match pull_socket.bind("tcp://127.0.0.1:3456") {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }

    match push_socket.bind("tcp://127.0.0.1:3457") {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }

    // Let the main thread know we're ready.
    ch.send(());

    let mut count = 0u;
    while workers != 0 {
        match unsafe { pull_socket.recv(0) } {
            Err(e) => fail!(e.to_str()),
            Ok(msg) => {
                do msg.with_str |s| {
                    if s == "" {
                        workers -= 1;
                    } else {
                        count += uint::from_str(s).get();
                    }
                }
            }
        }
    }

    match push_socket.send_str(count.to_str(), 0) {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }

    pull_socket.close();
    push_socket.close();

    ch.send(());
}

fn worker(ctx: zmq::Context, count: uint) {
    let mut push_socket = match ctx.socket(zmq::PUSH) {
        Ok(socket) => socket,
        Err(e) => fail!(e.to_str()),
    };

    match push_socket.connect("tcp://127.0.0.1:3456") {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }

    for count.times {
        match push_socket.send_str(100u.to_str(), 0) {
          Ok(()) => { }
          Err(e) => fail!(e.to_str()),
        }
    }

    // Let the server know we're done.
    match push_socket.send_str("", 0) {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }

    push_socket.close();
}

fn run(ctx: zmq::Context, size: uint, workers: uint) {
    // Spawn the server.
    let (po, ch) = comm::stream();
    do task::spawn_sched(task::SingleThreaded) {
        server(ctx, &ch, workers);
    }

    // Wait for the server to start.
    po.recv();

    // Create some command/control sockets.
    let push_socket = match ctx.socket(zmq::PUSH) {
        Ok(socket) => socket,
        Err(e) => fail!(e.to_str()),
    };

    match push_socket.connect("tcp://127.0.0.1:3456") {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }

    let pull_socket = match ctx.socket(zmq::PULL) {
        Ok(socket) => socket,
        Err(e) => fail!(e.to_str()),
    };

    match pull_socket.connect("tcp://127.0.0.1:3457") {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }

    let start = extra::time::precise_time_s();

    // Spawn all the workers.
    let mut worker_results = ~[];

    for workers.times {
        let (po, ch) = comm::stream();

        worker_results.push(po);

        do task::spawn_sched(task::SingleThreaded) {
            worker(ctx, size / workers);
            ch.send(());
        }
    }

    // Block until all the workers finish.
    for worker_results.each |po| { po.recv(); }

    /*
    // Shut down the server.
    push_socket.send_str("stop", 0);
    match push_socket.close() {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    }
    */

    po.recv();

    // Receive the final count.
    let result = match unsafe { pull_socket.recv(0) } {
        Ok(msg) => msg.with_str(|s| uint::from_str(s).get()),
        Err(e) => fail!(e.to_str()),
    };

    let end = extra::time::precise_time_s();
    let elapsed = end - start;

    io::println(fmt!("Count is %?", result));
    io::println(fmt!("Test took %? seconds", elapsed));
    let thruput = ((size / workers * workers) as float) / (elapsed as float);
    io::println(fmt!("Throughput=%f per sec", thruput));
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

    let size = uint::from_str(args[1]).get();
    let workers = uint::from_str(args[2]).get();

    let ctx = match zmq::init(1) {
        Ok(ctx) => ctx,
        Err(e) => fail!(e.to_str()),
    };

    run(ctx, size, workers);

    match ctx.term() {
        Ok(()) => { }
        Err(e) => fail!(e.to_str()),
    };
}
