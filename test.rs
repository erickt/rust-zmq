use std;
use zmq;

import result::{ok, err};

fn main() {
    let (major, minor, patch) = zmq::version();

    log_err #fmt("version: %d %d %d", major, minor, patch);

    let ctx = alt zmq::init(1) {
      ok(ctx) { ctx }
      err(e) { fail zmq::error_to_str(e) }
    };

    let socket = alt ctx.socket(zmq::REQ) {
      ok(socket) { socket }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.setsockopt_u64(zmq::constants::ZMQ_HWM, 10u64) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.getsockopt_u64(zmq::constants::ZMQ_HWM) {
      ok(hwm) { log_err #fmt("hwm: %s", u64::str(hwm)) }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.setsockopt_vec(
            zmq::constants::ZMQ_IDENTITY,
            str::bytes("identity")) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.getsockopt_vec(zmq::constants::ZMQ_IDENTITY) {
      ok(identity) {
        log_err #fmt("hwm: %s", str::unsafe_from_bytes(identity))
      }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.connect("tcp://127.0.0.1:3456") {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.send(str::bytes("foo"), 0i32) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e); }
    }

    alt socket.recv(0i32) {
      ok(d) { log_err str::unsafe_from_bytes(d); }
      err(e) { fail zmq::error_to_str(e); }
    }

    alt socket.close() {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt ctx.term() {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };
}
