use std;
use zmq;

import zmq::{context, socket, socket_util, error};
import result::{ok, err};
import std::io;

fn new_server(&&ctx: zmq::context) {
    let socket = alt ctx.socket(zmq::REP) {
      ok(socket) { socket }
      err(e) { fail e.to_str() }
    };

    alt socket.bind_str("tcp://127.0.0.1:3456") {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    let msg = alt socket.recv_str(0) {
      ok(s) { s}
      err(e) { fail e.to_str() }
    };

    io::println(#fmt("received %s", msg));

    alt socket.send_str(#fmt("hello %s", msg), 0) {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    alt socket.close() {
      ok(()) { }
      err(e) { fail e.to_str() }
    };
}

fn new_client(&&ctx: zmq::context) {
    let socket = alt ctx.socket(zmq::REQ) {
      ok(socket) { socket }
      err(e) { fail e.to_str() }
    };

    alt socket.set_hwm(10u64) {
      ok(()) { }
      err(e) { fail e.to_str(); }
    };

    alt socket.get_hwm() {
      ok(hwm) { io::println(#fmt("hwm: %s", u64::str(hwm))); }
      err(e) { fail e.to_str(); }
    }

    alt socket.set_identity(str::bytes("identity")) {
      ok(()) { }
      err(e) { fail e.to_str(); }
    };

    alt socket.get_identity() {
      ok(identity) {
          io::println(#fmt("hwm: %s",
                      unsafe { str::unsafe::from_bytes(identity) }))
      }
      err(e) { fail e.to_str() }
    };

    alt socket.connect_str("tcp://127.0.0.1:3456") {
      ok(()) { }
      err(e) { fail e.to_str() }
    };

    alt socket.send_str("foo", 0) {
      ok(()) { }
      err(e) { fail e.to_str(); }
    }

    alt socket.recv_str(0) {
      ok(s) { io::println(s); }
      err(e) { fail e.to_str(); }
    }

    alt socket.close() {
      ok(()) { }
      err(e) { fail e.to_str() }
    };
}

fn main() {
    let (major, minor, patch) = zmq::version();

    io::println(#fmt("version: %d %d %d", major, minor, patch));

    let ctx = alt zmq::init(1) {
      ok(ctx) { ctx }
      err(e) { fail e.to_str() }
    };

    let builder = task::mk_task_builder();
    let server_result = task::future_result(builder);
    task::run(builder) { || new_server(ctx) }

    let builder = task::mk_task_builder();
    let client_result = task::future_result(builder);
    task::run(builder) { || new_client(ctx) }

    future::get(server_result);
    future::get(client_result);

    alt ctx.term() {
      ok(()) { }
      err(e) { fail e.to_str() }
    };
}
