use std;
use zmq;

import result::{ok, err};

fn new_server(&&ctx: zmq::context) {
    let socket = alt zmq::socket(ctx, zmq::REP) {
      ok(socket) { socket }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt zmq::bind(socket, "tcp://127.0.0.1:3456") {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e); }
    }

    let msg = alt zmq::recv(socket, 0i32) {
      ok(d) { str::unsafe_from_bytes(d) }
      err(e) { fail zmq::error_to_str(e) }
    };

    log_err #fmt("received %s", msg);

    alt zmq::send(socket, str::bytes(#fmt("hello %s", msg)), 0i32) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e); }
    }

    alt zmq::close(socket) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };
}

fn new_client(&&ctx: zmq::context) {
    let socket = alt zmq::socket(ctx, zmq::REQ) {
      ok(socket) { socket }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt zmq::setsockopt_u64(socket, zmq::constants::ZMQ_HWM, 10u64) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e); }
    };

    alt zmq::getsockopt_u64(socket, zmq::constants::ZMQ_HWM) {
      ok(hwm) { log_err #fmt("hwm: %s", u64::str(hwm)); }
      err(e) { fail zmq::error_to_str(e); }
    }

    alt zmq::setsockopt_vec(
            socket,
            zmq::constants::ZMQ_IDENTITY,
            str::bytes("identity")) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e); }
    };

    alt zmq::getsockopt_vec(socket, zmq::constants::ZMQ_IDENTITY) {
      ok(identity) {
        log_err #fmt("hwm: %s", str::unsafe_from_bytes(identity))
      }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt zmq::connect(socket, "tcp://127.0.0.1:3456") {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt zmq::send(socket, str::bytes("foo"), 0i32) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e); }
    }

    alt zmq::recv(socket, 0i32) {
      ok(d) { log_err str::unsafe_from_bytes(d); }
      err(e) { fail zmq::error_to_str(e); }
    }

    alt zmq::close(socket) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };
}

fn main() {
    let (major, minor, patch) = zmq::version();

    log_err #fmt("version: %d %d %d", major, minor, patch);

    let ctx = alt zmq::init(1) {
      ok(ctx) { ctx }
      err(e) { fail zmq::error_to_str(e) }
    };

    let server_task = task::spawn_joinable(copy ctx, new_server);
    let client_task = task::spawn_joinable(copy ctx, new_client);

    task::join(server_task);
    task::join(client_task);

    alt zmq::term(ctx) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };
}
