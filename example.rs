use std;
use zmq;

import result::{ok, err};

import zmq::{context, socket, ToStr};

fn new_server(ctx: zmq::context, ch: comm::Chan<()>) {
    // FIXME: https://github.com/mozilla/rust/issues/2329.
    let socket = match ctx.socket(zmq::REP) {
      ok(socket) => socket.take(),
      err(e) => fail e.to_str(),
    };

    match socket.bind("tcp://127.0.0.1:3456") {
      ok(()) => { }
      err(e) => fail e.to_str()
    }

    // FIXME: https://github.com/mozilla/rust/issues/2329.
    let msg = match socket.recv_str(0) {
      ok(s) => s.take(),
      err(e) => fail e.to_str()
    };

    io::println(#fmt("received %s", msg));

    match socket.send_str(#fmt("hello %s", msg), 0) {
      ok(()) => { }
      err(e) => fail e.to_str()
    }

    // Let the main thread know we're done.
    ch.send(());
}

fn new_client(ctx: zmq::context) {
    io::println("starting client");

    // FIXME: https://github.com/mozilla/rust/issues/2329.
    let socket = match ctx.socket(zmq::REQ) {
      ok(socket) => socket.take(),
      err(e) => fail e.to_str(),
    };

    match socket.set_hwm(10u64) {
      ok(()) => { }
      err(e) => fail e.to_str()
    };

    match socket.get_hwm() {
      ok(hwm) => io::println(#fmt("hwm: %s", u64::str(hwm))),
      err(e) => fail e.to_str()
    }

    match socket.set_identity("identity") {
      ok(()) => { }
      err(e) => fail e.to_str()
    };

    match socket.get_identity() {
      ok(identity) =>
          io::println(#fmt("identity: %s", str::from_bytes(identity))),
      err(e) => fail e.to_str()
    };

    io::println("client connecting to server");

    match socket.connect("tcp://127.0.0.1:3456") {
      ok(()) => { }
      err(e) => fail e.to_str()
    };

    match socket.send_str("foo", 0) {
      ok(()) => { }
      err(e) => fail e.to_str()
    }

    match socket.recv_str(0) {
      ok(s) => io::println(s.take()),
      err(e) => fail e.to_str()
    }
}

fn main() {
    let (major, minor, patch) = zmq::version();

    io::println(#fmt("version: %d %d %d", major, minor, patch));

    let ctx = match zmq::init(1) {
      ok(ctx) => ctx.take(),
      err(e) => fail e.to_str()
    };

    // We need to start the server in a separate scheduler as it blocks.
    let po = comm::port();
    let ch = comm::chan(po);
    do task::spawn_sched(task::SingleThreaded) { new_server(ctx, ch) }

    new_client(ctx);

    // Wait for the server to shut down.
    po.recv();

    match ctx.term() {
      ok(()) => { }
      err(e) => fail e.to_str()
    };
}
