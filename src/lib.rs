//! Module: zmq

#![allow(trivial_numeric_casts)]

use bitflags::bitflags;
use libc::{c_int, c_long, c_short};

use std::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_void;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd as UnixRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, RawSocket};
use std::ptr;
use std::result;
use std::string::FromUtf8Error;
use std::sync::Arc;

use zmq_sys::{errno, RawFd};

macro_rules! zmq_try {
    ($($tt:tt)*) => {{
        let rc = $($tt)*;
        if rc == -1 {
            return Err(crate::errno_to_error());
        }
        rc
    }}
}

mod message;
mod sockopt;

use crate::message::msg_ptr;
pub use crate::message::Message;
pub use crate::SocketType::*;

/// `zmq`-specific Result type.
pub type Result<T> = result::Result<T, Error>;

/// Socket types
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketType {
    PAIR,
    PUB,
    SUB,
    REQ,
    REP,
    DEALER,
    ROUTER,
    PULL,
    PUSH,
    XPUB,
    XSUB,
    STREAM,
}

impl SocketType {
    fn to_raw(self) -> c_int {
        let raw = match self {
            PAIR => zmq_sys::ZMQ_PAIR,
            PUB => zmq_sys::ZMQ_PUB,
            SUB => zmq_sys::ZMQ_SUB,
            REQ => zmq_sys::ZMQ_REQ,
            REP => zmq_sys::ZMQ_REP,
            DEALER => zmq_sys::ZMQ_DEALER,
            ROUTER => zmq_sys::ZMQ_ROUTER,
            PULL => zmq_sys::ZMQ_PULL,
            PUSH => zmq_sys::ZMQ_PUSH,
            XPUB => zmq_sys::ZMQ_XPUB,
            XSUB => zmq_sys::ZMQ_XSUB,
            STREAM => zmq_sys::ZMQ_STREAM,
        };
        raw as c_int
    }
    fn from_raw(raw: c_int) -> SocketType {
        match raw as u32 {
            zmq_sys::ZMQ_PAIR => PAIR,
            zmq_sys::ZMQ_PUB => PUB,
            zmq_sys::ZMQ_SUB => SUB,
            zmq_sys::ZMQ_REQ => REQ,
            zmq_sys::ZMQ_REP => REP,
            zmq_sys::ZMQ_DEALER => DEALER,
            zmq_sys::ZMQ_ROUTER => ROUTER,
            zmq_sys::ZMQ_PULL => PULL,
            zmq_sys::ZMQ_PUSH => PUSH,
            zmq_sys::ZMQ_XPUB => XPUB,
            zmq_sys::ZMQ_XSUB => XSUB,
            zmq_sys::ZMQ_STREAM => STREAM,
            _ => panic!("socket type is out of range!"),
        }
    }
}

/// Socket Events
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketEvent {
    // TODO: This should become a proper enum, including the data.
    CONNECTED = zmq_sys::ZMQ_EVENT_CONNECTED as isize,
    CONNECT_DELAYED = zmq_sys::ZMQ_EVENT_CONNECT_DELAYED as isize,
    CONNECT_RETRIED = zmq_sys::ZMQ_EVENT_CONNECT_RETRIED as isize,
    LISTENING = zmq_sys::ZMQ_EVENT_LISTENING as isize,
    BIND_FAILED = zmq_sys::ZMQ_EVENT_BIND_FAILED as isize,
    ACCEPTED = zmq_sys::ZMQ_EVENT_ACCEPTED as isize,
    ACCEPT_FAILED = zmq_sys::ZMQ_EVENT_ACCEPT_FAILED as isize,
    CLOSED = zmq_sys::ZMQ_EVENT_CLOSED as isize,
    CLOSE_FAILED = zmq_sys::ZMQ_EVENT_CLOSE_FAILED as isize,
    DISCONNECTED = zmq_sys::ZMQ_EVENT_DISCONNECTED as isize,
    MONITOR_STOPPED = zmq_sys::ZMQ_EVENT_MONITOR_STOPPED as isize,
    HANDSHAKE_FAILED_NO_DETAIL = zmq_sys::ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL as isize,
    HANDSHAKE_SUCCEEDED = zmq_sys::ZMQ_EVENT_HANDSHAKE_SUCCEEDED as isize,
    HANDSHAKE_FAILED_PROTOCOL = zmq_sys::ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL as isize,
    HANDSHAKE_FAILED_AUTH = zmq_sys::ZMQ_EVENT_HANDSHAKE_FAILED_AUTH as isize,
    ALL = zmq_sys::ZMQ_EVENT_ALL as isize,
}

impl SocketEvent {
    pub fn to_raw(self) -> u16 {
        self as u16
    }

    // TODO: this should not need to be public
    pub fn from_raw(raw: u16) -> SocketEvent {
        use SocketEvent::*;
        match u32::from(raw) {
            zmq_sys::ZMQ_EVENT_CONNECTED => CONNECTED,
            zmq_sys::ZMQ_EVENT_CONNECT_DELAYED => CONNECT_DELAYED,
            zmq_sys::ZMQ_EVENT_CONNECT_RETRIED => CONNECT_RETRIED,
            zmq_sys::ZMQ_EVENT_LISTENING => LISTENING,
            zmq_sys::ZMQ_EVENT_BIND_FAILED => BIND_FAILED,
            zmq_sys::ZMQ_EVENT_ACCEPTED => ACCEPTED,
            zmq_sys::ZMQ_EVENT_ACCEPT_FAILED => ACCEPT_FAILED,
            zmq_sys::ZMQ_EVENT_CLOSED => CLOSED,
            zmq_sys::ZMQ_EVENT_CLOSE_FAILED => CLOSE_FAILED,
            zmq_sys::ZMQ_EVENT_DISCONNECTED => DISCONNECTED,
            zmq_sys::ZMQ_EVENT_MONITOR_STOPPED => MONITOR_STOPPED,
            zmq_sys::ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL => HANDSHAKE_FAILED_NO_DETAIL,
            zmq_sys::ZMQ_EVENT_HANDSHAKE_SUCCEEDED => HANDSHAKE_SUCCEEDED,
            zmq_sys::ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL => HANDSHAKE_FAILED_PROTOCOL,
            zmq_sys::ZMQ_EVENT_HANDSHAKE_FAILED_AUTH => HANDSHAKE_FAILED_AUTH,
            zmq_sys::ZMQ_EVENT_ALL => ALL,
            x => panic!("unknown event type {}", x),
        }
    }
}

/// Flag for socket `send` methods that specifies non-blocking mode.
pub const DONTWAIT: i32 = zmq_sys::ZMQ_DONTWAIT as i32;
/// Flag for socket `send` methods that specifies that more frames of a
/// multipart message will follow.
pub const SNDMORE: i32 = zmq_sys::ZMQ_SNDMORE as i32;

/// Security Mechanism
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mechanism {
    // TODO: Fix the naming
    ZMQ_NULL,
    ZMQ_PLAIN,
    ZMQ_CURVE,
    ZMQ_GSSAPI,
}

/// An error returned by a 0MQ API function.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Error {
    EACCES,
    EADDRINUSE,
    EAGAIN,
    EBUSY,
    ECONNREFUSED,
    EFAULT,
    EINTR,
    EHOSTUNREACH,
    EINPROGRESS,
    EINVAL,
    EMFILE,
    EMSGSIZE,
    ENAMETOOLONG,
    ENODEV,
    ENOENT,
    ENOMEM,
    ENOTCONN,
    ENOTSOCK,
    EPROTO,
    EPROTONOSUPPORT,
    ENOTSUP,
    ENOBUFS,
    ENETDOWN,
    EADDRNOTAVAIL,

    // native zmq error codes
    EFSM,
    ENOCOMPATPROTO,
    ETERM,
    EMTHREAD,
}

impl Error {
    pub fn to_raw(self) -> i32 {
        match self {
            Error::EACCES => errno::EACCES,
            Error::EADDRINUSE => errno::EADDRINUSE,
            Error::EAGAIN => errno::EAGAIN,
            Error::EBUSY => errno::EBUSY,
            Error::ECONNREFUSED => errno::ECONNREFUSED,
            Error::EFAULT => errno::EFAULT,
            Error::EINTR => errno::EINTR,
            Error::EHOSTUNREACH => errno::EHOSTUNREACH,
            Error::EINPROGRESS => errno::EINPROGRESS,
            Error::EINVAL => errno::EINVAL,
            Error::EMFILE => errno::EMFILE,
            Error::EMSGSIZE => errno::EMSGSIZE,
            Error::ENAMETOOLONG => errno::ENAMETOOLONG,
            Error::ENODEV => errno::ENODEV,
            Error::ENOENT => errno::ENOENT,
            Error::ENOMEM => errno::ENOMEM,
            Error::ENOTCONN => errno::ENOTCONN,
            Error::ENOTSOCK => errno::ENOTSOCK,
            Error::EPROTO => errno::EPROTO,
            Error::EPROTONOSUPPORT => errno::EPROTONOSUPPORT,
            Error::ENOTSUP => errno::ENOTSUP,
            Error::ENOBUFS => errno::ENOBUFS,
            Error::ENETDOWN => errno::ENETDOWN,
            Error::EADDRNOTAVAIL => errno::EADDRNOTAVAIL,

            Error::EFSM => errno::EFSM,
            Error::ENOCOMPATPROTO => errno::ENOCOMPATPROTO,
            Error::ETERM => errno::ETERM,
            Error::EMTHREAD => errno::EMTHREAD,
        }
    }

    pub fn from_raw(raw: i32) -> Error {
        match raw {
            errno::EACCES => Error::EACCES,
            errno::EADDRINUSE => Error::EADDRINUSE,
            errno::EAGAIN => Error::EAGAIN,
            errno::EBUSY => Error::EBUSY,
            errno::ECONNREFUSED => Error::ECONNREFUSED,
            errno::EFAULT => Error::EFAULT,
            errno::EHOSTUNREACH => Error::EHOSTUNREACH,
            errno::EINPROGRESS => Error::EINPROGRESS,
            errno::EINVAL => Error::EINVAL,
            errno::EMFILE => Error::EMFILE,
            errno::EMSGSIZE => Error::EMSGSIZE,
            errno::ENAMETOOLONG => Error::ENAMETOOLONG,
            errno::ENODEV => Error::ENODEV,
            errno::ENOENT => Error::ENOENT,
            errno::ENOMEM => Error::ENOMEM,
            errno::ENOTCONN => Error::ENOTCONN,
            errno::ENOTSOCK => Error::ENOTSOCK,
            errno::EPROTO => Error::EPROTO,
            errno::EPROTONOSUPPORT => Error::EPROTONOSUPPORT,
            errno::ENOTSUP => Error::ENOTSUP,
            errno::ENOBUFS => Error::ENOBUFS,
            errno::ENETDOWN => Error::ENETDOWN,
            errno::EADDRNOTAVAIL => Error::EADDRNOTAVAIL,
            errno::EINTR => Error::EINTR,

            // These may turn up on platforms that don't support these
            // errno codes natively (Windows)
            errno::ENOTSUP_ALT => Error::ENOTSUP,
            errno::EPROTONOSUPPORT_ALT => Error::EPROTONOSUPPORT,
            errno::ENOBUFS_ALT => Error::ENOBUFS,
            errno::ENETDOWN_ALT => Error::ENETDOWN,
            errno::EADDRINUSE_ALT => Error::EADDRINUSE,
            errno::EADDRNOTAVAIL_ALT => Error::EADDRNOTAVAIL,
            errno::ECONNREFUSED_ALT => Error::ECONNREFUSED,
            errno::EINPROGRESS_ALT => Error::EINPROGRESS,
            errno::ENOTSOCK_ALT => Error::ENOTSOCK,
            errno::EMSGSIZE_ALT => Error::EMSGSIZE,

            // TODO: these are present in `zmq-sys`, but not handled, as that
            // would break backwards-compatibility for the `Error` enum.

            // errno::EAFNOSUPPORT_ALT => Error::EAFNOSUPPORT,
            // errno::ENETUNREACH_ALT => Error::ENETUNREACH,
            // errno::ECONNABORTED_ALT => Error::ECONNABORTED,
            // errno::ECONNRESET_ALT => Error::ECONNRESET,
            // errno::ENOTCONN_ALT => Error::ENOTCONN,
            // errno::ETIMEDOUT_ALT => Error::ETIMEDOUT,
            // errno::EHOSTUNREACH_ALT => Error::EHOSTUNREACH,
            // errno::ENETRESET_ALT => Error::ENETRESET,

            // 0MQ native error codes
            errno::EFSM => Error::EFSM,
            errno::ENOCOMPATPROTO => Error::ENOCOMPATPROTO,
            errno::ETERM => Error::ETERM,
            errno::EMTHREAD => Error::EMTHREAD,

            x => unsafe {
                let s = zmq_sys::zmq_strerror(x);
                panic!(
                    "unknown error [{}]: {}",
                    x,
                    ffi::CStr::from_ptr(s).to_str().unwrap()
                )
            },
        }
    }

    /// Returns the error message provided by 0MQ.
    pub fn message(self) -> &'static str {
        unsafe {
            let s = zmq_sys::zmq_strerror(self.to_raw());
            ffi::CStr::from_ptr(s).to_str().unwrap()
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: An unquoted string is not a good `Debug` output.
        write!(f, "{}", self.message())
    }
}

impl From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;

        let kind = match error {
            Error::ENOENT => ErrorKind::NotFound,
            Error::EACCES => ErrorKind::PermissionDenied,
            Error::ECONNREFUSED => ErrorKind::ConnectionRefused,
            Error::ENOTCONN => ErrorKind::NotConnected,
            Error::EADDRINUSE => ErrorKind::AddrInUse,
            Error::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
            Error::EAGAIN => ErrorKind::WouldBlock,
            Error::EINVAL => ErrorKind::InvalidInput,
            Error::EINTR => ErrorKind::Interrupted,
            _ => ErrorKind::Other,
        };
        // TODO: With rust 1.14 and up there is an optimization
        // opportunity using `std::io::Error: From<ErrorKind>` when
        // `kind != Other`. We should do that once 1.14 has been
        // stable for a bit.
        std::io::Error::new(kind, error)
    }
}

fn errno_to_error() -> Error {
    Error::from_raw(unsafe { zmq_sys::zmq_errno() })
}

/// Return the current zeromq version, as `(major, minor, patch)`.
pub fn version() -> (i32, i32, i32) {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;

    unsafe {
        zmq_sys::zmq_version(&mut major, &mut minor, &mut patch);
    }

    (major, minor, patch)
}

struct RawContext {
    ctx: *mut c_void,
}

impl RawContext {
    fn term(&self) -> Result<()> {
        zmq_try!(unsafe { zmq_sys::zmq_ctx_term(self.ctx) });
        Ok(())
    }
}

unsafe impl Send for RawContext {}
unsafe impl Sync for RawContext {}

impl Drop for RawContext {
    fn drop(&mut self) {
        let mut e = self.term();
        while e == Err(Error::EINTR) {
            e = self.term();
        }
    }
}

/// Handle for a 0MQ context, used to create sockets.
///
/// It is thread safe, and can be safely cloned and shared. Each clone
/// references the same underlying C context. Internally, an `Arc` is
/// used to implement this in a threadsafe way.
///
/// Also note that this binding deviates from the C API in that each
/// socket created from a context initially owns a clone of that
/// context. This reference is kept to avoid a potential deadlock
/// situation that would otherwise occur:
///
/// Destroying the underlying C context is an operation which
/// blocks waiting for all sockets created from it to be closed
/// first. If one of the sockets belongs to thread issuing the
/// destroy operation, you have established a deadlock.
///
/// You can still deadlock yourself (or intentionally close sockets in
/// other threads, see `zmq_ctx_destroy`(3)) by explicitly calling
/// `Context::destroy`.
///
#[derive(Clone)]
pub struct Context {
    raw: Arc<RawContext>,
}

impl Context {
    /// Create a new reference-counted context handle.
    pub fn new() -> Context {
        Context {
            raw: Arc::new(RawContext {
                ctx: unsafe { zmq_sys::zmq_ctx_new() },
            }),
        }
    }

    /// Get the size of the ØMQ thread pool to handle I/O operations.
    pub fn get_io_threads(&self) -> Result<i32> {
        let rc =
            zmq_try!(unsafe { zmq_sys::zmq_ctx_get(self.raw.ctx, zmq_sys::ZMQ_IO_THREADS as _) });
        Ok(rc as i32)
    }

    /// Set the size of the ØMQ thread pool to handle I/O operations.
    pub fn set_io_threads(&self, value: i32) -> Result<()> {
        zmq_try!(unsafe {
            zmq_sys::zmq_ctx_set(self.raw.ctx, zmq_sys::ZMQ_IO_THREADS as _, value)
        });
        Ok(())
    }

    /// Create a new socket.
    ///
    /// Note that the returned socket keeps a an `Arc` reference to
    /// the context it was created from, and will keep that context
    /// from being dropped while being live.
    pub fn socket(&self, socket_type: SocketType) -> Result<Socket> {
        let sock = unsafe { zmq_sys::zmq_socket(self.raw.ctx, socket_type.to_raw()) };

        if sock.is_null() {
            return Err(errno_to_error());
        }

        Ok(Socket {
            sock,
            context: Some(self.clone()),
            owned: true,
        })
    }

    /// Try to destroy the context. This is different than the destructor; the
    /// destructor will loop when zmq_ctx_term returns EINTR.
    pub fn destroy(&mut self) -> Result<()> {
        self.raw.term()
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

/// A socket, the central object in 0MQ.
pub struct Socket {
    sock: *mut c_void,
    // The `context` field is never accessed, but implicitly does
    // reference counting via the `Drop` trait.
    #[allow(dead_code)]
    context: Option<Context>,
    owned: bool,
}

unsafe impl Send for Socket {}

impl Drop for Socket {
    fn drop(&mut self) {
        if self.owned && unsafe { zmq_sys::zmq_close(self.sock) } == -1 {
            panic!("{}", errno_to_error());
        }
    }
}

#[cfg(unix)]
impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> UnixRawFd {
        self.get_fd().unwrap() as UnixRawFd
    }
}

#[cfg(windows)]
impl AsRawSocket for Socket {
    fn as_raw_socket(&self) -> RawSocket {
        self.get_fd().unwrap() as RawSocket
    }
}

macro_rules! sockopt_getter {
    ( $(#[$meta:meta])*
      pub $getter:ident => $constant_name:ident as $ty:ty
    ) => {
        $(#[$meta])*
        pub fn $getter(&self) -> Result<$ty> {
            <$ty as sockopt::Getter>::get(self.sock, zmq_sys::$constant_name as c_int)
        }
    };
}

macro_rules! sockopt_setter {
    ( $(#[$meta:meta])*
      pub $setter:ident => $constant_name:ident as $ty:ty
    ) => {
        $(#[$meta])*
        pub fn $setter(&self, value: $ty) -> Result<()> {
            <$ty as sockopt::Setter>::set(self.sock, zmq_sys::$constant_name as c_int, value)
        }
    };
}

macro_rules! sockopt_seq {
    ( META { $($meta:meta)* }, ) => ();
    ( META { $($meta:meta)* }, $(#[$item_meta:meta])* (_, $setter:ident) => $constant_name:ident as $ty:ty,
      $($rest:tt)*
    ) => {
        sockopt_setter! {
            $(#[$meta])* $(#[$item_meta])*
            pub $setter => $constant_name as $ty
        }
        sockopt_seq!(META { $($meta)* }, $($rest)*);
    };
    ( META { $($meta:meta)* }, $(#[$item_meta:meta])* ($getter:ident) => $constant_name:ident as $ty:ty,
      $($rest:tt)*
    ) => {
        sockopt_getter! {
            $(#[$meta])* $(#[$item_meta])*
            pub $getter => $constant_name as $ty
        }
        sockopt_seq!(META { $($meta)* }, $($rest)*);
    };
    ( META { $($meta:meta)* }, $(#[$item_meta:meta])* ($getter:ident, $setter:ident) => $constant_name:ident as $ty:ty,
      $($rest:tt)*
    ) => {
        sockopt_getter! {
            $(#[$meta])* $(#[$item_meta])*
            pub $getter => $constant_name as $ty
        }
        sockopt_setter! {
            $(#[$meta])* $(#[$item_meta])*
            pub $setter => $constant_name as $ty
        }
        sockopt_seq!(META { $($meta)* }, $($rest)*);
    };
}

macro_rules! sockopts {
    () => ();
    ( $($rest:tt)* ) => {
        sockopt_seq!(META {}, $($rest)*);
    };
}

/// Sendable over a `Socket`.
///
/// A type can implement this trait there is an especially efficient
/// implementation for sending it as a message over a zmq socket.
///
/// If the type needs to be directly passed to `Socket::send()`, but
/// the overhead of allocating a `Message` instance is not an issue,
/// `Into<Message>` should be implemented instead.
///
pub trait Sendable {
    fn send(self, socket: &Socket, flags: i32) -> Result<()>;
}

impl<T> Sendable for T
where
    T: Into<Message>,
{
    fn send(self, socket: &Socket, flags: i32) -> Result<()> {
        let mut msg = self.into();
        zmq_try!(unsafe { zmq_sys::zmq_msg_send(msg_ptr(&mut msg), socket.sock, flags as c_int) });
        Ok(())
    }
}

impl Socket {
    /// Consume the Socket and return the raw socket pointer.
    ///
    /// Failure to close the raw socket manually or call `from_raw`
    /// will lead to a memory leak. Also note that is function
    /// relinquishes the reference on the context is was created from.
    pub fn into_raw(mut self) -> *mut c_void {
        self.owned = false;
        self.sock
    }

    /// Create a Socket from a raw socket pointer.
    ///
    /// The Socket assumes ownership of the pointer and will close the socket
    /// when it is dropped. The returned socket will not reference any context.
    ///
    /// # Safety
    ///
    /// The socket pointer must be a socket created via the `into_raw`
    /// method. The ownership of the socket is transferred the returned Socket,
    /// so any other pointers to the same socket may only be used until it is
    /// dropped.
    pub unsafe fn from_raw(sock: *mut c_void) -> Socket {
        Socket {
            sock,
            context: None,
            owned: true,
        }
    }

    /// Return the inner pointer to this Socket.
    ///
    /// **WARNING**:
    /// It is your responsibility to make sure that the underlying
    /// memory is not freed too early.
    pub fn as_mut_ptr(&mut self) -> *mut c_void {
        self.sock
    }

    /// Accept connections on a socket.
    pub fn bind(&self, endpoint: &str) -> Result<()> {
        let c_str = ffi::CString::new(endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe { zmq_sys::zmq_bind(self.sock, c_str.as_ptr()) });
        Ok(())
    }

    /// Stop accepting connections on a socket
    pub fn unbind(&self, endpoint: &str) -> Result<()> {
        let c_str = ffi::CString::new(endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe { zmq_sys::zmq_unbind(self.sock, c_str.as_ptr()) });
        Ok(())
    }

    /// Connect a socket.
    pub fn connect(&self, endpoint: &str) -> Result<()> {
        let c_str = ffi::CString::new(endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe { zmq_sys::zmq_connect(self.sock, c_str.as_ptr()) });
        Ok(())
    }

    /// Disconnect a previously connected socket
    pub fn disconnect(&self, endpoint: &str) -> Result<()> {
        let c_str = ffi::CString::new(endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe { zmq_sys::zmq_disconnect(self.sock, c_str.as_ptr()) });
        Ok(())
    }

    /// Configure the socket for monitoring
    pub fn monitor(&self, monitor_endpoint: &str, events: i32) -> Result<()> {
        let c_str = ffi::CString::new(monitor_endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe {
            zmq_sys::zmq_socket_monitor(self.sock, c_str.as_ptr(), events as c_int)
        });
        Ok(())
    }

    /// Send a message.
    ///
    /// Due to the provided `From` implementations, this works for
    /// `&[u8]`, `Vec<u8>` and `&str` `Message` itself.
    pub fn send<T>(&self, data: T, flags: i32) -> Result<()>
    where
        T: Sendable,
    {
        data.send(self, flags)
    }

    /// Send a `Message` message.
    #[deprecated(since = "0.9.0", note = "Use `send` instead")]
    pub fn send_msg(&self, msg: Message, flags: i32) -> Result<()> {
        self.send(msg, flags)
    }

    #[deprecated(since = "0.9.0", note = "Use `send` instead")]
    pub fn send_str(&self, data: &str, flags: i32) -> Result<()> {
        self.send(data, flags)
    }

    pub fn send_multipart<I, T>(&self, iter: I, flags: i32) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<Message>,
    {
        let mut last_part: Option<T> = None;
        for part in iter {
            let maybe_last = last_part.take();
            if let Some(last) = maybe_last {
                self.send(last.into(), flags | SNDMORE)?;
            }
            last_part = Some(part);
        }
        if let Some(last) = last_part {
            self.send(last.into(), flags)
        } else {
            Ok(())
        }
    }

    /// Receive a message into a `Message`. The length passed to zmq_msg_recv
    /// is the length of the buffer.
    pub fn recv(&self, msg: &mut Message, flags: i32) -> Result<()> {
        zmq_try!(unsafe { zmq_sys::zmq_msg_recv(msg_ptr(msg), self.sock, flags as c_int) });
        Ok(())
    }

    /// Receive bytes into a slice. The length passed to `zmq_recv` is the length of the slice. The
    /// return value is the number of bytes in the message, which may be larger than the length of
    /// the slice, indicating truncation.
    pub fn recv_into(&self, bytes: &mut [u8], flags: i32) -> Result<usize> {
        let bytes_ptr = bytes.as_mut_ptr() as *mut c_void;
        let rc = zmq_try!(unsafe {
            zmq_sys::zmq_recv(self.sock, bytes_ptr, bytes.len(), flags as c_int)
        });
        Ok(rc as usize)
    }

    /// Receive a message into a fresh `Message`.
    pub fn recv_msg(&self, flags: i32) -> Result<Message> {
        let mut msg = Message::new();
        self.recv(&mut msg, flags).map(|_| msg)
    }

    /// Receive a message as a byte vector.
    pub fn recv_bytes(&self, flags: i32) -> Result<Vec<u8>> {
        self.recv_msg(flags).map(|msg| msg.to_vec())
    }

    /// Receive a `String` from the socket.
    ///
    /// If the received message is not valid UTF-8, it is returned as the original
    /// Vec in the `Err` part of the inner result.
    pub fn recv_string(&self, flags: i32) -> Result<result::Result<String, Vec<u8>>> {
        self.recv_bytes(flags)
            .map(|bytes| String::from_utf8(bytes).map_err(FromUtf8Error::into_bytes))
    }

    /// Receive a multipart message from the socket.
    ///
    /// Note that this will allocate a new vector for each message part; for
    /// many applications it will be possible to process the different parts
    /// sequentially and reuse allocations that way.
    pub fn recv_multipart(&self, flags: i32) -> Result<Vec<Vec<u8>>> {
        let mut parts: Vec<Vec<u8>> = vec![];
        loop {
            let part = self.recv_bytes(flags)?;
            parts.push(part);

            let more_parts = self.get_rcvmore()?;
            if !more_parts {
                break;
            }
        }
        Ok(parts)
    }

    sockopts! {
        /// Accessor for the `ZMQ_IPV6` option.
        (is_ipv6, set_ipv6) => ZMQ_IPV6 as bool,
        /// Accessor for the `ZMQ_IMMEDIATE` option.
        (is_immediate, set_immediate) => ZMQ_IMMEDIATE as bool,
        /// Accessor for the `ZMQ_PLAIN_SERVER` option.
        (is_plain_server, set_plain_server) => ZMQ_PLAIN_SERVER as bool,
        /// Accessor for the `ZMQ_CONFLATE` option.
        (is_conflate, set_conflate) => ZMQ_CONFLATE as bool,
        (is_probe_router, set_probe_router) => ZMQ_PROBE_ROUTER as bool,
        (is_router_mandatory, set_router_mandatory) => ZMQ_ROUTER_MANDATORY as bool,
        (is_router_handover, set_router_handover) => ZMQ_ROUTER_HANDOVER as bool,
        (is_curve_server, set_curve_server) => ZMQ_CURVE_SERVER as bool,
        (is_gssapi_server, set_gssapi_server) => ZMQ_GSSAPI_SERVER as bool,
        (is_gssapi_plaintext, set_gssapi_plaintext) => ZMQ_GSSAPI_PLAINTEXT as bool,
        (_, set_req_relaxed) => ZMQ_REQ_RELAXED as bool,
        (_, set_req_correlate) => ZMQ_REQ_CORRELATE as bool,
    }

    /// Return the type of this socket.
    pub fn get_socket_type(&self) -> Result<SocketType> {
        sockopt::get(self.sock, zmq_sys::ZMQ_TYPE as c_int).map(SocketType::from_raw)
    }

    /// Return true if there are more frames of a multipart message to receive.
    pub fn get_rcvmore(&self) -> Result<bool> {
        sockopt::get(self.sock, zmq_sys::ZMQ_RCVMORE as c_int).map(|o: i64| o == 1i64)
    }

    sockopts! {
        (get_maxmsgsize, set_maxmsgsize) => ZMQ_MAXMSGSIZE as i64,
        (get_sndhwm, set_sndhwm) => ZMQ_SNDHWM as i32,
        (get_rcvhwm, set_rcvhwm) => ZMQ_RCVHWM as i32,
        (get_affinity, set_affinity) => ZMQ_AFFINITY as u64,
        (get_rate, set_rate) => ZMQ_RATE as i32,
        (get_recovery_ivl, set_recovery_ivl) => ZMQ_RECOVERY_IVL as i32,
        (get_sndbuf, set_sndbuf) => ZMQ_SNDBUF as i32,
        (get_rcvbuf, set_rcvbuf) => ZMQ_RCVBUF as i32,
        (get_tos, set_tos) => ZMQ_TOS as i32,
        (get_linger, set_linger) => ZMQ_LINGER as i32,
        (get_reconnect_ivl, set_reconnect_ivl) => ZMQ_RECONNECT_IVL as i32,
        (get_reconnect_ivl_max, set_reconnect_ivl_max) => ZMQ_RECONNECT_IVL_MAX as i32,
        (get_backlog, set_backlog) => ZMQ_BACKLOG as i32,

        /// Get the event notification file descriptor.
        ///
        /// Getter for the `ZMQ_FD` option. Note that the returned
        /// type is platform-specific; it aliases either
        /// `std::os::unix::io::RawFd` and or
        /// `std::os::windows::io::RawSocket`.
        ///
        /// Note that the returned file desciptor has special
        /// semantics: it should only used with an operating system
        /// facility like Unix `poll()` to check its readability.
        (get_fd) => ZMQ_FD as RawFd,

        /// Get the currently pending events.
        ///
        /// Note that the result of this function can also change due
        /// to receiving or sending a message on the socket, without
        /// the signalling FD (see `Socket::get_fd()`).
        ///
        /// # Examples
        ///
        /// ```
        /// use zmq;
        /// let ctx = zmq::Context::new();
        /// let socket = ctx.socket(zmq::REQ).unwrap();
        /// let events = socket.get_events().unwrap();
        /// if events.contains(zmq::POLLIN) {
        ///   println!("socket readable")
        /// }
        /// drop(socket);
        /// ```
        ///
        /// # Compatibility
        ///
        /// This function currently returns the bitmask as an `i32`
        /// for backwards compatibility; in effect it should have been
        /// using the same type as `PollItem::get_revents()` all
        /// along.
        ///
        /// In the `0.9` series, this will be rectified.
        (get_events) => ZMQ_EVENTS as PollEvents,

        (get_multicast_hops, set_multicast_hops) => ZMQ_MULTICAST_HOPS as i32,
        (get_rcvtimeo, set_rcvtimeo) => ZMQ_RCVTIMEO as i32,
        (get_sndtimeo, set_sndtimeo) => ZMQ_SNDTIMEO as i32,
        (get_tcp_keepalive, set_tcp_keepalive) => ZMQ_TCP_KEEPALIVE as i32,
        (get_tcp_keepalive_cnt, set_tcp_keepalive_cnt) => ZMQ_TCP_KEEPALIVE_CNT as i32,
        (get_tcp_keepalive_idle, set_tcp_keepalive_idle) => ZMQ_TCP_KEEPALIVE_IDLE as i32,
        (get_tcp_keepalive_intvl, set_tcp_keepalive_intvl) => ZMQ_TCP_KEEPALIVE_INTVL as i32,
        (get_handshake_ivl, set_handshake_ivl) => ZMQ_HANDSHAKE_IVL as i32,
        // TODO: deprecate to align with ZMQ's preferred naming
        (_, set_identity) => ZMQ_ROUTING_ID as &[u8],
        (_, set_subscribe) => ZMQ_SUBSCRIBE as &[u8],
        (_, set_unsubscribe) => ZMQ_UNSUBSCRIBE as &[u8],
        (get_heartbeat_ivl, set_heartbeat_ivl) => ZMQ_HEARTBEAT_IVL as i32,
        (get_heartbeat_ttl, set_heartbeat_ttl) => ZMQ_HEARTBEAT_TTL as i32,
        (get_heartbeat_timeout, set_heartbeat_timeout) => ZMQ_HEARTBEAT_TIMEOUT as i32,
        (get_connect_timeout, set_connect_timeout) => ZMQ_CONNECT_TIMEOUT as i32,
    }

    // TODO: deprecate to align with ZMQ's preferred naming
    pub fn get_identity(&self) -> Result<Vec<u8>> {
        // 255 = identity max length
        sockopt::get_bytes(self.sock, zmq_sys::ZMQ_ROUTING_ID as c_int, 255)
    }

    pub fn get_socks_proxy(&self) -> Result<result::Result<String, Vec<u8>>> {
        // 255 = longest allowable domain name is 253 so this should
        // be a reasonable size.
        sockopt::get_string(self.sock, zmq_sys::ZMQ_SOCKS_PROXY as c_int, 255, true)
    }

    pub fn get_mechanism(&self) -> Result<Mechanism> {
        sockopt::get(self.sock, zmq_sys::ZMQ_MECHANISM as c_int).map(|mech| match mech {
            zmq_sys::ZMQ_NULL => Mechanism::ZMQ_NULL,
            zmq_sys::ZMQ_PLAIN => Mechanism::ZMQ_PLAIN,
            zmq_sys::ZMQ_CURVE => Mechanism::ZMQ_CURVE,
            zmq_sys::ZMQ_GSSAPI => Mechanism::ZMQ_GSSAPI,
            _ => panic!("Mechanism is out of range!"),
        })
    }

    pub fn get_plain_username(&self) -> Result<result::Result<String, Vec<u8>>> {
        // 255 = arbitrary size
        sockopt::get_string(self.sock, zmq_sys::ZMQ_PLAIN_USERNAME as c_int, 255, true)
    }

    pub fn get_plain_password(&self) -> Result<result::Result<String, Vec<u8>>> {
        // 256 = arbitrary size based on std crypto key size
        sockopt::get_string(self.sock, zmq_sys::ZMQ_PLAIN_PASSWORD as c_int, 256, true)
    }

    pub fn get_zap_domain(&self) -> Result<result::Result<String, Vec<u8>>> {
        // 255 = arbitrary size
        sockopt::get_string(self.sock, zmq_sys::ZMQ_ZAP_DOMAIN as c_int, 255, true)
    }

    /// Return the address of the last endpoint this socket was bound to.
    ///
    /// Note that the returned address is not guaranteed to be the
    /// same as the one used with `bind`, and might also not be
    /// directly usable with `connect`. In particular, when `bind` is
    /// used with the wildcard address (`"*"`), in the address
    /// returned, the wildcard will be expanded into the any address
    /// (i.e. `0.0.0.0` with IPv4).
    pub fn get_last_endpoint(&self) -> Result<result::Result<String, Vec<u8>>> {
        // 256 + 9 + 1 = maximum inproc name size (= 256) + "inproc://".len() (= 9), plus null byte
        sockopt::get_string(
            self.sock,
            zmq_sys::ZMQ_LAST_ENDPOINT as c_int,
            256 + 9 + 1,
            true,
        )
    }

    /// Set the `ZMQ_CURVE_PUBLICKEY` option value.
    ///
    /// The key is returned as raw bytes. Use `z85_encode` on the
    /// resulting data to get the Z85-encoded string representation of
    /// the key.
    pub fn get_curve_publickey(&self) -> Result<Vec<u8>> {
        sockopt::get_bytes(self.sock, zmq_sys::ZMQ_CURVE_PUBLICKEY as c_int, 32)
    }

    /// Get the `ZMQ_CURVE_SECRETKEY` option value.
    ///
    /// The key is returned as raw bytes. Use `z85_encode` on the
    /// resulting data to get the Z85-encoded string representation of
    /// the key.
    pub fn get_curve_secretkey(&self) -> Result<Vec<u8>> {
        sockopt::get_bytes(self.sock, zmq_sys::ZMQ_CURVE_SECRETKEY as c_int, 32)
    }

    /// Get `ZMQ_CURVE_SERVERKEY` option value.
    ///
    /// Note that the key is returned as raw bytes, as a 32-byte
    /// vector. Use `z85_encode()` explicitly to obtain the
    /// Z85-encoded string variant.
    pub fn get_curve_serverkey(&self) -> Result<Vec<u8>> {
        // 41 = Z85 encoded keysize + 1 for null byte
        sockopt::get_bytes(self.sock, zmq_sys::ZMQ_CURVE_SERVERKEY as c_int, 32)
    }

    pub fn get_gssapi_principal(&self) -> Result<result::Result<String, Vec<u8>>> {
        // 260 = best guess of max length based on docs.
        sockopt::get_string(self.sock, zmq_sys::ZMQ_GSSAPI_PRINCIPAL as c_int, 260, true)
    }

    pub fn get_gssapi_service_principal(&self) -> Result<result::Result<String, Vec<u8>>> {
        // 260 = best guess of max length based on docs.
        sockopt::get_string(
            self.sock,
            zmq_sys::ZMQ_GSSAPI_SERVICE_PRINCIPAL as c_int,
            260,
            true,
        )
    }

    sockopts! {
        (_, set_socks_proxy) => ZMQ_SOCKS_PROXY as Option<&str>,
        (_, set_plain_username) => ZMQ_PLAIN_USERNAME as Option<&str>,
        (_, set_plain_password) => ZMQ_PLAIN_PASSWORD as Option<&str>,
        (_, set_zap_domain) => ZMQ_ZAP_DOMAIN as &str,
        (_, set_xpub_welcome_msg) => ZMQ_XPUB_WELCOME_MSG as Option<&str>,
        (_, set_xpub_verbose) => ZMQ_XPUB_VERBOSE as bool,

        (_, set_curve_publickey) => ZMQ_CURVE_PUBLICKEY as &[u8],
        (_, set_curve_secretkey) => ZMQ_CURVE_SECRETKEY as &[u8],
        (_, set_curve_serverkey) => ZMQ_CURVE_SERVERKEY as &[u8],
        (_, set_gssapi_principal) => ZMQ_GSSAPI_PRINCIPAL as &str,
        (_, set_gssapi_service_principal) => ZMQ_GSSAPI_SERVICE_PRINCIPAL as &str,
    }

    /// Create a `PollItem` from the socket.
    pub fn as_poll_item(&self, events: PollEvents) -> PollItem {
        PollItem {
            socket: self.sock,
            fd: 0,
            events: events.bits(),
            revents: 0,
            marker: PhantomData,
        }
    }

    /// Do a call to `zmq_poll` with only this socket.
    ///
    /// The return value on success will be either zero (no event) or one (some
    /// event was signaled).
    pub fn poll(&self, events: PollEvents, timeout_ms: i64) -> Result<i32> {
        poll(&mut [self.as_poll_item(events)], timeout_ms)
    }
}

// TODO: Duplicating the values inside the bitflags struct and on the top level
// is unfortunate.
bitflags! {
    /// Type representing pending socket events.
    pub struct PollEvents: i16 {
        /// For `poll()`, specifies to signal when a message/some data
        /// can be read from a socket.
        const POLLIN = zmq_sys::ZMQ_POLLIN as i16;
        /// For `poll()`, specifies to signal when a message/some data
        /// can be written to a socket.
        const POLLOUT = zmq_sys::ZMQ_POLLOUT as i16;
        /// For `poll()`, specifies to signal when an error condition
        /// is present on a socket.  This only applies to non-0MQ
        /// sockets.
        const POLLERR = zmq_sys::ZMQ_POLLERR as i16;
    }
}

/// For `poll()`, specifies to signal when a message/some data can be
/// read from a socket.
pub const POLLIN: PollEvents = PollEvents::POLLIN;

/// For `poll()`, specifies to signal when a message/some data can be
/// written to a socket.
pub const POLLOUT: PollEvents = PollEvents::POLLOUT;

/// For `poll()`, specifies to signal when an error condition is
/// present on a socket.  This only applies to non-0MQ sockets.
pub const POLLERR: PollEvents = PollEvents::POLLERR;

/// Represents a handle that can be `poll()`ed.
///
/// This is either a reference to a 0MQ socket, or a standard socket.
/// Apart from that it contains the requested event mask, and is updated
/// with the occurred events after `poll()` finishes.
#[repr(C)]
pub struct PollItem<'a> {
    socket: *mut c_void,
    fd: RawFd,
    events: c_short,
    revents: c_short,
    marker: PhantomData<&'a Socket>,
}

impl<'a> PollItem<'a> {
    /// Construct a PollItem from a non-0MQ socket, given by its file
    /// descriptor and the events that should be polled.
    pub fn from_fd(fd: RawFd, events: PollEvents) -> PollItem<'a> {
        PollItem {
            socket: ptr::null_mut(),
            fd,
            events: events.bits(),
            revents: 0,
            marker: PhantomData,
        }
    }

    /// Change the events to wait for.
    pub fn set_events(&mut self, events: PollEvents) {
        self.events = events.bits();
    }

    /// Retrieve the events that occurred for this handle.
    pub fn get_revents(&self) -> PollEvents {
        PollEvents::from_bits_truncate(self.revents)
    }

    /// Returns true if the polled socket has messages ready to receive.
    pub fn is_readable(&self) -> bool {
        (self.revents & POLLIN.bits()) != 0
    }

    /// Returns true if the polled socket can accept messages to be sent
    /// without blocking.
    pub fn is_writable(&self) -> bool {
        (self.revents & POLLOUT.bits()) != 0
    }

    /// Returns true if the polled socket encountered an error condition.
    pub fn is_error(&self) -> bool {
        (self.revents & POLLERR.bits()) != 0
    }

    /// Returns true if the polled socket is the given 0MQ socket.
    pub fn has_socket(&self, socket: &Socket) -> bool {
        self.socket == socket.sock
    }

    /// Returns true if the polled socket is the given file descriptor.
    pub fn has_fd(&self, fd: RawFd) -> bool {
        self.socket.is_null() && self.fd == fd
    }
}

/// Poll for events on multiple sockets.
///
/// For every poll item given, the events given in the `events` bitmask are
/// monitored, and signaled in `revents` when they occur. Any number of poll
/// items can have events signaled when the function returns.
///
/// The given timeout is in milliseconds and can be zero. A timeout of `-1`
/// indicates to block indefinitely until an event has occurred.
///
/// The result, if not `Err`, indicates the number of poll items that have
/// events signaled.
pub fn poll(items: &mut [PollItem], timeout: i64) -> Result<i32> {
    let rc = zmq_try!(unsafe {
        zmq_sys::zmq_poll(
            items.as_mut_ptr() as *mut zmq_sys::zmq_pollitem_t,
            items.len() as c_int,
            timeout as c_long,
        )
    });
    Ok(rc as i32)
}

/// Start a 0MQ proxy in the current thread.
///
/// A proxy connects a frontend socket with a backend socket, where the exact
/// behavior depends on the type of both sockets.
///
/// This function only returns (always with an `Err`) when the sockets' context
/// has been closed.
pub fn proxy(frontend: &Socket, backend: &Socket) -> Result<()> {
    zmq_try!(unsafe { zmq_sys::zmq_proxy(frontend.sock, backend.sock, ptr::null_mut()) });
    Ok(())
}

/// Start a 0MQ proxy in the current thread, with a capture socket.
///
/// The capture socket is sent all messages received on the frontend and backend
/// sockets.
pub fn proxy_with_capture(
    frontend: &mut Socket,
    backend: &mut Socket,
    capture: &mut Socket,
) -> Result<()> {
    zmq_try!(unsafe { zmq_sys::zmq_proxy(frontend.sock, backend.sock, capture.sock) });
    Ok(())
}

/// Start a 0MQ proxy in the current thread, with a control socket.
///
/// If PAUSE is received on the control socket, the proxy suspends its activities. If RESUME is received,
/// it goes on. If TERMINATE is received, it terminates smoothly. At start, the proxy runs normally
/// as if `proxy` was used.
pub fn proxy_steerable(
    frontend: &mut Socket,
    backend: &mut Socket,
    control: &mut Socket,
) -> Result<()> {
    zmq_try!(unsafe {
        zmq_sys::zmq_proxy_steerable(frontend.sock, backend.sock, ptr::null_mut(), control.sock)
    });
    Ok(())
}

/// Start a 0MQ proxy in the current thread, with capture and control sockets.
///
/// Provides a steerable proxy with a capture socket. See `proxy_with_capture`
pub fn proxy_steerable_with_capture(
    frontend: &mut Socket,
    backend: &mut Socket,
    capture: &mut Socket,
    control: &mut Socket,
) -> Result<()> {
    zmq_try!(unsafe {
        zmq_sys::zmq_proxy_steerable(frontend.sock, backend.sock, capture.sock, control.sock)
    });
    Ok(())
}

/// Return true if the used 0MQ library has the given capability.
///
/// The return value is always the `Some` variant; it used to return
/// `None` for older, now unsupported versions of 0MQ that didn't have
/// the wrapped `zmq_has` function. Thus, for code that requires `zmq`
/// version 0.9.0 or newer, you can safely call `unwrap` on the return
/// value.
///
/// For a list of capabilities, please consult the `zmq_has` manual
/// page.
///
/// # Compatibility
///
/// In the `zmq` 0.10.0, this function will simply return `bool`.
///
pub fn has(capability: &str) -> Option<bool> {
    let c_str = ffi::CString::new(capability).unwrap();
    unsafe { Some(zmq_sys::zmq_has(c_str.as_ptr()) == 1) }
}

/// A CURVE key pair generated by 0MQ.
///
/// Note that for API consistency reasons, since version 0.9, the key
/// pair is represented in the binary form. This is in contrast to
/// libzmq, which returns the z85-encoded representation.
#[derive(Debug)]
pub struct CurveKeyPair {
    pub public_key: [u8; 32],
    pub secret_key: [u8; 32],
}

impl CurveKeyPair {
    /// Create a new key pair.
    pub fn new() -> Result<CurveKeyPair> {
        // Curve keypairs are currently 40 bytes long, plus terminating NULL.
        let mut ffi_public_key = [0u8; 41];
        let mut ffi_secret_key = [0u8; 41];

        zmq_try!(unsafe {
            zmq_sys::zmq_curve_keypair(
                ffi_public_key.as_mut_ptr() as *mut libc::c_char,
                ffi_secret_key.as_mut_ptr() as *mut libc::c_char,
            )
        });

        let mut pair = CurveKeyPair {
            public_key: [0; 32],
            secret_key: [0; 32],
        };
        unsafe {
            // No need to check return code here, as zmq_curve_keypair
            // is supposed to generate valid z85-encoded keys
            zmq_sys::zmq_z85_decode(
                pair.public_key.as_mut_ptr(),
                ffi_public_key.as_ptr() as *mut libc::c_char,
            );
            zmq_sys::zmq_z85_decode(
                pair.secret_key.as_mut_ptr(),
                ffi_secret_key.as_ptr() as *mut libc::c_char,
            );
        }

        Ok(pair)
    }
}

/// Errors that can occur while encoding Z85.
#[derive(Debug)]
pub enum EncodeError {
    BadLength,
    FromUtf8Error(FromUtf8Error),
}

impl From<FromUtf8Error> for EncodeError {
    fn from(err: FromUtf8Error) -> Self {
        EncodeError::FromUtf8Error(err)
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::BadLength => write!(f, "Invalid data length. Should be multiple of 4."),
            EncodeError::FromUtf8Error(ref e) => write!(f, "UTF8 conversion error: {}", e),
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadLength => None,
            Self::FromUtf8Error(err) => Some(err),
        }
    }
}

/// Encode a binary key as Z85 printable text.
///
/// Z85 is an encoding similar to Base64, but operates on 4-byte chunks,
/// which are encoded into 5-byte sequences.
///
/// The input slice *must* have a length divisible by 4.
pub fn z85_encode(data: &[u8]) -> result::Result<String, EncodeError> {
    if data.len() % 4 != 0 {
        return Err(EncodeError::BadLength);
    }

    let len = data.len() / 4 * 5 + 1;
    let mut dest = vec![0u8; len];

    unsafe {
        zmq_sys::zmq_z85_encode(
            dest.as_mut_ptr() as *mut libc::c_char,
            data.as_ptr(),
            data.len(),
        );
    }

    dest.truncate(len - 1);
    String::from_utf8(dest).map_err(EncodeError::FromUtf8Error)
}

/// Errors that can occur while decoding Z85.
#[derive(Debug)]
pub enum DecodeError {
    /// The input string slice's length was not a multiple of 5.
    BadLength,
    /// The input string slice had embedded NUL bytes.
    NulError(ffi::NulError),
}

impl From<ffi::NulError> for DecodeError {
    fn from(err: ffi::NulError) -> Self {
        DecodeError::NulError(err)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::BadLength => write!(f, "Invalid data length. Should be multiple of 5."),
            DecodeError::NulError(ref e) => write!(f, "Nul byte error: {}", e),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadLength => None,
            Self::NulError(err) => Some(err),
        }
    }
}

/// Decode a binary key from Z85-encoded text.
///
/// The input string must have a length divisible by 5.
///
/// Note that 0MQ silently accepts characters outside the range defined for
/// the Z85 encoding.
pub fn z85_decode(data: &str) -> result::Result<Vec<u8>, DecodeError> {
    if data.len() % 5 != 0 {
        return Err(DecodeError::BadLength);
    }

    let len = data.len() / 5 * 4;
    let mut dest = vec![0u8; len];

    let c_str = ffi::CString::new(data)?;

    unsafe {
        zmq_sys::zmq_z85_decode(dest.as_mut_ptr(), c_str.into_raw());
    }

    Ok(dest)
}
