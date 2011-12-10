/*
Module: zmq
*/

#[cfg(target_os = "macos")];

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

#[link_name = "zmq"]
#[link_args = "-L /opt/local/lib"]
native mod libzmq {
    type ctx_t;
    type socket_t;
    type msg_t;

    fn zmq_init(io_threads: c_int) -> *ctx_t;
    fn zmq_term(ctx: *ctx_t) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> str::sbuf;

    fn zmq_socket(ctx: *ctx_t, typ: c_int) -> *socket_t;
    fn zmq_close(socket: *socket_t) -> c_int;

    fn zmq_getsockopt<T>(socket: *socket_t, option: c_int, optval: *T,
                         size: *size_t) -> c_int;
    fn zmq_setsockopt<T>(socket: *socket_t, option: c_int, optval: *T,
                         size: size_t) -> c_int;

    fn zmq_bind(socket: *socket_t, endpoint: str::sbuf) -> c_int;
    fn zmq_connect(socket: *socket_t, endpoint: str::sbuf) -> c_int;

/*
fn zmq_msg_init(msg: *msg_t) -> int;
fn zmq_msg_init_size(msg: *msg_t, size: size_t) -> int;
fn zmq_msg_init_data(msg: *msg_t, data: *u8, size: size_t,
free_cb: free_cb_t, hint: *void);

fn zmq_send(socket: *socket_t, msg: *msg_t, flags: int) -> int;
fn zmq_recv(socket: *socket_t, msg: *msg_t, flags: int) -> int;
*/
}

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

    const ZMQ_NOBLOCK : c_int = 1i32;
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
    type ctx_t = *libzmq::ctx_t;
    type socket_t = *libzmq::socket_t;

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

    fn init(io_threads: int) -> ctx_t { libzmq::zmq_init(io_threads as i32) }

    fn term(ctx: ctx_t) -> option::t<error_t> {
        let r = libzmq::zmq_term(ctx);
        if r == -1i32 { some(errno_to_error()) } else { none }
    }

    fn socket(ctx: ctx_t, kind: socket_kind) -> result::t<socket_t, error_t> unsafe {
        let s = libzmq::zmq_socket(ctx, socket_kind_to_i32(kind));
        ret if unsafe::reinterpret_cast(s) == 0 {
            err(errno_to_error())
        } else {
            ok(s)
        };
    }

    fn close(socket: socket_t) -> option::t<error_t> {
        let r = libzmq::zmq_close(socket);
        if r == -1i32 { some(errno_to_error()) } else { none }
    }

    fn bind(socket: socket_t, endpoint: str) -> option::t<error_t> {
        let r = str::as_buf(endpoint, { |buf| libzmq::zmq_bind(socket, buf) });

        if r == -1i32 { some(errno_to_error()) } else { none }
    }

    fn get_high_water_mark(socket: socket_t) -> result::t<u64, error_t> {
        let hwm = 0u64;
        let size = sys::size_of::<u64>();

        let r = libzmq::zmq_getsockopt(
            socket,
            libzmq_constants::ZMQ_HWM,
            ptr::addr_of(hwm),
            ptr::addr_of(size)
        );

        if r == -1i32 { err(errno_to_error()) } else { ok(hwm) }
    }

    fn set_high_water_mark(socket: socket_t, hwm: u64) -> option::t<error_t> {
        let r = libzmq::zmq_setsockopt(
            socket,
            libzmq_constants::ZMQ_HWM,
            ptr::addr_of(hwm),
            sys::size_of::<u64>()
        );

        if r == -1i32 { some(errno_to_error()) } else { none }
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
    let ctx = zmq::init(1);
    let socket = alt zmq::socket(ctx, zmq::REP) {
      ok(s) { s }
      err(e) { fail zmq::error_to_str(e) }
    };

    alt zmq::set_high_water_mark(socket, 10u64) {
      none. { log_err "no error"; }
      some(e) { zmq::error_to_str(e); }
    }

    alt zmq::bind(socket, "tcp://127.0.0.1:2345") {
      none. { log_err "no error"; }
      some(e) { zmq::error_to_str(e); }
    }

    zmq::close(socket);
    log_err "hello\n";
    zmq::term(ctx);
}
