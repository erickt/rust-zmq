/*
Module: zmq
*/

use std;
import std::ctypes::*;
import std::option;
import std::option::{none, some};
import std::ptr;
import std::str;
import std::sys;
import std::unsafe;
import std::result;
import std::result::{ok, err};
import std::u64;
import std::vec;
import std::c_vec;

#[link_name = "zmq"]
#[link_args = "-L /opt/local/lib"]
native mod libzmq {
    fn zmq_version(major: *c_int, minor: *c_int, patch: *c_int);

    fn zmq_init(io_threads: c_int) -> zmq_ctx_t;
    fn zmq_term(ctx: zmq_ctx_t) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> str::sbuf;

    fn zmq_socket(ctx: zmq_ctx_t, typ: c_int) -> zmq_socket_t;
    fn zmq_close(socket: zmq_socket_t) -> c_int;

    fn zmq_getsockopt<T>(socket: zmq_socket_t, option: c_int, optval: *T,
                         size: *size_t) -> c_int;
    fn zmq_setsockopt<T>(socket: zmq_socket_t, option: c_int, optval: *T,
                         size: size_t) -> c_int;

    fn zmq_bind(socket: zmq_socket_t, endpoint: str::sbuf) -> c_int;
    fn zmq_connect(socket: zmq_socket_t, endpoint: str::sbuf) -> c_int;

    fn zmq_msg_init(msg: zmq_msg_t) -> c_int;
    fn zmq_msg_init_size(msg: zmq_msg_t, size: size_t) -> c_int;
    fn zmq_msg_data(msg: zmq_msg_t) -> *mutable u8;
    fn zmq_msg_size(msg: zmq_msg_t) -> size_t;
    fn zmq_msg_close(msg: zmq_msg_t) -> c_int;

    fn zmq_send(socket: zmq_socket_t, msg: zmq_msg_t, flags: c_int) -> c_int;
    fn zmq_recv(socket: zmq_socket_t, msg: zmq_msg_t, flags: c_int) -> c_int;
}

#[link_args = "-L ."]
native mod rustzmq {
    fn rustzmq_msg_create() -> zmq_msg_t;
    fn rustzmq_msg_destroy(msg: zmq_msg_t);
}

type zmq_ctx_t = *void;
type zmq_socket_t = *void;
type zmq_msg_t = *void;

mod libzmq_constants {
    const ZMQ_PAIR : c_int = 0i32;
    const ZMQ_PUB : c_int = 1i32;
    const ZMQ_SUB : c_int = 2i32;
    const ZMQ_REQ : c_int = 3i32;
    const ZMQ_REP : c_int = 4i32;
    const ZMQ_DEALER : c_int = 5i32;
    const ZMQ_ROUTER : c_int = 6i32;
    const ZMQ_PULL : c_int = 7i32;
    const ZMQ_PUSH : c_int = 8i32;
    const ZMQ_XPUB : c_int = 9i32;
    const ZMQ_XSUB : c_int = 10i32;

    const ZMQ_HWM : c_int = 1i32;
    const ZMQ_SNDHWM : c_int = 1i32;
    const ZMQ_RCVHWM : c_int = 1i32;
    const ZMQ_SWAP : c_int = 3i32;
    const ZMQ_AFFINITY : c_int = 4i32;
    const ZMQ_IDENTITY : c_int = 5i32;
    const ZMQ_SUBSCRIBE : c_int = 6i32;
    const ZMQ_UNSUBSCRIBE : c_int = 7i32;
    const ZMQ_RATE : c_int = 8i32;
    const ZMQ_RECOVERY_IVL : c_int = 9i32;
    const ZMQ_MCAST_LOOP : c_int = 10i32;
    const ZMQ_SNDBUF : c_int = 11i32;
    const ZMQ_RCVBUF : c_int = 12i32;
    const ZMQ_RCVMORE : c_int = 13i32;
    const ZMQ_FD : c_int = 14i32;
    const ZMQ_EVENTS : c_int = 15i32;
    const ZMQ_TYPE : c_int = 16i32;
    const ZMQ_LINGER : c_int = 17i32;
    const ZMQ_RECONNECT_IVL : c_int = 18i32;
    const ZMQ_BACKLOG : c_int = 19i32;
    const ZMQ_RECOVERY_IVL_MSEC : c_int = 20i32;
    const ZMQ_RECONNECT_IVL_MAX : c_int = 21i32;

    const ZMQ_DONTWAIT : c_int = 1i32;
    const ZMQ_SNDMORE : c_int = 2i32;

    const ZMQ_POLLIN : c_int = 1i32;
    const ZMQ_POLLOUT : c_int = 2i32;
    const ZMQ_POLLERR : c_int = 4i32;

    const ZMQ_MAX_VSM_SIZE : c_int = 30i32;
    const ZMQ_DELIMITER : c_int = 31i32;
    const ZMQ_VSM : c_int = 32i32;

    const ZMQ_MSG_MORE : c_int = 1i32;
    const ZMQ_MSG_SHARED : c_int = 128i32;
    const ZMQ_MSG_MASK : c_int = 129i32;

    const ZMQ_HAUSNUMERO : c_int = 156384712i32;

    const ENOTSUP : c_int = 156384712i32 + 1i32; //ZMQ_HAUSNUMERO + 1i32;
    const EPROTONOSUPPORT : c_int = 156384712i32 + 2i32; //ZMQ_HAUSNUMERO + 2i32;
    const ENOBUFS : c_int = 156384712i32 + 3i32; //ZMQ_HAUSNUMERO + 3i32;
    const ENETDOWN : c_int = 156384712i32 + 4i32; //ZMQ_HAUSNUMERO + 4i32;
    const EADDRINUSE : c_int = 156384712i32 + 5i32; //ZMQ_HAUSNUMERO + 5i32;
    const EADDRNOTAVAIL : c_int = 156384712i32 + 6i32; //ZMQ_HAUSNUMERO + 6i32;
    const ECONNREFUSED : c_int = 156384712i32 + 7i32; //ZMQ_HAUSNUMERO + 7i32;
    const EINPROGRESS : c_int = 156384712i32 + 8i32; //ZMQ_HAUSNUMERO + 8i32;
    const ENOTSOCK : c_int = 156384712i32 + 9i32; //ZMQ_HAUSNUMERO + 9i32;

    const EFSM : c_int = 156384712i32 + 51i32; //ZMQ_HAUSNUMERO + 51i32;
    const ENOCOMPATPROTO : c_int = 156384712i32 + 52i32; //ZMQ_HAUSNUMERO + 52i32;
    const ETERM : c_int = 156384712i32 + 53i32; //ZMQ_HAUSNUMERO + 53i32;
    const EMTHREAD : c_int = 156384712i32 + 54i32; //ZMQ_HAUSNUMERO + 54i32;
}

mod zmq {
    tag socket_kind {
        PAIR;
        PUB;
        SUB;
        REQ;
        REP;
        DEALER;
        ROUTER;
        PULL;
        PUSH;
        XPUB;
        XSUB;
    }

    tag error_t {
        ENOTSUP;
        EPROTONOSUPPORT;
        ENOBUFS;
        ENETDOWN;
        EADDRINUSE;
        EADDRNOTAVAIL;
        ECONNREFUSED;
        EINPROGRESS;
        ENOTSOCK;
        EFSM;
        ENOCOMPATPROTO;
        ETERM;
        EMTHREAD;
        UNKNOWN(c_int);
    }

    fn version() -> (int, int, int) {
        let major = 0i32;
        let minor = 0i32;
        let patch = 0i32;
        libzmq::zmq_version(ptr::addr_of(major), ptr::addr_of(minor), ptr::addr_of(patch));
        (major as int, minor as int, patch as int)
    }

    type context_t = obj {
        fn socket(kind: socket_kind) -> result::t<socket_t, error_t>;
        fn close() -> result::t<(), error_t>;
    };

    type socket_t = obj {
        fn bind(endpoint: str) -> result::t<(), error_t>;
        fn connect(endpoint: str) -> result::t<(), error_t>;
        fn sendmsg(data: [u8], flags: c_int) -> result::t<(), error_t>;
        fn recvmsg(flags: c_int) -> result::t<[u8], error_t>;
        fn close() -> result::t<(), error_t>;

        fn getsockopt_i64(option: i32) -> result::t<i64, error_t>;
        fn getsockopt_u64(option: i32) -> result::t<u64, error_t>;
        fn getsockopt_vec(option: i32) -> result::t<[u8], error_t>;

        fn setsockopt_i64(option: i32, value: i64) -> result::t<(), error_t>;
        fn setsockopt_u64(option: i32, value: u64) -> result::t<(), error_t>;
        fn setsockopt_vec(option: i32, value: [u8]) -> result::t<(), error_t>;
    };

    fn create(io_threads: int) -> result::t<context_t, error_t> unsafe {
        let ctx = libzmq::zmq_init(io_threads as i32);
        ret if unsafe::reinterpret_cast(ctx) == 0 {
            err(errno_to_error())
        } else {
            ok(Context(ctx))
        }
    }

    obj Context(ctx: zmq_ctx_t) {
        fn socket(kind: socket_kind) -> result::t<socket_t, error_t> unsafe {
            let sock = libzmq::zmq_socket(ctx, socket_kind_to_i32(kind));
            ret if unsafe::reinterpret_cast(sock) == 0 {
                err(errno_to_error())
            } else {
                ok(Socket(sock))
            }
        }

        fn close() -> result::t<(), error_t> {
            let rc = libzmq::zmq_term(ctx);
            if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
        }
    }

    obj Socket(sock: zmq_socket_t) {
        fn bind(endpoint: str) -> result::t<(), error_t> {
            _bind(sock, endpoint)
        }

        fn connect(endpoint: str) -> result::t<(), error_t> {
            _connect(sock, endpoint)
        }

        fn sendmsg(data: [u8], flags: c_int) -> result::t<(), error_t> {
            let size = vec::len(data);
            let msg = rustzmq::rustzmq_msg_create();

            libzmq::zmq_msg_init_size(msg, size);
            let msg_data = libzmq::zmq_msg_data(msg);

            let i = 0u;
            while i < size {
                unsafe { *ptr::mut_offset(msg_data, i) = data[i]; }
                i += 1u;
            }

            let rc = libzmq::zmq_send(sock, msg, flags);

            libzmq::zmq_msg_close(msg);
            rustzmq::rustzmq_msg_destroy(msg);

            if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
        }

        fn recvmsg(flags: c_int) -> result::t<[u8], error_t> unsafe {
            let msg = rustzmq::rustzmq_msg_create();

            libzmq::zmq_msg_init(msg);

            let rc = libzmq::zmq_recv(sock, msg, flags);

            let msg_data = libzmq::zmq_msg_data(msg);
            let msg_size = libzmq::zmq_msg_size(msg);
            let data = vec::init_fn({ |i| *ptr::mut_offset(msg_data, i) }, msg_size);

            libzmq::zmq_msg_close(msg);
            rustzmq::rustzmq_msg_destroy(msg);

            if rc == -1i32 { err(errno_to_error()) } else { ok(data) }
        }

        fn close() -> result::t<(), error_t> {
            let rc = libzmq::zmq_close(sock);
            if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
        }

        fn getsockopt_i64(option: i32) -> result::t<i64, error_t> {
            let value = 0i64;
            let size = sys::size_of::<i64>();

            let r = libzmq::zmq_getsockopt(
                sock,
                option,
                ptr::addr_of(value),
                ptr::addr_of(size)
            );

            if r == -1i32 { err(errno_to_error()) } else { ok(value) }
        }

        fn getsockopt_u64(option: i32) -> result::t<u64, error_t> {
            let value = 0u64;
            let size = sys::size_of::<u64>();

            let r = libzmq::zmq_getsockopt(
                sock,
                option,
                ptr::addr_of(value),
                ptr::addr_of(size)
            );

            if r == -1i32 { err(errno_to_error()) } else { ok(value) }
        }

        fn getsockopt_vec(option: i32) -> result::t<[u8], error_t> unsafe {
            let value = [];

            // The only binary option in zeromq is ZMQ_IDENTITY, which can have
            // a max size of 255 bytes.
            let size = 255u;
            vec::reserve::<u8>(value, size);

            let r = libzmq::zmq_getsockopt(
                sock,
                option,
                vec::to_ptr(value),
                ptr::addr_of(size)
            );

            if r == -1i32 {
                err(errno_to_error())
            } else {
                vec::unsafe::set_len(value, size);
                ok(value)
            }
        }

        fn setsockopt_i64(option: i32, value: i64) -> result::t<(), error_t> {
            let r = libzmq::zmq_setsockopt(
                sock,
                option,
                ptr::addr_of(value),
                sys::size_of::<u64>()
            );

            if r == -1i32 { err(errno_to_error()) } else { ok(()) }
        }

        fn setsockopt_u64(option: i32, value: u64) -> result::t<(), error_t> {
            let r = libzmq::zmq_setsockopt(
                sock,
                option,
                ptr::addr_of(value),
                sys::size_of::<u64>()
            );

            if r == -1i32 { err(errno_to_error()) } else { ok(()) }
        }

        fn setsockopt_vec(option: i32, value: [u8]) -> result::t<(), error_t> unsafe {
            let r = libzmq::zmq_setsockopt(
                sock,
                option,
                vec::to_ptr(value),
                vec::len(value)
            );

            if r == -1i32 { err(errno_to_error()) } else { ok(()) }
        }
    }

    // Work around a bug by moving this out of an object.
    fn _bind(sock: zmq_socket_t, endpoint: str) -> result::t<(), error_t> {
        let rc = str::as_buf(endpoint, { |b| libzmq::zmq_bind(sock, b) });
        if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
    }

    fn _connect(sock: zmq_socket_t, endpoint: str) -> result::t<(), error_t> {
        let rc = str::as_buf(endpoint, { |b| libzmq::zmq_connect(sock, b) });
        if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
    }



    fn socket_kind_to_i32(k: socket_kind) -> c_int {
        alt k {
          PAIR. { libzmq_constants::ZMQ_PAIR }
          PUB. { libzmq_constants::ZMQ_PUB }
          SUB. { libzmq_constants::ZMQ_SUB }
          REQ. { libzmq_constants::ZMQ_REQ }
          REP. { libzmq_constants::ZMQ_REP }
          DEALER. { libzmq_constants::ZMQ_DEALER }
          ROUTER. { libzmq_constants::ZMQ_ROUTER }
          PULL. { libzmq_constants::ZMQ_PULL }
          PUSH. { libzmq_constants::ZMQ_PUSH }
          XPUB. { libzmq_constants::ZMQ_XPUB }
          XSUB. { libzmq_constants::ZMQ_XSUB }
        }
    }

    fn error_to_str(error: error_t) -> str unsafe {
        let s = libzmq::zmq_strerror(error_to_errno(error));
        ret if unsafe::reinterpret_cast(s) == -1 {
            let s = unsafe::reinterpret_cast(s);
            str::str_from_cstr(s)
        } else {
            ""
        }
    }

    fn errno_to_error() -> error_t {
        alt libzmq::zmq_errno() {
          e when e == libzmq_constants::ENOTSUP { ENOTSUP }
          e when e == libzmq_constants::EPROTONOSUPPORT { EPROTONOSUPPORT }
          e when e == libzmq_constants::ENOBUFS { ENOBUFS }
          e when e == libzmq_constants::ENETDOWN { ENETDOWN }
          e when e == libzmq_constants::EADDRINUSE { EADDRINUSE }
          e when e == libzmq_constants::EADDRNOTAVAIL { EADDRNOTAVAIL }
          e when e == libzmq_constants::ECONNREFUSED { ECONNREFUSED }
          e when e == libzmq_constants::EINPROGRESS { EINPROGRESS }
          e when e == libzmq_constants::ENOTSOCK { ENOTSOCK }
          e when e == libzmq_constants::EFSM { EFSM }
          e when e == libzmq_constants::ENOCOMPATPROTO { ENOCOMPATPROTO }
          e when e == libzmq_constants::ETERM { ETERM }
          e when e == libzmq_constants::EMTHREAD { EMTHREAD }
          e { UNKNOWN(e) }
        }
    }

    fn error_to_errno(error: error_t) -> c_int {
        alt error {
          ENOTSUP. { libzmq_constants::ENOTSUP }
          EPROTONOSUPPORT. { libzmq_constants::EPROTONOSUPPORT }
          ENOBUFS. { libzmq_constants::ENOBUFS }
          ENETDOWN. { libzmq_constants::ENETDOWN }
          EADDRINUSE. { libzmq_constants::EADDRINUSE }
          EADDRNOTAVAIL. { libzmq_constants::EADDRNOTAVAIL }
          ECONNREFUSED. { libzmq_constants::ECONNREFUSED }
          EINPROGRESS. { libzmq_constants::EINPROGRESS }
          ENOTSOCK. { libzmq_constants::ENOTSOCK }
          EFSM. { libzmq_constants::EFSM }
          ENOCOMPATPROTO. { libzmq_constants::ENOCOMPATPROTO }
          ETERM. { libzmq_constants::ETERM }
          EMTHREAD. { libzmq_constants::EMTHREAD }
          UNKNOWN(e) { e }
        }
    }
}

fn main() {
    let (major, minor, patch) = zmq::version();

    log_err #fmt("version: %d %d %d", major, minor, patch);

    let ctx = alt zmq::create(1) {
      ok(ctx) { ctx }
      err(e) { fail zmq::error_to_str(e) }
    };

    let socket = alt ctx.socket(zmq::REQ) {
      ok(socket) { socket }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.setsockopt_u64(libzmq_constants::ZMQ_HWM, 10u64) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.getsockopt_u64(libzmq_constants::ZMQ_HWM) {
      ok(hwm) { log_err #fmt("hwm: %s", u64::str(hwm)) }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.setsockopt_vec(
            libzmq_constants::ZMQ_IDENTITY,
            str::bytes("identity")) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.getsockopt_vec(libzmq_constants::ZMQ_IDENTITY) {
      ok(identity) {
        log_err #fmt("hwm: %s", str::unsafe_from_bytes(identity))
      }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.connect("tcp://127.0.0.1:3456") {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt socket.sendmsg(str::bytes("foo"), 0i32) {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e); }
    }

    alt socket.recvmsg(0i32) {
        ok(d) { log_err str::unsafe_from_bytes(d); }
        err(e) { fail zmq::error_to_str(e); }
    }

    alt socket.close() {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt ctx.close() {
      ok(()) { }
      err(e) { fail zmq::error_to_str(e) }
    };
}
