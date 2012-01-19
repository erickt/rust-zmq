/*
Module: zmq
*/

use std;
import ctypes::*;
import result::{ok,err};

export context;
export socket;
export socket_kind;
export constants;
export version;
export init;
export term;
export getsockopt_i64;
export getsockopt_u64;
export getsockopt_vec;
export setsockopt_i64;
export setsockopt_u64;
export setsockopt_vec;
export bind;
export connect;
export send;
export send_str;
export recv;
export recv_str;
export close;
export error;
export error_to_str;

type context = *void;
type socket = *void;
type msg = {
    content: *void,
    flags: u8,
    vsm_size: u8,
    vsm_data0: u32,
    vsm_data1: u32,
    vsm_data2: u32,
    vsm_data3: u32,
    vsm_data4: u32,
    vsm_data5: u32,
    vsm_data6: u32,
};

#[nolink]
#[link_args = "-L/opt/local/lib -lzmq"]
native mod libzmq {
    fn zmq_version(major: *c_int, minor: *c_int, patch: *c_int);

    fn zmq_init(io_threads: c_int) -> context;
    fn zmq_term(ctx: context) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> str::sbuf;

    fn zmq_socket(ctx: context, typ: c_int) -> socket;
    fn zmq_close(socket: socket) -> c_int;

    fn zmq_getsockopt<T>(
            socket: socket,
            option: c_int,
            optval: *T,
            size: *size_t) -> c_int;
    fn zmq_setsockopt<T>(
            socket: socket,
            option: c_int,
            optval: *T,
            size: size_t) -> c_int;

    fn zmq_bind(socket: socket, endpoint: str::sbuf) -> c_int;
    fn zmq_connect(socket: socket, endpoint: str::sbuf) -> c_int;

    fn zmq_msg_init(msg: msg) -> c_int;
    fn zmq_msg_init_size(msg: msg, size: size_t) -> c_int;
    fn zmq_msg_data(msg: msg) -> *mutable u8;
    fn zmq_msg_size(msg: msg) -> size_t;
    fn zmq_msg_close(msg: msg) -> c_int;

    fn zmq_send(socket: socket, msg: msg, flags: c_int) -> c_int;
    fn zmq_recv(socket: socket, msg: msg, flags: c_int) -> c_int;
}

mod constants {
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

tag error {
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

// Return the current zeromq version.
fn version() -> (int, int, int) {
    let major = 0i32;
    let minor = 0i32;
    let patch = 0i32;
    libzmq::zmq_version(ptr::addr_of(major), ptr::addr_of(minor), ptr::addr_of(patch));
    (major as int, minor as int, patch as int)
}

// Create a zeromq context.
fn init(io_threads: int) -> result::t<context, error> unsafe {
    let ctx = libzmq::zmq_init(io_threads as i32);

    ret if unsafe::reinterpret_cast(ctx) == 0 {
        err(errno_to_error())
    } else {
        ok(ctx)
    }
}

fn socket(ctx: context, kind: socket_kind) -> result::t<socket, error> unsafe {
    let sock = libzmq::zmq_socket(ctx, socket_kind_to_i32(kind));

    ret if unsafe::reinterpret_cast(sock) == 0 {
        err(errno_to_error())
    } else {
        ok(sock)
    }
}

fn term(ctx: context) -> result::t<(), error> {
    let rc = libzmq::zmq_term(ctx);
    if rc == -1i32 {
        err(errno_to_error())
    } else {
        ok(())
    }
}

fn getsockopt_i64(sock: socket, option: i32) -> result::t<i64, error> {
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

fn getsockopt_u64(sock: socket, option: i32) -> result::t<u64, error> {
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

fn getsockopt_vec(sock: socket, option: i32) -> result::t<[u8], error> unsafe {
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

fn setsockopt_i64(sock: socket, option: i32, value: i64) -> result::t<(), error> {
    let r = libzmq::zmq_setsockopt(
            sock,
            option,
            ptr::addr_of(value),
            sys::size_of::<u64>()
            );

    if r == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn setsockopt_u64(sock: socket, option: i32, value: u64) -> result::t<(), error> {
    let r = libzmq::zmq_setsockopt(
            sock,
            option,
            ptr::addr_of(value),
            sys::size_of::<u64>()
            );

    if r == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn setsockopt_vec(sock: socket, option: i32, value: [u8]) -> result::t<(), error> unsafe {
    let r = libzmq::zmq_setsockopt(
            sock,
            option,
            vec::to_ptr(value),
            vec::len(value)
            );

    if r == -1i32 { err(errno_to_error()) } else { ok(()) }
}

// Accept connections on a socket.
fn bind(sock: socket, endpoint: str) -> result::t<(), error> {
    // Work around rust bug #1286.
    let sock = sock;
    let rc = str::as_buf(endpoint, { |b| libzmq::zmq_bind(sock, b) });
    if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
}

// Connect a socket.
fn connect(sock: socket, endpoint: str) -> result::t<(), error> {
    // Work around rust bug #1286.
    let sock = sock;
    let rc = str::as_buf(endpoint, { |b| libzmq::zmq_connect(sock, b) });
    if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn send(sock: socket, data: [u8], flags: c_int) -> result::t<(), error> {
    let size = vec::len(data);
    let msg = {
        content: ptr::null::<void>(),
        flags: 0u8,
        vsm_size: 0u8,
        vsm_data0: 0u32,
        vsm_data1: 0u32,
        vsm_data2: 0u32,
        vsm_data3: 0u32,
        vsm_data4: 0u32,
        vsm_data5: 0u32,
        vsm_data6: 0u32,
    };

    libzmq::zmq_msg_init_size(msg, size);
    let msg_data = libzmq::zmq_msg_data(msg);

    let i = 0u;
    while i < size {
        unsafe { *ptr::mut_offset(msg_data, i) = data[i]; }
        i += 1u;
    }

    let rc = libzmq::zmq_send(sock, msg, flags);

    libzmq::zmq_msg_close(msg);

    if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn send_str(sock: socket, data: str, flags: c_int) -> result::t<(), error> {
    send(sock, str::bytes(data), flags)
}

fn recv(sock: socket, flags: c_int) -> result::t<[u8], error> unsafe {
    let msg = {
        content: ptr::null::<void>(),
        flags: 0u8,
        vsm_size: 0u8,
        vsm_data0: 0u32,
        vsm_data1: 0u32,
        vsm_data2: 0u32,
        vsm_data3: 0u32,
        vsm_data4: 0u32,
        vsm_data5: 0u32,
        vsm_data6: 0u32,
    };

    libzmq::zmq_msg_init(msg);

    let rc = libzmq::zmq_recv(sock, msg, flags);

    let msg_data = libzmq::zmq_msg_data(msg);
    let msg_size = libzmq::zmq_msg_size(msg);
    let data = vec::init_fn({ |i| *ptr::mut_offset(msg_data, i) }, msg_size);

    libzmq::zmq_msg_close(msg);

    if rc == -1i32 { err(errno_to_error()) } else { ok(data) }
}

fn recv_str(sock: socket, flags: c_int) -> result::t<str, error> {
    result::chain(recv(sock, flags)) {|bytes|
        ok(str::unsafe_from_bytes(bytes))
    }
}

fn close(sock: socket) -> result::t<(), error> {
    if libzmq::zmq_close(sock) == -1i32 {
        err(errno_to_error())
    } else {
        ok(())
    }
}

// Convert a socket kind into the constant value.
fn socket_kind_to_i32(k: socket_kind) -> c_int {
    alt k {
        PAIR. { constants::ZMQ_PAIR }
        PUB. { constants::ZMQ_PUB }
        SUB. { constants::ZMQ_SUB }
        REQ. { constants::ZMQ_REQ }
        REP. { constants::ZMQ_REP }
        DEALER. { constants::ZMQ_DEALER }
        ROUTER. { constants::ZMQ_ROUTER }
        PULL. { constants::ZMQ_PULL }
        PUSH. { constants::ZMQ_PUSH }
        XPUB. { constants::ZMQ_XPUB }
        XSUB. { constants::ZMQ_XSUB }
    }
}

// Return the error string for an error.
fn error_to_str(error: error) -> str unsafe {
    let s = libzmq::zmq_strerror(error_to_errno(error));
    ret if unsafe::reinterpret_cast(s) == -1 {
        let s = unsafe::reinterpret_cast(s);
        str::from_cstr(s)
    } else {
        ""
    }
}

// Convert the errno into an error type.
fn errno_to_error() -> error {
    alt libzmq::zmq_errno() {
        e if e == constants::ENOTSUP { ENOTSUP }
        e if e == constants::EPROTONOSUPPORT { EPROTONOSUPPORT }
        e if e == constants::ENOBUFS { ENOBUFS }
        e if e == constants::ENETDOWN { ENETDOWN }
        e if e == constants::EADDRINUSE { EADDRINUSE }
        e if e == constants::EADDRNOTAVAIL { EADDRNOTAVAIL }
        e if e == constants::ECONNREFUSED { ECONNREFUSED }
        e if e == constants::EINPROGRESS { EINPROGRESS }
        e if e == constants::ENOTSOCK { ENOTSOCK }
        e if e == constants::EFSM { EFSM }
        e if e == constants::ENOCOMPATPROTO { ENOCOMPATPROTO }
        e if e == constants::ETERM { ETERM }
        e if e == constants::EMTHREAD { EMTHREAD }
        e { UNKNOWN(e) }
    }
}

// Convert an error into an error number.
fn error_to_errno(error: error) -> c_int {
    alt error {
        ENOTSUP. { constants::ENOTSUP }
        EPROTONOSUPPORT. { constants::EPROTONOSUPPORT }
        ENOBUFS. { constants::ENOBUFS }
        ENETDOWN. { constants::ENETDOWN }
        EADDRINUSE. { constants::EADDRINUSE }
        EADDRNOTAVAIL. { constants::EADDRNOTAVAIL }
        ECONNREFUSED. { constants::ECONNREFUSED }
        EINPROGRESS. { constants::EINPROGRESS }
        ENOTSOCK. { constants::ENOTSOCK }
        EFSM. { constants::EFSM }
        ENOCOMPATPROTO. { constants::ENOCOMPATPROTO }
        ETERM. { constants::ETERM }
        EMTHREAD. { constants::EMTHREAD }
        UNKNOWN(e) { e }
    }
}
