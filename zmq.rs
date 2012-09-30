//! Module: zmq

export Context;
export Socket;
export socket_util;
export SocketType;
export PAIR;
export PUB;
export SUB;
export REQ;
export REP;
export DEALER;
export ROUTER;
export PULL;
export PUSH;
export XPUB;
export XSUB;
export DONTWAIT;
export SNDMORE;
export version;
export init;
export POLLIN;
export POLLOUT;
export POLLERR;
export poll;
export Error;
export to_str;

/// The ZMQ container that manages all the sockets
type Context_ = *c_void;

/// A ZMQ socket
type Socket_ = *c_void;

/// A message
type Msg_ = {
    content: *c_void,
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

extern mod zmq {
    fn zmq_version(major: *c_int, minor: *c_int, patch: *c_int);

    fn zmq_init(io_threads: c_int) -> Context_;
    fn zmq_term(ctx: Context_) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> *c_char;

    fn zmq_socket(ctx: Context_, typ: c_int) -> Socket_;
    fn zmq_close(socket: Socket_) -> c_int;

    fn zmq_getsockopt(
            socket: Socket_,
            opt: c_int,
            optval: *c_void,
            size: *size_t) -> c_int;
    fn zmq_setsockopt(
            socket: Socket_,
            opt: c_int,
            optval: *c_void,
            size: size_t) -> c_int;

    fn zmq_bind(socket: Socket_, endpoint: *c_char) -> c_int;
    fn zmq_connect(socket: Socket_, endpoint: *c_char) -> c_int;

    fn zmq_msg_init(msg: &Msg_) -> c_int;
    fn zmq_msg_init_size(msg: &Msg_, size: size_t) -> c_int;
    fn zmq_msg_data(msg: &Msg_) -> *u8;
    fn zmq_msg_size(msg: &Msg_) -> size_t;
    fn zmq_msg_close(msg: &Msg_) -> c_int;

    fn zmq_send(socket: Socket_, msg: &Msg_, flags: c_int) -> c_int;
    fn zmq_recv(socket: Socket_, msg: &Msg_, flags: c_int) -> c_int;

    fn zmq_poll(items: *PollItem, nitems: c_int, timeout: c_long) -> c_int;
}

/// Socket types
pub enum SocketType {
    PAIR = 0,
    PUB = 1,
    SUB = 2,
    REQ = 3,
    REP = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL = 7,
    PUSH = 8,
    XPUB = 9,
    XSUB = 10,
}

pub const DONTWAIT : int = 1;
pub const SNDMORE : int = 2;

pub mod constants {
    pub const ZMQ_HWM : c_int = 1i32;
    pub const ZMQ_SNDHWM : c_int = 1i32;
    pub const ZMQ_RCVHWM : c_int = 1i32;
    pub const ZMQ_SWAP : c_int = 3i32;
    pub const ZMQ_AFFINITY : c_int = 4i32;
    pub const ZMQ_IDENTITY : c_int = 5i32;
    pub const ZMQ_SUBSCRIBE : c_int = 6i32;
    pub const ZMQ_UNSUBSCRIBE : c_int = 7i32;
    pub const ZMQ_RATE : c_int = 8i32;
    pub const ZMQ_RECOVERY_IVL : c_int = 9i32;
    pub const ZMQ_MCAST_LOOP : c_int = 10i32;
    pub const ZMQ_SNDBUF : c_int = 11i32;
    pub const ZMQ_RCVBUF : c_int = 12i32;
    pub const ZMQ_RCVMORE : c_int = 13i32;
    pub const ZMQ_FD : c_int = 14i32;
    pub const ZMQ_EVENTS : c_int = 15i32;
    pub const ZMQ_TYPE : c_int = 16i32;
    pub const ZMQ_LINGER : c_int = 17i32;
    pub const ZMQ_RECONNECT_IVL : c_int = 18i32;
    pub const ZMQ_BACKLOG : c_int = 19i32;
    pub const ZMQ_RECOVERY_IVL_MSEC : c_int = 20i32;
    pub const ZMQ_RECONNECT_IVL_MAX : c_int = 21i32;

    pub const ZMQ_MAX_VSM_SIZE : c_int = 30i32;
    pub const ZMQ_DELIMITER : c_int = 31i32;
    pub const ZMQ_VSM : c_int = 32i32;

    pub const ZMQ_MSG_MORE : c_int = 1i32;
    pub const ZMQ_MSG_SHARED : c_int = 128i32;
    pub const ZMQ_MSG_MASK : c_int = 129i32;

    pub const ZMQ_HAUSNUMERO : c_int = 156384712i32;
}

enum Error {
    ENOTSUP = 156384712 + 1, //ZMQ_HAUSNUMERO + 1,
    EPROTONOSUPPORT = 156384712 + 2, //ZMQ_HAUSNUMERO + 2,
    ENOBUFS = 156384712 + 3, //ZMQ_HAUSNUMERO + 3,
    ENETDOWN = 156384712 + 4, //ZMQ_HAUSNUMERO + 4,
    EADDRINUSE = 156384712 + 5, //ZMQ_HAUSNUMERO + 5,
    EADDRNOTAVAIL = 156384712 + 6, //ZMQ_HAUSNUMERO + 6,
    ECONNREFUSED = 156384712 + 7, //ZMQ_HAUSNUMERO + 7,
    EINPROGRESS = 156384712 + 8, //ZMQ_HAUSNUMERO + 8,
    ENOTSOCK = 156384712 + 9, //ZMQ_HAUSNUMERO + 9,

    EFSM = 156384712 + 51, //ZMQ_HAUSNUMERO + 51,
    ENOCOMPATPROTO = 156384712 + 52, //ZMQ_HAUSNUMERO + 52,
    ETERM = 156384712 + 53, //ZMQ_HAUSNUMERO + 53,
    EMTHREAD = 156384712 + 54, //ZMQ_HAUSNUMERO + 54,
}

// Return the current zeromq version.
pub fn version() -> (int, int, int) {
    let major = 0i32;
    let minor = 0i32;
    let patch = 0i32;
    zmq::zmq_version(
        ptr::addr_of(major),
        ptr::addr_of(minor),
        ptr::addr_of(patch));
    (major as int, minor as int, patch as int)
}

// Create a zeromq context.
pub fn init(io_threads: int) -> Result<Context, Error> unsafe {
    let ctx = zmq::zmq_init(io_threads as i32);

    if ctx.is_null() {
        return Err(errno_to_error());
    }

    Ok(Context { ctx: ctx })
}

struct Context {
    priv ctx: Context_,
}

pub impl Context {
    fn socket(socket_type: SocketType) -> Result<Socket, Error> unsafe {
        let sock = zmq::zmq_socket(self.ctx, socket_type as c_int);

        if sock.is_null() {
            return Err(errno_to_error());
        }

        Ok(Socket { sock: sock as Socket_, closed: false })
    }

    fn term() -> Result<(), Error> {
        let rc = zmq::zmq_term(self.ctx);
        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

struct Socket {
    priv sock: Socket_,
    priv mut closed: bool,

    drop {
        match self.close() {
            Ok(()) => {}
            Err(e) => fail e.to_str()
        }
    }
}

pub impl Socket {
    fn get_socket_type() -> Result<SocketType, Error> {
        do getsockopt_int(self.sock, constants::ZMQ_TYPE).map |ty| {
            match *ty {
                0 => PAIR,
                1 => PUB,
                2 => SUB,
                3 => REQ,
                4 => REP,
                5 => DEALER,
                6 => ROUTER,
                7 => PULL,
                8 => PUSH,
                9 => XPUB,
                10 => XSUB,
                _ => fail ~"socket type is out of range!",
            }
        }
    }

    fn get_rcvmore() -> Result<bool, Error> {
        do getsockopt_i64(self.sock, constants::ZMQ_RCVMORE).chain |o| {
            Ok(o == 1i64)
        }
    }

    fn get_hwm() -> Result<u64, Error> {
        getsockopt_u64(self.sock, constants::ZMQ_HWM)
    }

    fn get_affinity() -> Result<u64, Error> {
        getsockopt_u64(self.sock, constants::ZMQ_AFFINITY)
    }

    fn get_identity() -> Result<~[u8], Error> {
        getsockopt_bytes(self.sock, constants::ZMQ_IDENTITY)
    }

    fn get_rate() -> Result<i64, Error> {
        getsockopt_i64(self.sock, constants::ZMQ_RATE)
    }

    fn get_recovery_ivl() -> Result<i64, Error> {
        getsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL)
    }

    fn get_recovery_ivl_msec() -> Result<i64, Error> {
        getsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL_MSEC)
    }

    fn get_mcast_loop() -> Result<bool, Error> {
        do getsockopt_i64(self.sock, constants::ZMQ_MCAST_LOOP).chain |o| {
            Ok(o == 1i64)
        }
    }

    fn get_sndbuf() -> Result<u64, Error> {
        getsockopt_u64(self.sock, constants::ZMQ_SNDBUF)
    }

    fn get_rcvbuf() -> Result<u64, Error> {
        getsockopt_u64(self.sock, constants::ZMQ_RCVBUF)
    }

    fn get_linger() -> Result<i64, Error> {
        getsockopt_i64(self.sock, constants::ZMQ_LINGER)
    }

    fn get_reconnect_ivl() -> Result<int, Error> {
        getsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL)
    }

    fn get_reconnect_ivl_max() -> Result<int, Error> {
        getsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL_MAX)
    }

    fn get_backlog() -> Result<int, Error> {
        getsockopt_int(self.sock, constants::ZMQ_BACKLOG)
    }

    fn get_fd() -> Result<i64, Error> {
        getsockopt_i64(self.sock, constants::ZMQ_FD)
    }

    fn get_events() -> Result<u32, Error> {
        getsockopt_u32(self.sock, constants::ZMQ_EVENTS)
    }

    fn set_hwm(value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, constants::ZMQ_HWM, value)
    }

    fn set_affinity(value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, constants::ZMQ_AFFINITY, value)
    }

    fn set_identity(value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, constants::ZMQ_IDENTITY, value)
    }

    fn set_subscribe(value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, constants::ZMQ_SUBSCRIBE, value)
    }

    fn set_unsubscribe(value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, constants::ZMQ_UNSUBSCRIBE, value)
    }

    fn set_rate(value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, constants::ZMQ_RATE, value)
    }

    fn set_recovery_ivl(value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL, value)
    }

    fn set_recovery_ivl_msec(value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL_MSEC, value)
    }

    fn set_mcast_loop(value: bool) -> Result<(), Error> {
        let value = if value { 1i64 } else { 0i64 };
        setsockopt_i64(self.sock, constants::ZMQ_MCAST_LOOP, value)
    }

    fn set_sndbuf(value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, constants::ZMQ_SNDBUF, value)
    }

    fn set_rcvbuf(value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, constants::ZMQ_RCVBUF, value)
    }

    fn set_linger(value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, constants::ZMQ_LINGER, value)
    }

    fn set_reconnect_ivl(value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL, value)
    }

    fn set_reconnect_ivl_max(value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL_MAX, value)
    }

    fn set_backlog(value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, constants::ZMQ_BACKLOG, value)
    }

    /// Accept connections on a socket.
    fn bind(endpoint: &str) -> Result<(), Error> unsafe {
        let rc = do str::as_c_str(endpoint) |cstr| {
            zmq::zmq_bind(self.sock, cstr)
        };

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Connect a socket.
    fn connect(endpoint: &str) -> Result<(), Error> unsafe {
        let rc = do str::as_c_str(endpoint) |cstr| {
            zmq::zmq_connect(self.sock, cstr)
        };

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    fn send(data: &[const u8], flags: int) -> Result<(), Error> {
        do vec::as_const_buf(data) |base_ptr, len| {
            let msg = {
                content: ptr::null(),
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

            // Copy the data into the message.
            zmq::zmq_msg_init_size(&msg, len as size_t);

            unsafe {
                ptr::memcpy(
                    ::cast::transmute(zmq::zmq_msg_data(&msg)),
                    base_ptr,
                    len);
            }

            let rc = zmq::zmq_send(self.sock, &msg, flags as c_int);

            zmq::zmq_msg_close(&msg);

            if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
        }
    }

    fn send_str(data: &str, flags: int) -> Result<(), Error> {
        str::byte_slice(data, |bytes| self.send(bytes, flags))
    }

    unsafe fn recv(flags: int) -> Result<Message, Error> {
        let msg = {
            content: ptr::null(),
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

        zmq::zmq_msg_init(&msg);
        let rc = zmq::zmq_recv(self.sock, &msg, flags as c_int);

        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(Message { msg: msg })
        }
    }

    fn recv_bytes(flags: int) -> Result<~[u8], Error> unsafe {
        match move self.recv(flags) {
            Ok(move msg) => Ok(msg.to_bytes()),
            Err(move e) => Err(e),
        }
    }

    fn recv_str(flags: int) -> Result<~str, Error> unsafe {
        match move self.recv(flags) {
            Ok(move msg) => Ok(msg.to_str()),
            Err(move e) => Err(e),
        }
    }

    fn close() -> Result<(), Error> {
        if !self.closed {
            self.closed = true;

            if zmq::zmq_close(self.sock) == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }
}

struct Message {
    priv msg: Msg_,

    drop {
        zmq::zmq_msg_close(&self.msg);
    }
}

pub impl Message {
    unsafe fn with_ptr<T>(f: fn(*u8, uint) -> T) -> T {
        let data = zmq::zmq_msg_data(&self.msg);
        let len = zmq::zmq_msg_size(&self.msg) as uint;

        f(data, len)
    }

    fn with_bytes<T>(f: fn(&[u8]) -> T) -> T unsafe {
        do self.with_ptr |data, len| {
            vec::raw::form_slice(data, len, f)
        }
    }

    fn with_str<T>(f: fn(&str) -> T) -> T unsafe {
        do self.with_ptr |data, len| {
            str::raw::buf_as_slice(data, len, f)
        }
    }

    fn to_bytes() -> ~[u8] {
        self.with_bytes(|v| vec::from_slice(v))
    }

    fn to_str() -> ~str {
        self.with_str(|s| str::from_slice(s))
    }
}

pub const POLLIN : i16 = 1i16;
pub const POLLOUT : i16 = 2i16;
pub const POLLERR : i16 = 4i16;

pub type PollItem = {
    socket: Socket_,
    fd: c_int,
    mut events: i16,
    mut revents: i16,
};

pub fn poll(items: &[PollItem], timeout: i64) -> Result<(), Error> unsafe {
    do vec::as_imm_buf(items) |p, len| {
        let rc = zmq::zmq_poll(
            p,
            len as c_int,
            timeout as c_long);
        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }
}

pub impl Error: to_str::ToStr {
    /// Return the error string for an error.
    fn to_str() -> ~str unsafe {
        str::raw::from_c_str(zmq::zmq_strerror(self as c_int))
    }
}

/// Convert the errno into an error type.
fn errno_to_error() -> Error {
    match zmq::zmq_errno() {
        e if e == ENOTSUP as c_int         => ENOTSUP,
        e if e == EPROTONOSUPPORT as c_int => EPROTONOSUPPORT,
        e if e == ENOBUFS as c_int         => ENOBUFS,
        e if e == ENETDOWN as c_int        => ENETDOWN,
        e if e == EADDRINUSE as c_int      => EADDRINUSE,
        e if e == EADDRNOTAVAIL as c_int   => EADDRNOTAVAIL,
        e if e == ECONNREFUSED as c_int    => ECONNREFUSED,
        e if e == EINPROGRESS as c_int     => EINPROGRESS,
        e if e == ENOTSOCK as c_int        => ENOTSOCK,
        e if e == EFSM as c_int            => EFSM,
        e if e == ENOCOMPATPROTO as c_int  => ENOCOMPATPROTO,
        e if e == ETERM as c_int           => ETERM,
        e if e == EMTHREAD as c_int        => EMTHREAD,
        e => unsafe {
            fail str::raw::from_c_str(zmq::zmq_strerror(e as c_int))
        }
    }
}

fn getsockopt_int(sock: Socket_, opt: c_int) -> Result<int, Error> {
    let value = 0u32 as c_int;
    let size = sys::size_of::<c_int>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value as int) }
}

fn getsockopt_u32(sock: Socket_, opt: c_int) -> Result<u32, Error> {
    let value = 0u32;
    let size = sys::size_of::<u32>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value) }
}

fn getsockopt_i64(sock: Socket_, opt: c_int) -> Result<i64, Error> {
    let value = 0i64;
    let size = sys::size_of::<i64>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value) }
}

fn getsockopt_u64(sock: Socket_, opt: c_int) -> Result<u64, Error> {
    let value = 0u64;
    let size = sys::size_of::<u64>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value) }
}

fn getsockopt_bytes(
    sock: Socket_,
    opt: c_int
) -> Result<~[u8], Error> unsafe {
    // The only binary option in zeromq is ZMQ_IDENTITY, which can have
    // a max size of 255 bytes.
    let mut size = 255 as size_t;
    let mut value = vec::with_capacity(size as uint);

    let r = zmq::zmq_getsockopt(
        sock,
        opt as c_int,
        unsafe { vec::raw::to_ptr(value) as *c_void },
        ptr::addr_of(size));

    if r == -1i32 {
        Err(errno_to_error())
    } else {
        vec::raw::set_len(&mut value, size as uint);
        Ok(value)
    }
}

fn setsockopt_int(
    sock: Socket_,
    opt: c_int,
    value: int
) -> Result<(), Error> {
    let value = value as c_int;
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        sys::size_of::<c_int>() as size_t);

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_i64(
    sock: Socket_,
    opt: c_int,
    value: i64
) -> Result<(), Error> {
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        sys::size_of::<i64>() as size_t);

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_u64(
    sock: Socket_,
    opt: c_int,
    value: u64
) -> Result<(), Error> {
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        sys::size_of::<u64>() as size_t);

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_buf(
    sock: Socket_,
    opt: c_int,
    p: *u8,
    len: uint
) -> Result<(), Error> unsafe {
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        unsafe { p as *c_void },
        len as size_t);

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_bytes(
    sock: Socket_,
    opt: c_int,
    value: &[u8]
) -> Result<(), Error> unsafe {
    vec::as_imm_buf(value, |p, len| setsockopt_buf(sock, opt, p, len))
}

fn setsockopt_str(
    sock: Socket_,
    opt: c_int,
    value: &str
) -> Result<(), Error> unsafe {
    str::as_buf(value, |p, len| setsockopt_buf(sock, opt, p, len))
}
