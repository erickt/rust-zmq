// A port of the simplistic benchmark from
//
//    http://github.com/PaulKeeble/ScalaVErlangAgents
//
// I *think* it's the same, more or less.

use std;
use zmq;

//import io::{writer, writer_util};
import result::{ok, err};
//import comm::methods;
import dvec::{DVec, dvec}; //, extensions};
//import zmq::{socket_util, to_str};

fn server(ctx: zmq::context, ch: comm::Chan<()>, workers: uint) {
    let mut workers = workers;

    let pull_socket = match ctx.socket(zmq::PULL) {
      ok(socket) => socket.take(),
      err(e) => fail e.to_str(),
    };

    let push_socket = match ctx.socket(zmq::PUSH) {
      ok(socket) => socket.take(),
      err(e) => fail e.to_str(),
    };

    match pull_socket.bind("tcp://127.0.0.1:3456") {
      ok(()) => { }
      err(e) => fail e.to_str(),
    }

    match push_socket.bind("tcp://127.0.0.1:3457") {
      ok(()) => { }
      err(e) => fail e.to_str(),
    }

    // Let the main thread know we're ready.
    ch.send(());

    let mut count = 0u;
    while workers != 0 {
        let msg = match pull_socket.recv_str(0) {
          ok(msg) => msg.take(),
          err(e) => fail e.to_str(),
        };

        if msg == ~"" {
            workers -= 1;
        } else {
            count += uint::from_str(msg).get();
        }
    }

    match push_socket.send_str(uint::str(count), 0) {
      ok(()) => { }
      err(e) => fail e.to_str(),
    }

    pull_socket.close();
    push_socket.close();

    ch.send(());
}

fn worker(ctx: zmq::context, count: uint) {
    let push_socket = match ctx.socket(zmq::PUSH) {
      ok(socket) => socket.take(),
      err(e) => fail e.to_str(),
    };

    match push_socket.connect("tcp://127.0.0.1:3456") {
      ok(()) => { }
      err(e) => fail e.to_str(),
    }

    for count.times {
        match push_socket.send_str(uint::str(100u), 0) {
          ok(()) => { }
          err(e) => fail e.to_str(),
        }
    }

    // Let the server know we're done.
    match push_socket.send_str("", 0) {
      ok(()) => { }
      err(e) => fail e.to_str(),
    }

    push_socket.close();
}

fn run(ctx: zmq::context, size: uint, workers: uint) {
    // Spawn the server.
    let po = comm::port();
    let ch = comm::chan(po);
    do task::spawn_sched(task::SingleThreaded) {
        server(ctx, ch, workers);
    }

    // Wait for the server to start.
    po.recv();

    // Create some command/control sockets.
    let push_socket = match ctx.socket(zmq::PUSH) {
      ok(socket) => socket.take(),
      err(e) => fail e.to_str(),
    };

    match push_socket.connect("tcp://127.0.0.1:3456") {
      ok(()) => { }
      err(e) => fail e.to_str(),
    }

    let pull_socket = match ctx.socket(zmq::PULL) {
      ok(socket) => socket.take(),
      err(e) => fail e.to_str(),
    };

    match pull_socket.connect("tcp://127.0.0.1:3457") {
      ok(()) => { }
      err(e) => fail e.to_str(),
    }

    let start = std::time::precise_time_s();

    // Spawn all the workers.
    let worker_results: DVec<comm::Port<()>> = dvec();

    for workers.times {
        let po = comm::port();
        let ch = comm::chan(po);

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
      ok(()) => { }
      err(e) => fail e.to_str(),
    }
    */

    po.recv();

    // Receive the final count.
    let result = match pull_socket.recv_str(0) {
      ok(msg) => uint::from_str(msg.take()).get(),
      err(e) => fail e.to_str(),
    };

    let end = std::time::precise_time_s();
    let elapsed = end - start;

    io::println(#fmt("Count is %?", result));
    io::println(#fmt("Test took %? seconds", elapsed));
    let thruput = ((size / workers * workers) as float) / (elapsed as float);
    io::println(#fmt("Throughput=%f per sec", thruput));
}

fn main(args: ~[~str]) {
    let args = if os::getenv(~"RUST_BENCH").is_some() {
        ~[~"", ~"1000000", ~"10000"]
    } else if args.len() <= 1u {
        ~[~"", ~"10000", ~"4"]
    } else {
        copy args
    };

    let size = uint::from_str(args[1]).get();
    let workers = uint::from_str(args[2]).get();

    let ctx = match zmq::init(1) {
      ok(ctx) => ctx.take(),
      err(e) => fail e.to_str(),
    };

    run(ctx, size, workers);

    match ctx.term() {
      ok(()) => { }
      err(e) => fail e.to_str(),
    };
}
