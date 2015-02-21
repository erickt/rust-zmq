//! Module: zmq

#![feature(int_uint, core, std_misc, libc, rustc_private)]

#[macro_use]
extern crate log;

extern crate libc;
extern crate "zmq-sys" as zmq_sys;

use libc::{c_int, c_void, size_t, int64_t, uint64_t};
use libc::consts::os::posix88;
use std::{mem, ptr, str, slice};
use std::ffi;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub use SocketType::*;

/// Socket types
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
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

impl Copy for SocketType {}

pub static DONTWAIT : int = 1;
pub static SNDMORE : int = 2;

#[allow(non_camel_case_types)]
#[derive(Clone)]
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
}

impl Copy for Constants {}

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

            x         => panic!("invalid constant {}", x as int),
        }
    }
}

const ZMQ_HAUSNUMERO: int = 156384712;

#[derive(Clone, Eq, PartialEq)]
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
    ENOTSUP         = ZMQ_HAUSNUMERO + 1,
    ENOBUFS         = ZMQ_HAUSNUMERO + 3,
    ENETDOWN        = ZMQ_HAUSNUMERO + 4,
    EADDRNOTAVAIL   = ZMQ_HAUSNUMERO + 6,

    // native zmq error codes
    EFSM            = ZMQ_HAUSNUMERO + 51,
    ENOCOMPATPROTO  = ZMQ_HAUSNUMERO + 52,
    ETERM           = ZMQ_HAUSNUMERO + 53,
    EMTHREAD        = ZMQ_HAUSNUMERO + 54,
}

impl Copy for Error {}

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
                    let s = zmq_sys::zmq_strerror(x) as *const i8;
                    panic!("unknown error [{}]: {}",
                        x as int,
                        str::from_utf8(ffi::CStr::from_ptr(s).to_bytes()).unwrap()
                    )
                }
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        unsafe {
            let s = zmq_sys::zmq_strerror(*self as c_int) as *const i8;
            let v: &'static [u8] =
                mem::transmute(ffi::CStr::from_ptr(s).to_bytes());
            str::from_utf8(v).unwrap()
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let s = zmq_sys::zmq_strerror(*self as c_int) as *const i8;
            let v: &'static [u8] =
                mem::transmute(ffi::CStr::from_ptr(s).to_bytes());
            write!(f, "{}", str::from_utf8(v).unwrap())
        }
    }
}

// Return the current zeromq version.
pub fn version() -> (int, int, int) {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;

    unsafe {
        zmq_sys::zmq_version(&mut major, &mut minor, &mut patch);
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
    ctx: *mut libc::c_void,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            ctx: unsafe { zmq_sys::zmq_ctx_new() }
        }
    }

    pub fn socket(&mut self, socket_type: SocketType) -> Result<Socket, Error> {
        let sock = unsafe { zmq_sys::zmq_socket(self.ctx, socket_type as c_int) };

        if sock.is_null() {
            return Err(errno_to_error());
        }

        Ok(Socket { sock: sock, closed: false })
    }

    /// Try to destroy the context. This is different than the destructor; the
    /// destructor will loop when zmq_ctx_destroy returns EINTR
    pub fn destroy(&mut self) -> Result<(), Error> {
        if unsafe { zmq_sys::zmq_ctx_destroy(self.ctx) } == -1i32 {
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
        while e == Err(Error::EFAULT) {
            e = self.destroy();
        }
    }
}

pub struct Socket {
    sock: *mut libc::c_void,
    closed: bool
}

unsafe impl Send for Socket {}

impl Drop for Socket {
    fn drop(&mut self) {
        match self.close_final() {
            Ok(()) => { debug!("socket dropped") },
            Err(e) => panic!(e)
        }
    }
}

impl Socket {
    /// Accept connections on a socket.
    pub fn bind(&mut self, endpoint: &str) -> Result<(), Error> {
        let rc = unsafe { zmq_sys::zmq_bind(self.sock,
                          ffi::CString::new(endpoint.as_bytes()).unwrap().as_ptr()) };
        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Connect a socket.
    pub fn connect(&mut self, endpoint: &str) -> Result<(), Error> {
        let rc = unsafe { zmq_sys::zmq_connect(self.sock,
                          ffi::CString::new(endpoint.as_bytes()).unwrap().as_ptr()) };
        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Send a `&[u8]` message.
    pub fn send(&mut self, data: &[u8], flags: int) -> Result<(), Error> {
        let msg = try!(Message::from_slice(data));
        self.send_msg(msg, flags)
    }

    /// Send a `Message` message.
    pub fn send_msg(&mut self, mut msg: Message, flags: int) -> Result<(), Error> {
        let rc = unsafe {
            zmq_sys::zmq_msg_send(&mut msg.msg, self.sock, flags as c_int)
        };

        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }

    pub fn send_str(&mut self, data: &str, flags: int) -> Result<(), Error> {
        self.send(data.as_bytes(), flags)
    }

    /// Receive a message into a `Message`. The length passed to zmq_msg_recv
    /// is the length of the buffer.
    pub fn recv(&mut self, msg: &mut Message, flags: int) -> Result<(), Error> {
        let rc = unsafe {
            zmq_sys::zmq_msg_recv(&mut msg.msg, self.sock, flags as c_int)
        };

        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }

    pub fn recv_msg(&mut self, flags: int) -> Result<Message, Error> {
        let mut msg = try!(Message::new());
        match self.recv(&mut msg, flags) {
            Ok(()) => Ok(msg),
            Err(e) => Err(e),
        }
    }

    pub fn recv_bytes(&mut self, flags: int) -> Result<Vec<u8>, Error> {
        match self.recv_msg(flags) {
            Ok(msg) => Ok(msg.as_slice().to_vec()),
            Err(e) => Err(e),
        }
    }

    #[deprecated = "use `socket.recv_string()` instead"]
    pub fn recv_str(&mut self, flags: int) -> Result<String, Error> {
        match self.recv_bytes(flags) {
            Ok(msg) => Ok(String::from_utf8(msg).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// Read a `String` from the socket.
    pub fn recv_string(&mut self, flags: int) -> Result<Result<String, Vec<u8>>, Error> {
        match self.recv_bytes(flags) {
            Ok(msg) => Ok(Ok(String::from_utf8(msg).unwrap_or("".to_string()))),
            Err(e) => Err(e),
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if !self.closed {
            self.closed = true;

            if unsafe { zmq_sys::zmq_close(self.sock) } == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }

    pub fn close_final(&mut self) -> Result<(), Error> {
        if !self.closed {
            if unsafe { zmq_sys::zmq_close(self.sock) } == -1i32 {
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

    pub fn as_poll_item<'a>(&self, events: i16) -> PollItem {
        PollItem {
            socket: self.sock,
            fd: 0,
            events: events,
            revents: 0
        }
    }
}

const MSG_SIZE: uint = 32;

pub struct Message {
    msg: zmq_sys::zmq_msg_t,
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            let rc = zmq_sys::zmq_msg_close(&mut self.msg);
            assert_eq!(rc, 0);
        }
    }
}

impl Message {
    /// Create an empty `Message`.
    pub fn new() -> Result<Message, Error> {
        unsafe {
            let mut msg = zmq_sys::zmq_msg_t { unnamed_field1: [0; MSG_SIZE] };
            let rc = zmq_sys::zmq_msg_init(&mut msg);

            if rc == -1i32 { return Err(errno_to_error()); }

            Ok(Message { msg: msg })
        }
    }

    /// Create a `Message` preallocated with `len` uninitialized bytes.
    pub unsafe fn with_capacity_unallocated(len: uint) -> Result<Message, Error> {
        let mut msg = zmq_sys::zmq_msg_t { unnamed_field1: [0; MSG_SIZE] };
        let rc = zmq_sys::zmq_msg_init_size(&mut msg, len as size_t);

        if rc == -1i32 { return Err(errno_to_error()); }

        Ok(Message { msg: msg })
    }

    /// Create a `Message` with space for `len` bytes that are initialized to 0.
    pub fn with_capacity(len: uint) -> Result<Message, Error> {
        unsafe {
            let mut msg = try!(Message::with_capacity_unallocated(len));
            ptr::zero_memory(msg.as_mut_ptr(), len);
            Ok(msg)
        }
    }

    /// Create a `Message` from a `&[u8]`. This will copy `data` into the message.
    pub fn from_slice(data: &[u8]) -> Result<Message, Error> {
        unsafe {
            let mut msg = try!(Message::with_capacity_unallocated(data.len()));
            ptr::copy_nonoverlapping_memory(msg.as_mut_ptr(), data.as_ptr(), data.len());
            Ok(msg)
        }
    }

    #[deprecated = "use `as_slice()` instead"]
    pub fn with_bytes<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(self.as_slice())
    }

    #[deprecated = "renamed to `*message` or `message.as_slice()`"]
    pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
        self.as_slice()
    }

    #[allow(deprecated)]
    #[deprecated = "use `str::from_utf8(message.as_slice().unwrap())` instead"]
    pub fn with_str<T, F: Fn(&str) -> T>(&self, f: F) -> T {
        f(self.as_str().unwrap())
    }

    pub fn as_str<'a>(&'a self) -> Option<&'a str> {
        str::from_utf8(self.as_slice()).ok()
    }

    #[allow(deprecated)]
    #[deprecated = "use `message.to_vec()` instead"]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.with_bytes(|v| v.to_vec())
    }

    #[allow(deprecated)]
    #[deprecated = "use `String::from_utf8(message.as_slice())` instead"]
    pub fn to_string(&self) -> String {
        self.with_str(|s| s.to_string())
    }
}

impl Deref for Message {
    type Target = [u8];

    fn deref<'a>(&'a self) -> &'a [u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe {
            let ptr = self.msg.unnamed_field1.as_ptr() as *mut _;
            let data = zmq_sys::zmq_msg_data(ptr);
            let len = zmq_sys::zmq_msg_size(ptr) as uint;
            slice::from_raw_parts(mem::transmute(&data), len)
        }
    }
}

impl DerefMut for Message {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe {
            let data = zmq_sys::zmq_msg_data(&mut self.msg);
            let len = zmq_sys::zmq_msg_size(&mut self.msg) as uint;
            slice::from_raw_parts_mut(mem::transmute(&data), len)
        }
    }
}

pub static POLLIN : i16 = 1i16;
pub static POLLOUT : i16 = 2i16;
pub static POLLERR : i16 = 4i16;

#[repr(C)]
pub struct PollItem {
    socket: *mut libc::c_void,
    fd: c_int,
    events: i16,
    revents: i16
}

impl<'a> PollItem {
    pub fn from_fd(fd: c_int) -> PollItem {
        PollItem {
            socket: ptr::null_mut(),
            fd: fd,
            events: 0,
            revents: 0
        }
    }

    pub fn get_revents(&self) -> i16 {
        self.revents
    }
}

pub fn poll<'a>(items: &mut [PollItem], timeout: i64) -> Result<int, Error> {
    unsafe {
        let rc = zmq_sys::zmq_poll(
            items.as_mut_ptr() as *mut zmq_sys::zmq_pollitem_t,
            items.len() as c_int,
            timeout);

        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(rc as int)
        }
    }
}

impl fmt::Debug for Error {
    /// Return the error string for an error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let s = zmq_sys::zmq_strerror(*self as c_int);
            write!(f, "{}",
                   str::from_utf8(ffi::CStr::from_ptr(s).to_bytes()).unwrap())
        }
    }
}

macro_rules! getsockopt_num(
    ($name:ident, $c_ty:ty, $ty:ty) => (
        fn $name(sock: *mut libc::c_void, opt: c_int) -> Result<$ty, Error> {
            unsafe {
                let mut value: $c_ty = 0;
                let value_ptr = &mut value as *mut $c_ty;
                let mut size = mem::size_of::<$c_ty>() as size_t;

                let rc = zmq_sys::zmq_getsockopt(
                    sock,
                    opt,
                    value_ptr as *mut c_void,
                    &mut size);

                if rc == -1 {
                    Err(errno_to_error())
                } else {
                    Ok(value as $ty)
                }
            }
        }
    )
);

getsockopt_num!(getsockopt_int, c_int, int);
getsockopt_num!(getsockopt_i64, int64_t, i64);
getsockopt_num!(getsockopt_u64, uint64_t, u64);

fn getsockopt_bytes(sock: *mut libc::c_void, opt: c_int) -> Result<Vec<u8>, Error> {
    unsafe {
        // The only binary option in zeromq is ZMQ_IDENTITY, which can have
        // a max size of 255 bytes.
        let mut size = 255 as size_t;
        let mut value = Vec::with_capacity(size as uint);

        let r = zmq_sys::zmq_getsockopt(
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
        fn $name(sock: *mut libc::c_void, opt: c_int, value: $ty) -> Result<(), Error> {
            unsafe {
                let size = mem::size_of::<$ty>() as size_t;

                let rc = zmq_sys::zmq_setsockopt(
                    sock,
                    opt,
                    (&value as *const $ty) as *const c_void,
                    size);

                if rc == -1 {
                    Err(errno_to_error())
                } else {
                    Ok(())
                }
            }
        }
    )
);

setsockopt_num!(setsockopt_int, int);
setsockopt_num!(setsockopt_i64, i64);
setsockopt_num!(setsockopt_u64, u64);

fn setsockopt_bytes(sock: *mut libc::c_void, opt: c_int, value: &[u8]) -> Result<(), Error> {
    unsafe {
        let r = zmq_sys::zmq_setsockopt(
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
    Error::from_raw(unsafe { zmq_sys::zmq_errno() })
}
