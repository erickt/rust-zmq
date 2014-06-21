//! Module: zmq

#![crate_name = "zmq"]

#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(phase, macro_rules)]

#[phase(plugin, link)]
extern crate log;

extern crate libc;

use libc::{c_int, c_long, c_void, size_t, c_char, int64_t, uint64_t};
use libc::consts::os::posix88;
use std::{mem, ptr, str, slice};
use std::fmt;

/// The ZMQ container that manages all the sockets
type Context_ = *mut c_void;

/// A ZMQ socket
type Socket_ = *mut c_void;

const MSG_SIZE: uint = 48;

/// A message
type Msg_ = [c_char, ..MSG_SIZE];

#[link(name = "zmq")]
extern {
    fn zmq_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int);

    fn zmq_ctx_new() -> Context_;
    fn zmq_ctx_destroy(ctx: Context_) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> *const c_char;

    fn zmq_socket(ctx: Context_, typ: c_int) -> Socket_;
    fn zmq_close(socket: Socket_) -> c_int;

    fn zmq_getsockopt(socket: Socket_, opt: c_int, optval: *mut c_void, size: *mut size_t) -> c_int;
    fn zmq_setsockopt(socket: Socket_, opt: c_int, optval: *const c_void, size: size_t) -> c_int;

    fn zmq_bind(socket: Socket_, endpoint: *const c_char) -> c_int;
    fn zmq_connect(socket: Socket_, endpoint: *const c_char) -> c_int;

    fn zmq_msg_init(msg: &Msg_) -> c_int;
    fn zmq_msg_init_size(msg: &Msg_, size: size_t) -> c_int;
    fn zmq_msg_data(msg: &Msg_) -> *const u8;
    fn zmq_msg_size(msg: &Msg_) -> size_t;
    fn zmq_msg_close(msg: &Msg_) -> c_int;

    fn zmq_msg_send(msg: &Msg_, socket: Socket_, flags: c_int) -> c_int;
    fn zmq_msg_recv(msg: &Msg_, socket: Socket_, flags: c_int) -> c_int;

    fn zmq_poll(items: *mut PollItem, nitems: c_int, timeout: c_long) -> c_int;
}

/// Socket types
#[allow(non_camel_case_types)]
#[deriving(Clone, Show)]
pub enum SocketType {
    PAIR   = 0,
    PUB    = 1,
    SUB    = 2,
    REQ    = 3,
    REP    = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL   = 7,
    PUSH   = 8,
    XPUB   = 9,
    XSUB   = 10,
}

pub static DONTWAIT : int = 1;
pub static SNDMORE : int = 2;

#[allow(non_camel_case_types)]
#[deriving(Clone)]
#[allow(non_camel_case_types)]
pub enum Constants {
    ZMQ_AFFINITY          = 4,
    ZMQ_IDENTITY          = 5,
    ZMQ_SUBSCRIBE         = 6,
    ZMQ_UNSUBSCRIBE       = 7,
    ZMQ_RATE              = 8,
    ZMQ_RECOVERY_IVL      = 9,
    ZMQ_MCAST_LOOP        = 10,
    ZMQ_SNDBUF            = 11,
    ZMQ_RCVBUF            = 12,
    ZMQ_RCVMORE           = 13,
    ZMQ_FD                = 14,
    ZMQ_EVENTS            = 15,
    ZMQ_TYPE              = 16,
    ZMQ_LINGER            = 17,
    ZMQ_RECONNECT_IVL     = 18,
    ZMQ_BACKLOG           = 19,
    ZMQ_RECOVERY_IVL_MSEC = 20,
    ZMQ_RECONNECT_IVL_MAX = 21,
    ZMQ_MAXMSGSIZE        = 22,
    ZMQ_SNDHWM            = 23,
    ZMQ_RCVHWM            = 24,

    ZMQ_MAX_VSM_SIZE      = 30,
    ZMQ_DELIMITER         = 31,
    ZMQ_VSM               = 32,

    ZMQ_MSG_MORE          = 1,
    ZMQ_MSG_SHARED        = 128,
    ZMQ_MSG_MASK          = 129,

    ZMQ_HAUSNUMERO        = 156384712,
}

impl Constants {
    pub fn to_raw(&self) -> i32 {
        *self as i32
    }

    pub fn from_raw(raw: i32) -> Constants {
        // fails if `raw` is not a valid value
        match raw {
            4         => Constants::ZMQ_AFFINITY,
            5         => Constants::ZMQ_IDENTITY,
            6         => Constants::ZMQ_SUBSCRIBE,
            7         => Constants::ZMQ_UNSUBSCRIBE,
            8         => Constants::ZMQ_RATE,
            9         => Constants::ZMQ_RECOVERY_IVL,
            10        => Constants::ZMQ_MCAST_LOOP,
            11        => Constants::ZMQ_SNDBUF,
            12        => Constants::ZMQ_RCVBUF,
            13        => Constants::ZMQ_RCVMORE,
            14        => Constants::ZMQ_FD,
            15        => Constants::ZMQ_EVENTS,
            16        => Constants::ZMQ_TYPE,
            17        => Constants::ZMQ_LINGER,
            18        => Constants::ZMQ_RECONNECT_IVL,
            19        => Constants::ZMQ_BACKLOG,
            20        => Constants::ZMQ_RECOVERY_IVL_MSEC,
            21        => Constants::ZMQ_RECONNECT_IVL_MAX,
            22        => Constants::ZMQ_MAXMSGSIZE,
            23        => Constants::ZMQ_SNDHWM,
            24        => Constants::ZMQ_RCVHWM,

            30        => Constants::ZMQ_MAX_VSM_SIZE,
            31        => Constants::ZMQ_DELIMITER,
            32        => Constants::ZMQ_VSM,

            1         => Constants::ZMQ_MSG_MORE,
            128       => Constants::ZMQ_MSG_SHARED,
            129       => Constants::ZMQ_MSG_MASK,

            156384712 => Constants::ZMQ_HAUSNUMERO,

            x         => panic!("invalid constant {}", x as int),
        }
    }
}

#[deriving(Clone, Eq, PartialEq)]
pub enum Error {
    EACCES          = posix88::EACCES as int,
    EADDRINUSE      = posix88::EADDRINUSE as int,
    EAGAIN          = posix88::EAGAIN as int,
    EBUSY           = posix88::EBUSY as int,
    ECONNREFUSED    = posix88::ECONNREFUSED as int,
    EFAULT          = posix88::EFAULT as int,
    EHOSTUNREACH    = posix88::EHOSTUNREACH as int,
    EINPROGRESS     = posix88::EINPROGRESS as int,
    EINVAL          = posix88::EINVAL as int,
    EMFILE          = posix88::EMFILE as int,
    EMSGSIZE        = posix88::EMSGSIZE as int,
    ENAMETOOLONG    = posix88::ENAMETOOLONG as int,
    ENODEV          = posix88::ENODEV as int,
    ENOENT          = posix88::ENOENT as int,
    ENOMEM          = posix88::ENOMEM as int,
    ENOTCONN        = posix88::ENOTCONN as int,
    ENOTSOCK        = posix88::ENOTSOCK as int,
    EPROTO          = posix88::EPROTO as int,
    EPROTONOSUPPORT = posix88::EPROTONOSUPPORT as int,
    // magic number is EHAUSNUMERO + num
    ENOTSUP         = 156384713,
    ENOBUFS         = 156384715,
    ENETDOWN        = 156384716,
    EADDRNOTAVAIL   = 156384718,

    // native zmq error codes
    EFSM            = 156384763,
    ENOCOMPATPROTO  = 156384764,
    ETERM           = 156384765,
    EMTHREAD        = 156384766,
}

impl Error {
    pub fn to_raw(&self) -> i32 {
        *self as i32
    }

    pub fn from_raw(raw: i32) -> Error {
        match raw {
            posix88::EACCES          => Error::EACCES,
            posix88::EADDRINUSE      => Error::EADDRINUSE,
            posix88::EAGAIN          => Error::EAGAIN,
            posix88::EBUSY           => Error::EBUSY,
            posix88::ECONNREFUSED    => Error::ECONNREFUSED,
            posix88::EFAULT          => Error::EFAULT,
            posix88::EHOSTUNREACH    => Error::EHOSTUNREACH,
            posix88::EINPROGRESS     => Error::EINPROGRESS,
            posix88::EINVAL          => Error::EINVAL,
            posix88::EMFILE          => Error::EMFILE,
            posix88::EMSGSIZE        => Error::EMSGSIZE,
            posix88::ENAMETOOLONG    => Error::ENAMETOOLONG,
            posix88::ENODEV          => Error::ENODEV,
            posix88::ENOENT          => Error::ENOENT,
            posix88::ENOMEM          => Error::ENOMEM,
            posix88::ENOTCONN        => Error::ENOTCONN,
            posix88::ENOTSOCK        => Error::ENOTSOCK,
            posix88::EPROTO          => Error::EPROTO,
            posix88::EPROTONOSUPPORT => Error::EPROTONOSUPPORT,
            156384713                => Error::ENOTSUP,
            156384714                => Error::EPROTONOSUPPORT,
            156384715                => Error::ENOBUFS,
            156384716                => Error::ENETDOWN,
            156384717                => Error::EADDRINUSE,
            156384718                => Error::EADDRNOTAVAIL,
            156384719                => Error::ECONNREFUSED,
            156384720                => Error::EINPROGRESS,
            156384721                => Error::ENOTSOCK,
            156384763                => Error::EFSM,
            156384764                => Error::ENOCOMPATPROTO,
            156384765                => Error::ETERM,
            156384766                => Error::EMTHREAD,

            x => {
                unsafe {
                    panic!("unknown error [{}]: {}",
                          x as int,
                          std::string::raw::from_buf(zmq_strerror(x) as *const u8)
                    )
                }
            }
        }
    }
}

// Return the current zeromq version.
pub fn version() -> (int, int, int) {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;

    unsafe {
        zmq_version(&mut major, &mut minor, &mut patch);
    }

    (major as int, minor as int, patch as int)
}

/// zmq context, used to create sockets. Is thread safe, and can be safely
/// shared, but dropping it while sockets are still open will cause them to
/// close (see zmq_ctx_destroy(3)).
///
/// For this reason, one should use an Arc to share it, rather than any unsafe
/// trickery you might think up that would call the destructor.
pub struct Context {
    ctx: Context_,
}

impl Context {
    pub fn new() -> Context {
        Context {
            ctx: unsafe { zmq_ctx_new() }
        }
    }

    pub fn socket(&mut self, socket_type: SocketType) -> Result<Socket, Error> {
        let sock = unsafe {zmq_socket(self.ctx, socket_type as c_int)};

        if sock.is_null() {
            return Err(errno_to_error());
        }

        Ok(Socket { sock: sock, closed: false })
    }

    /// Try to destroy the context. This is different than the destructor; the
    /// destructor will loop when zmq_ctx_destroy returns EINTR
    pub fn destroy(&mut self) -> Result<(), Error> {
        if unsafe { zmq_ctx_destroy(self.ctx) } == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        debug!("context dropped");
        let mut e = self.destroy();
        while e.is_err() && (e.unwrap_err() != Error::EFAULT) {
            e = self.destroy();
        }
    }
}

pub struct Socket {
    sock: Socket_,
    closed: bool
}

impl Drop for Socket {
    fn drop(&mut self) {
        match self.close_final() {
            Ok(()) => { debug!("socket dropped") },
            Err(e) => panic!(e.to_string())
        }
    }
}

impl Socket {
    /// Accept connections on a socket.
    pub fn bind(&mut self, endpoint: &str) -> Result<(), Error> {
        let rc = endpoint.with_c_str (|cstr| {
            unsafe {zmq_bind(self.sock, cstr)}
        });

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Connect a socket.
    pub fn connect(&mut self, endpoint: &str) -> Result<(), Error> {
        let rc = endpoint.with_c_str (|cstr| {
            unsafe {zmq_connect(self.sock, cstr)}
        });

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Send a message
    pub fn send(&mut self, msg: &mut Message, flags: int) -> Result<(), Error> {
        unsafe {
            let rc = zmq_msg_send(&msg.msg, self.sock, flags as c_int);
            if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
        }
    }

    pub fn send_bytes(&mut self, data: &[u8], flags: int) -> Result<(), Error> {
        unsafe {
            let base_ptr = data.as_ptr();
            let len = data.len();
            let msg = [0, ..MSG_SIZE];

            // Copy the data into the message.
            let rc = zmq_msg_init_size(&msg, len as size_t);

            if rc == -1i32 { return Err(errno_to_error()); }

            ptr::copy_memory(zmq_msg_data(&msg) as *mut u8, base_ptr, len);

            let rc = zmq_msg_send(&msg, self.sock, flags as c_int);

            if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
        }
    }

    pub fn send_str(&mut self, data: &str, flags: int) -> Result<(), Error> {
        self.send_bytes(data.as_bytes(), flags)
    }

    /// Receive a message into a `Message`. The length passed to zmq_msg_recv
    /// is the length of the buffer.
    pub fn recv(&mut self, msg: &mut Message, flags: int) -> Result<(), Error> {
        let rc = unsafe {
            zmq_msg_recv(&msg.msg, self.sock, flags as c_int)
        };

        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }

    pub fn recv_msg(&mut self, flags: int) -> Result<Message, Error> {
        let mut msg = Message::new();
        match self.recv(&mut msg, flags) {
            Ok(()) => Ok(msg),
            Err(e) => Err(e),
        }
    }

    pub fn recv_bytes(&mut self, flags: int) -> Result<Vec<u8>, Error> {
        match self.recv_msg(flags) {
            Ok(msg) => Ok(msg.to_bytes()),
            Err(e) => Err(e),
        }
    }

    pub fn recv_str(&mut self, flags: int) -> Result<String, Error> {
        match self.recv_msg(flags) {
            Ok(msg) => Ok(msg.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if !self.closed {
            self.closed = true;

            if unsafe { zmq_close(self.sock) } == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }

    pub fn close_final(&mut self) -> Result<(), Error> {
        if !self.closed {
            if unsafe { zmq_close(self.sock) } == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }

    pub fn get_socket_type(&self) -> Result<SocketType, Error> {
        getsockopt_int(self.sock, Constants::ZMQ_TYPE.to_raw()).map(|ty| {
            match ty {
                0 => SocketType::PAIR,
                1 => SocketType::PUB,
                2 => SocketType::SUB,
                3 => SocketType::REQ,
                4 => SocketType::REP,
                5 => SocketType::DEALER,
                6 => SocketType::ROUTER,
                7 => SocketType::PULL,
                8 => SocketType::PUSH,
                9 => SocketType::XPUB,
                10 => SocketType::XSUB,
                _ => panic!("socket type is out of range!")
            }
        })
    }

    pub fn get_rcvmore(&self) -> Result<bool, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_RCVMORE.to_raw())
            .map(|o| o == 1i64 )
    }

    pub fn get_maxmsgsize(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_MAXMSGSIZE.to_raw())
    }


    pub fn get_sndhwm(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, Constants::ZMQ_SNDHWM.to_raw())
    }

    pub fn get_rcvhwm(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, Constants::ZMQ_RCVHWM.to_raw())
    }

    pub fn get_affinity(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, Constants::ZMQ_AFFINITY.to_raw())
    }

    pub fn get_identity(&self) -> Result<Vec<u8>, Error> {
        getsockopt_bytes(self.sock, Constants::ZMQ_IDENTITY.to_raw())
    }

    pub fn get_rate(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_RATE.to_raw())
    }

    pub fn get_recovery_ivl(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_RECOVERY_IVL.to_raw())
    }

    pub fn get_recovery_ivl_msec(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_RECOVERY_IVL_MSEC.to_raw())
    }

    pub fn get_mcast_loop(&self) -> Result<bool, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_MCAST_LOOP.to_raw())
            .map(|o| o == 1i64)
    }

    pub fn get_sndbuf(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, Constants::ZMQ_SNDBUF.to_raw())
    }

    pub fn get_rcvbuf(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, Constants::ZMQ_RCVBUF.to_raw())
    }

    pub fn get_linger(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_LINGER.to_raw())
    }

    pub fn get_reconnect_ivl(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, Constants::ZMQ_RECONNECT_IVL.to_raw())
    }

    pub fn get_reconnect_ivl_max(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, Constants::ZMQ_RECONNECT_IVL_MAX.to_raw())
    }

    pub fn get_backlog(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, Constants::ZMQ_BACKLOG.to_raw())
    }

    pub fn get_fd(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, Constants::ZMQ_FD.to_raw())
    }

    pub fn get_events(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, Constants::ZMQ_EVENTS.to_raw())
    }

    pub fn set_maxmsgsize(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, Constants::ZMQ_MAXMSGSIZE.to_raw(), value)
    }

    pub fn set_sndhwm(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, Constants::ZMQ_SNDHWM.to_raw(), value)
    }

    pub fn set_rcvhwm(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, Constants::ZMQ_RCVHWM.to_raw(), value)
    }

    pub fn set_affinity(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, Constants::ZMQ_AFFINITY.to_raw(), value)
    }

    pub fn set_identity(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, Constants::ZMQ_IDENTITY.to_raw(), value)
    }

    pub fn set_subscribe(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, Constants::ZMQ_SUBSCRIBE.to_raw(), value)
    }

    pub fn set_unsubscribe(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, Constants::ZMQ_UNSUBSCRIBE.to_raw(), value)
    }

    pub fn set_rate(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, Constants::ZMQ_RATE.to_raw(), value)
    }

    pub fn set_recovery_ivl(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, Constants::ZMQ_RECOVERY_IVL.to_raw(), value)
    }

    pub fn set_recovery_ivl_msec(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(
            self.sock, Constants::ZMQ_RECOVERY_IVL_MSEC.to_raw(), value)
    }

    pub fn set_mcast_loop(&self, value: bool) -> Result<(), Error> {
        let value = if value { 1i64 } else { 0i64 };
        setsockopt_i64(self.sock, Constants::ZMQ_MCAST_LOOP.to_raw(), value)
    }

    pub fn set_sndbuf(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, Constants::ZMQ_SNDBUF.to_raw(), value)
    }

    pub fn set_rcvbuf(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, Constants::ZMQ_RCVBUF.to_raw(), value)
    }

    pub fn set_linger(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, Constants::ZMQ_LINGER.to_raw(), value)
    }

    pub fn set_reconnect_ivl(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, Constants::ZMQ_RECONNECT_IVL.to_raw(), value)
    }

    pub fn set_reconnect_ivl_max(&self, value: int) -> Result<(), Error> {
        setsockopt_int(
            self.sock, Constants::ZMQ_RECONNECT_IVL_MAX.to_raw(), value)
    }

    pub fn set_backlog(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, Constants::ZMQ_BACKLOG.to_raw(), value)
    }

    pub fn as_poll_item<'a>(&self, events: i16) -> PollItem<'a> {
        PollItem {
            socket: self.sock,
            fd: 0,
            events: events,
            revents: 0
        }
    }
}

pub struct Message {
    msg: Msg_
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            let _ = zmq_msg_close(&self.msg);
        }
    }
}

impl Message {
    pub fn new() -> Message {
        unsafe {
            let message = Message { msg: [0, ..MSG_SIZE] };
            let _ = zmq_msg_init(&message.msg);
            message
        }
    }

    pub fn with_bytes<T>(&self, f: |&[u8]| -> T) -> T {
        unsafe {
            let data = zmq_msg_data(&self.msg);
            let len = zmq_msg_size(&self.msg) as uint;
            slice::raw::buf_as_slice(data, len, f)
        }
    }

    pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe {
            let data = zmq_msg_data(&self.msg);
            let len = zmq_msg_size(&self.msg) as uint;
            ::std::mem::transmute(::std::raw::Slice {
                data: data,
                len: len,
            })
        }
    }

    pub fn with_str<T>(&self, f: |&str| -> T) -> T {
        self.with_bytes(|v| f(str::from_utf8(v).unwrap()))
    }

    pub fn as_str<'a>(&'a self) -> Option<&'a str> {
        str::from_utf8(self.as_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.with_bytes(|v| v.to_vec())
    }

    pub fn to_string(&self) -> String {
        self.with_str(|s| s.to_string())
    }
}

pub static POLLIN : i16 = 1i16;
pub static POLLOUT : i16 = 2i16;
pub static POLLERR : i16 = 4i16;

#[repr(C)]
pub struct PollItem<'a> {
    socket: Socket_,
    fd: c_int,
    events: i16,
    revents: i16
}

impl<'a> PollItem<'a> {
    pub fn get_revents(&self) -> i16 {
        self.revents
    }
}

pub fn poll<'a>(items: &mut [PollItem<'a>], timeout: i64) -> Result<int, Error> {
    unsafe {
        let rc = zmq_poll(
            items.as_mut_ptr(),
            items.len() as c_int,
            timeout);
        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(rc as int)
        }
    }
}

impl fmt::Show for Error {
    /// Return the error string for an error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "{}",
                   std::string::raw::from_buf(zmq_strerror(*self as c_int) as *const u8))
        }
    }
}

macro_rules! getsockopt_num(
    ($name:ident, $c_ty:ty, $ty:ty) => (
        fn $name(sock: Socket_, opt: c_int) -> Result<$ty, Error> {
            unsafe {
                let mut value: $c_ty = 0;
                let value_ptr = &mut value as *mut $c_ty;
                let mut size = mem::size_of::<$c_ty>() as size_t;

                if -1 == zmq_getsockopt(sock, opt, value_ptr as *mut c_void, &mut size) {
                    Err(errno_to_error())
                } else {
                    Ok(value as $ty)
                }
            }
        }
    )
)

getsockopt_num!(getsockopt_int, c_int, int)
getsockopt_num!(getsockopt_i64, int64_t, i64)
getsockopt_num!(getsockopt_u64, uint64_t, u64)

fn getsockopt_bytes(sock: Socket_, opt: c_int) -> Result<Vec<u8>, Error> {
    unsafe {
        // The only binary option in zeromq is ZMQ_IDENTITY, which can have
        // a max size of 255 bytes.
        let mut size = 255 as size_t;
        let mut value = Vec::with_capacity(size as uint);

        let r = zmq_getsockopt(
            sock,
            opt,
            value.as_mut_ptr() as *mut c_void,
            &mut size);

        if r == -1i32 {
            Err(errno_to_error())
        } else {
            value.truncate(size as uint);
            Ok(value)
        }
    }
}

macro_rules! setsockopt_num(
    ($name:ident, $ty:ty) => (
        fn $name(sock: Socket_, opt: c_int, value: $ty) -> Result<(), Error> {
            unsafe {
                let size = mem::size_of::<$ty>() as size_t;

                if -1 == zmq_setsockopt(sock, opt, (&value as *const $ty) as *const c_void, size) {
                    Err(errno_to_error())
                } else {
                    Ok(())
                }
            }
        }
    )
)

setsockopt_num!(setsockopt_int, int)
setsockopt_num!(setsockopt_i64, i64)
setsockopt_num!(setsockopt_u64, u64)

fn setsockopt_bytes(sock: Socket_, opt: c_int, value: &[u8]) -> Result<(), Error> {
    unsafe {
        let r = zmq_setsockopt(
            sock,
            opt,
            value.as_ptr() as *const c_void,
            value.len() as size_t
        );

        if r == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

fn errno_to_error() -> Error {
    Error::from_raw(unsafe { zmq_errno() })
}
