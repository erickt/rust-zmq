// A port of the simplistic benchmark from
//
//    http://github.com/PaulKeeble/ScalaVErlangAgents
//
// I *think* it's the same, more or less.

use std;
use zmq;

import io::{writer, writer_util};
import result::{ok, err};
import comm::methods;
import dvec::{dvec, extensions};
import zmq::{socket_util, to_str};

fn server(ctx: zmq::context, ch: comm::chan<()>, workers: uint) {
    let mut workers = workers;

    let pull_socket = ctx.socket(zmq::PULL);
    if pull_socket.is_err() { fail pull_socket.get_err().to_str() };
    let pull_socket = result::unwrap(pull_socket);

    let push_socket = ctx.socket(zmq::PUSH);
    if push_socket.is_err() { fail push_socket.get_err().to_str() };
    let push_socket = result::unwrap(push_socket);

    alt pull_socket.bind("inproc://requests") {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    alt push_socket.bind("inproc://responses") {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    // Let the main thread know we're ready.
    ch.send(());

    let mut count = 0u;
    while workers != 0 {
        let msg = pull_socket.recv_str(0);
        if msg.is_err() { fail msg.get_err().to_str() }
        let msg = result::unwrap(msg);

        if msg == "" {
            workers -= 1;
        } else {
            count += uint::from_str(msg).get();
        }
    }

    alt push_socket.send_str(uint::str(count), 0) {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    pull_socket.close();
    push_socket.close();

    ch.send(());
}

fn worker(ctx: zmq::context, count: uint) {
    let push_socket = ctx.socket(zmq::PUSH);
    if push_socket.is_err() { fail push_socket.get_err().to_str() };
    let push_socket = result::unwrap(push_socket);

    alt push_socket.connect("inproc://requests") {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    for count.times {
        alt push_socket.send_str(uint::str(100u), 0) {
          ok(()) { }
          err(e) { fail e.to_str(); }
        }
    }

    // Let the server know we're done.
    alt push_socket.send_str("", 0) {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    push_socket.close();
}

fn run(ctx: zmq::context, size: uint, workers: uint) {
    // Spawn the server.
    let po = comm::port();
    let ch = comm::chan(po);
    do task::spawn_sched(task::single_threaded) {
        server(ctx, ch, workers);
    }

    // Wait for the server to start.
    po.recv();

    // Create some command/control sockets.
    let push_socket = ctx.socket(zmq::PUSH);
    if push_socket.is_err() { fail push_socket.get_err().to_str() };
    let push_socket = result::unwrap(push_socket);

    alt push_socket.connect("inproc://requests") {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    let pull_socket = ctx.socket(zmq::PULL);
    if pull_socket.is_err() { fail pull_socket.get_err().to_str() };
    let pull_socket = result::unwrap(pull_socket);

    alt pull_socket.connect("inproc://responses") {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    let start = std::time::precise_time_s();

    // Spawn all the workers.
    let worker_results: dvec<comm::port<()>> = dvec();

    for workers.times {
        let po = comm::port();
        let ch = comm::chan(po);

        worker_results.push(po);

        do task::spawn_sched(task::single_threaded) {
            worker(ctx, size / workers);
            ch.send(());
        }
    }

    // Block until all the workers finish.
    for worker_results.each |po| { po.recv(); }

    /*
    // Shut down the server.
    push_socket.send_str("stop", 0);
    alt push_socket.close() {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }
    */

    po.recv();

    // Receive the final count.
    let msg = pull_socket.recv_str(0);
    if msg.is_err() { fail msg.get_err().to_str(); }
    let msg = result::unwrap(msg);
    let result = uint::from_str(msg).get();

    let end = std::time::precise_time_s();
    let elapsed = end - start;

    io::println(#fmt("Count is %?", result));
    io::println(#fmt("Test took %? seconds", elapsed));
    let thruput = ((size / workers * workers) as float) / (elapsed as float);
    io::println(#fmt("Throughput=%f per sec", thruput));
}

fn main(args: ~[str]) {
    let args = if os::getenv("RUST_BENCH").is_some() {
        ~["", "1000000", "10000"]
    } else if args.len() <= 1u {
        ~["", "10000", "4"]
    } else {
        copy args
    };

    let size = uint::from_str(args[1]).get();
    let workers = uint::from_str(args[2]).get();

    let ctx = alt zmq::init(1) {
      ok(ctx) { ctx }
      err(e) { fail e.to_str() }
    };

    run(ctx, size, workers);

    alt ctx.term() {
      ok(()) { }
      err(e) { fail e.to_str() }
    };
}
